use std::io;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    println!("What is the quote?");
    stdin.read_line(&mut buffer).unwrap();

    println!("Who said it?");
    stdin.read_line(&mut buffer).unwrap();

    let output = buffer.lines().last().unwrap().to_string() + " says, \"" + buffer.lines().nth(0).unwrap() + "\"";

    println!("{}", output);
}
