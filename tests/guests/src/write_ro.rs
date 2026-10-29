fn main() {
    match std::fs::write("note.txt", "pwned") {
        Ok(()) => println!("WROTE"),
        Err(err) => println!("blocked {err}"),
    }
}
