mod templating;

fn main() {
    println!("{}", templating::render("greeting".to_string(), None).unwrap());
}
