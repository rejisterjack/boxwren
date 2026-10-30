fn main() {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:1".to_string());
    match std::net::TcpStream::connect(&addr) {
        Ok(_) => println!("CONNECTED"),
        Err(err) => println!("blocked {err}"),
    }
}
