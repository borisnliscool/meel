use crate::utils;
use ammonia::clean_text;
use minify_html::{minify, Cfg};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub type TemplateDataMap = HashMap<String, Value>;

pub fn get_template_directory() -> String {
    format!(
        "{}/templates",
        utils::env::get_var("MEEL_DATA_DIRECTORY", Some("./data")).unwrap()
    )
}

fn get_globals() -> Result<TemplateDataMap, String> {
    let globals_path = format!(
        "{}/globals.json",
        utils::env::get_var("MEEL_DATA_DIRECTORY", Some("./data")).unwrap()
    );

    let mut file = match File::open(globals_path) {
        Ok(file) => file,
        Err(_) => return Err("Failed to open globals file".to_string()),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read globals file".to_string()),
    };

    let globals: TemplateDataMap = match serde_json::from_str(&contents) {
        Ok(globals) => globals,
        Err(_) => return Err("Failed to parse globals file".to_string()),
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
        Err(_) => Err(format!("Template {template_name} not found")),
    }
}

/// Get a plain text template file based on the name. The name may contain a directory path.
fn get_plain_text_file(template_name: String) -> Result<File, String> {
    let template_path = format!("{}/{}.txt", get_template_directory(), template_name);

    match File::open(template_path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("Template {template_name} not found")),
    }
}

/// Recursively apply the layout to the template until the root layout is reached.
fn apply_layout(path: String, contents: String) -> Result<String, String> {
    let template_directory = get_template_directory();
    let root_template_path = Path::new(&template_directory);

    let template_parent_path = match Path::new(&path).parent() {
        Some(parent) => parent,
        None => return Err("Failed to get parent directory".to_string()),
    };

    let layout_path = format!("{}/layout.meel", template_parent_path.display());

    let layout_contents = if Path::new(&layout_path).exists() {
        let mut layout_file = match File::open(&layout_path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open layout file".to_string()),
        };

        let mut layout_contents = String::new();
        match layout_file.read_to_string(&mut layout_contents) {
            Ok(_) => layout_contents,
            Err(_) => return Err("Failed to read layout file".to_string()),
        }
    } else {
        "<slot />".to_string()
    };

    let re = match Regex::new(r"<slot( ?)/>|<slot>(.*?)</slot>") {
        Ok(re) => re,
        Err(_) => return Err("Failed to compile regex".to_string()),
    };

    // TODO: The indenting isn't correct for nested slots. We might actually want to compress the content though.
    let result = re.replace_all(&layout_contents, &contents).to_string();

    if root_template_path.eq(template_parent_path) {
        Ok(result)
    } else {
        apply_layout(template_parent_path.display().to_string(), result)
    }
}

/// Apply placeholders to the supplied template contents.
pub fn apply_placeholders(
    contents: String,
    data: TemplateDataMap,
    allow_html: bool,
) -> Result<String, String> {
    let template =
        mustache::compile_str(&contents).map_err(|_| "Failed to compile template".to_string())?;

    let cleaned_data = if allow_html {
        data
    } else {
        let mut cleaned_data = TemplateDataMap::new();

        fn clean(data: Value) -> Value {
            match data {
                Value::Null => Value::Null,
                Value::Bool(bool) => Value::Bool(bool),
                Value::Number(num) => Value::Number(num),
                Value::String(value) => Value::String(clean_text(&value)),
                Value::Array(array) => {
                    Value::Array(array.iter().map(|arg: &Value| clean(arg.clone())).collect())
                }
                Value::Object(value) => Value::Object(
                    value
                        .iter()
                        .map(|(key, arg)| (key.clone(), clean(arg.clone())))
                        .collect(),
                ),
            }
        }

        for (key, value) in data {
            cleaned_data.insert(key, clean(value));
        }
        cleaned_data
    };

    template
        .render_to_string(&cleaned_data)
        .map_err(|_| "Failed to render template".to_string())
}

#[test]
fn test_apply_placeholders() {
    assert_eq!(
        apply_placeholders(
            "Hello {{ what }}!".to_string(),
            TemplateDataMap::from([("what".to_string(), Value::String("World".to_string()))]),
            false
        )
        .unwrap(),
        "Hello World!"
    );

    assert_eq!(
        apply_placeholders(
            "Hello {{ what }}!".to_string(),
            TemplateDataMap::from([(
                "what".to_string(),
                Value::String("<strong>World</strong>".to_string())
            )]),
            true
        )
        .unwrap(),
        "Hello &lt;strong&gt;World&lt;/strong&gt;!"
    );

    assert_eq!(
        apply_placeholders(
            "{{#things}}{{.}}{{/things}}".to_string(),
            TemplateDataMap::from([(
                "things".to_string(),
                Value::Array(vec![
                    Value::String("One".to_string()),
                    Value::String("Two".to_string())
                ])
            )]),
            false
        )
        .unwrap(),
        "OneTwo"
    );
}

/// Render a template with the given data.
pub fn render(
    template_name: String,
    mut data: TemplateDataMap,
    allow_html: bool,
    minify_html: bool,
) -> Result<String, String> {
    let mut file = get_template_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string()),
    };

    let globals = get_globals().unwrap_or_default();
    data.extend(globals);

    let content = apply_placeholders(
        apply_layout(
            format!("{}/{}", get_template_directory(), &template_name),
            contents,
        )?,
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
        Err(_) => Ok(content),
    }
}

pub fn render_plain_text(
    template_name: String,
    mut data: TemplateDataMap,
) -> Result<String, String> {
    let mut file = get_plain_text_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string()),
    };

    let globals = get_globals().unwrap_or_default();
    data.extend(globals);

    apply_placeholders(contents, data, false)
}
