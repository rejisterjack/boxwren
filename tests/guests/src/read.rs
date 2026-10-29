fn main() {
    match std::fs::read_to_string("hello.txt") {
        Ok(text) => print!("{text}"),
        Err(err) => {
            eprintln!("read failed: {err}");
            std::process::exit(1);
        }
    }
}
