mod templating;

fn main() {
    println!("{}", templating::render_template("greeting".to_string(), None).unwrap());
}
