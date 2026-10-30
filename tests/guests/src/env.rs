fn main() {
    match std::env::var("BOXWREN_SECRET") {
        Ok(value) => println!("LEAKED {value}"),
        Err(_) => println!("absent"),
    }
}
