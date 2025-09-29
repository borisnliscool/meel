use std::env;

pub fn get_var(name: &str, fallback: Option<&str>) -> Option<String> {
    let value = env::var(name);

    if value.is_err() {
        return fallback.map(|fallback| fallback.to_string());
    }

    let val = value.unwrap_or(
        fallback.map(|f| f.to_string()).unwrap_or("".to_string())
    );

    if val.is_empty() {
        return fallback.map(|f| f.to_string());
    }

    Some(val)
}