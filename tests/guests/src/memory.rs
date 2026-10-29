fn main() {
    let mut chunks = Vec::new();
    for _ in 0..64 {
        chunks.push(vec![1u8; 1024 * 1024]);
    }
    println!("allocated {}", chunks.len());
}
