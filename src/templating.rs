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

    let name = if template_name.ends_with(".meel") {
        template_name
    } else {
        format!("{}.meel", template_name)
    };

    let template_path = format!("{}/{}", ROOT_TEMPLATE_DIRECTORY, name);

    match File::open(template_path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("Template {} not found", name))
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

/// Render a template with the given data.
pub fn render(template_name: String, data: HashMap<String, String>) -> Result<String, String> {
    let mut file = get_template_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string())
    };

    contents = apply_layout(format!("{}/{}", ROOT_TEMPLATE_DIRECTORY, &template_name), contents)?;

    // Loop over the data, and apply it to the template
    for (key, value) in data.into_iter() {
        let re = Regex::new(&format!(r"\{{\{{\s*{}\s*\}}\}}", key)).unwrap();
        contents = re.replace_all(&contents, value).to_string();
    }

    Ok(contents)
}