use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use ammonia::clean_text;
use minify_html::{Cfg, minify};
use regex::Regex;

use crate::utils;

pub fn get_template_directory() -> String {
    format!("{}/templates", utils::env::get_var("MEEL_DATA_DIRECTORY", Some("./data")).unwrap())
}

fn get_globals() -> Result<HashMap<String, String>, String> {
    let globals_path = format!("{}/globals.json", utils::env::get_var("MEEL_DATA_DIRECTORY", Some("./data")).unwrap());

    let mut file = match File::open(globals_path) {
        Ok(file) => file,
        Err(_) => return Err("Failed to open globals file".to_string())
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read globals file".to_string())
    };

    let globals: HashMap<String, String> = match serde_json::from_str(&contents) {
        Ok(globals) => globals,
        Err(_) => return Err("Failed to parse globals file".to_string())
    };

    Ok(globals)
}

/// Get a template file based on the name. The name may contain a directory path.
fn get_template_file(template_name: String) -> Result<File, String> {
    if template_name.is_empty() {
        return Err("Template name cannot be empty".to_string());
    }

    if template_name.contains("..") {
        return Err("Template name cannot contain '..'".to_string());
    }

    let template_path = format!("{}/{}.meel", get_template_directory(), template_name);

    match File::open(template_path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("Template {} not found", template_name))
    }
}

/// Get a plain text template file based on the name. The name may contain a directory path.
fn get_plain_text_file(template_name: String) -> Result<File, String> {
    let template_path = format!("{}/{}.txt", get_template_directory(), template_name);

    match File::open(template_path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("Template {} not found", template_name))
    }
}

/// Recursively apply the layout to the template until the root layout is reached.
fn apply_layout(path: String, contents: String) -> Result<String, String> {
    let template_directory = get_template_directory();
    let root_template_path = Path::new(&template_directory);

    let template_parent_path = match Path::new(&path).parent() {
        Some(parent) => parent,
        None => return Err("Failed to get parent directory".to_string())
    };

    let layout_path = format!("{}/layout.meel", template_parent_path.display());

    let layout_contents = if Path::new(&layout_path).exists() {
        let mut layout_file = match File::open(&layout_path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open layout file".to_string())
        };

        let mut layout_contents = String::new();
        match layout_file.read_to_string(&mut layout_contents) {
            Ok(_) => layout_contents,
            Err(_) => return Err("Failed to read layout file".to_string())
        }
    } else {
        "<slot />".to_string()
    };

    let re = match Regex::new(r"<slot( ?)/>|<slot>(.*?)</slot>") {
        Ok(re) => re,
        Err(_) => return Err("Failed to compile regex".to_string())
    };

    // TODO: The indenting isn't correct for nested slots. We might actually want to compress the content though.
    let result = re.replace_all(&layout_contents, &contents).to_string();

    if root_template_path.eq(template_parent_path) {
        Ok(result)
    } else {
        apply_layout(template_parent_path.display().to_string(), result)
    }
}

fn create_placeholder_regex() -> Result<Regex, String> {
    match Regex::new(r"\{\{\s*(.*?)\s*}}") {
        Ok(re) => Ok(re),
        Err(_) => Err("Failed to compile regex".to_string())
    }
}

/// Apply placeholders to the supplied template contents.
fn apply_placeholders(mut contents: String, data: HashMap<String, String>, allow_html: bool) -> Result<String, String> {
    let re = create_placeholder_regex()?;
    let placeholders: Vec<String> = re.find_iter(&contents).map(|m| m.as_str().to_string()).collect();

    for capture in placeholders {
        let key = &capture[2..capture.len() - 2].trim().to_string();
        if let Some(value) = data.get(key) {
            let replacement = if allow_html { value.clone() } else { clean_text(value) };
            contents = contents.replace(&capture, &replacement);
        }
    }

    Ok(contents)
}

/// Get the placeholders in a template.
pub fn get_template_placeholders(template_name: String) -> Result<Vec<String>, String> {
    let mut file = get_template_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string())
    };

    let contents = apply_layout(format!("{}/{}", get_template_directory(), &template_name), contents)?;

    let re = create_placeholder_regex()?;
    Ok(re.find_iter(&contents).map(|m| m.as_str()[2..m.len() - 2].trim().to_string()).collect())
}

/// Render a template with the given data.
pub fn render(template_name: String, mut data: HashMap<String, String>, allow_html: bool, minify_html: bool) -> Result<String, String> {
    let mut file = get_template_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string())
    };

    let globals = get_globals().unwrap_or_default();
    data.extend(globals);

    let content = apply_placeholders(
        apply_layout(format!("{}/{}", get_template_directory(), &template_name), contents)?,
        data,
        allow_html,
    )?;

    if !minify_html {
        return Ok(content);
    }

    let mut cfg = Cfg::new();
    cfg.keep_closing_tags = true;

    match String::from_utf8(minify(content.as_ref(), &cfg)) {
        Ok(content) => Ok(content),
        // We failed to minify here so return the original content
        Err(_) => Ok(content)
    }
}

pub fn render_plain_text(template_name: String, mut data: HashMap<String, String>) -> Result<String, String> {
    let mut file = get_plain_text_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string())
    };

    let globals = get_globals().unwrap_or_default();
    data.extend(globals);

    apply_placeholders(contents, data, false)
}