use std::io;

fn main() {
    println!("What is your name?");
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).unwrap();

    println!("Hello, {}, nice to meet you!", buffer.trim());
}
