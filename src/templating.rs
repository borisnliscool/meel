use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use regex::Regex;

fn get_template_directory() -> &'static str {
    "./data/templates"
}

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

    let path = format!("{}/{}", get_template_directory(), name);

    match File::open(path) {
        Ok(file) => Ok(file),
        Err(_) => Err(format!("Template {} not found", name))
    }
}

fn apply_layout(path: String, contents: String) -> Result<String, String> {
    let root = Path::new(get_template_directory());
    let path = Path::new(&path);

    let mut components = path.components();
    components.next_back();

    let layout_path = format!("{}/layout.meel", components.as_path().display());

    let mut layout_file = match File::open(&layout_path) {
        Ok(file) => file,
        Err(_) => return Err(format!("Layout {} not found", layout_path).to_string())
    };

    let mut layout_contents = String::new();
    match layout_file.read_to_string(&mut layout_contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read layout file".to_string())
    };

    let re = match Regex::new(r"<slot( ?)/>|<slot>(.*?)</slot>") {
        Ok(re) => re,
        Err(_) => return Err("Failed to compile regex".to_string())
    };

    let result = re.replace_all(&layout_contents, &contents).to_string();

    if root.eq(components.as_path()) {
        Ok(result)
    } else {
        apply_layout(components.as_path().display().to_string(), result)
    }
}

// TODO: It would be nice to make this "data" parameter optional in the future.
pub fn render_template(template_name: String, data: Option<HashMap<String, String>>) -> Result<String, String> {
    let data = data.unwrap_or_default();
    let mut file = get_template_file(template_name.clone())?;

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Failed to read template file".to_string())
    };

    contents = apply_layout(format!("{}/{}", get_template_directory(), &template_name), contents)?;

    // Loop over the data, and apply it to the template
    for (key, value) in data.into_iter() {
        contents = contents.replace(&format!("{{{}}}", key), &value);
    }

    Ok(contents)
}