use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use regex::Regex;

const ROOT_TEMPLATE_DIRECTORY: &str = "./data/templates";

/// Get a template file based on the name. The name may contain a directory path.
fn get_template_file(template_name: String) -> Result<File, String> {
    if template_name.is_empty() {
        return Err("Template name cannot be empty".to_string());
    }

    if template_name.contains("..") {
        return Err("Template name cannot contain '..'".to_string());
    }

    let template_path = format!("{}/{}.meel", ROOT_TEMPLATE_DIRECTORY, template_name);

    match File::open(template_path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("Template {} not found", template_name))
    }
}

fn get_plain_text_file(template_name: String) -> Result<File, String> {
    let template_path = format!("{}/{}.txt", ROOT_TEMPLATE_DIRECTORY, template_name);

    match File::open(template_path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("Template {} not found", template_name))
    }
}

/// Recursively apply the layout to the template until the root layout is reached.
fn apply_layout(path: String, contents: String) -> Result<String, String> {
    let root_template_path = Path::new(ROOT_TEMPLATE_DIRECTORY);

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

    // TODO: The indenting isn't correct for nested slots.
    let result = re.replace_all(&layout_contents, &contents).to_string();

    if root_template_path.eq(template_parent_path) {
        Ok(result)
    } else {
        apply_layout(template_parent_path.display().to_string(), result)
    }
}

fn apply_placeholders(mut contents: String, data: HashMap<String, String>) -> Result<String, String> {
    let re = match Regex::new(r"\{\{\s*(.*?)\s*}}") {
        Ok(re) => re,
        Err(_) => return Err("Failed to compile regex".to_string())
    };

    let variables: Vec<String> = re.find_iter(&contents).map(|m| m.as_str().to_string()).collect();

    for capture in variables {
        let key = &capture[2..capture.len() - 2].trim().to_string();
        if let Some(value) = data.get(key) {
            contents = contents.replace(&capture, value);
        }
    }

    Ok(contents)
}

/// Render a template with the given data.
pub fn render(template_name: String, data: HashMap<String, String>) -> Result<String, String> {
    let mut file = get_template_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string())
    };

    apply_placeholders(
        apply_layout(format!("{}/{}", ROOT_TEMPLATE_DIRECTORY, &template_name), contents)?,
        data,
    )
}

pub fn render_plain_text(template_name: String, data: HashMap<String, String>) -> Result<String, String> {
    let mut file = get_plain_text_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string())
    };

    apply_placeholders(contents, data)
}

pub fn get_template_vars(template_name: String) -> Result<Vec<String>, String> {
    let mut file = get_template_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string())
    };

    let re = match Regex::new(r"\{\{\s*(.*?)\s*}}") {
        Ok(re) => re,
        Err(_) => return Err("Failed to compile regex".to_string())
    };

    Ok(re.find_iter(&contents).map(|m| m.as_str()[2..m.len() - 2].trim().to_string()).collect())
}