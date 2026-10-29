fn main() {
    match std::fs::read_to_string("../secret.txt") {
        Ok(text) => println!("ESCAPED {text}"),
        Err(err) => println!("blocked {err}"),
    }
}
