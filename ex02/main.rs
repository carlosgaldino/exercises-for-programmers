use std::io;

fn main() {
    println!("What is the input string?");
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).unwrap();

    println!("{} has {} characters", buffer.trim(), buffer.trim().chars().count());
}
