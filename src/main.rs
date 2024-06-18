use std::collections::HashMap;

mod templating;

fn main() {
    let mut map = HashMap::new();
    map.insert("name".to_string(), "John".to_string());

    println!("{}", templating::render("greeting".to_string(), map).unwrap());
}
