use std::io;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    println!("What is the first number?");
    stdin.read_line(&mut buffer).unwrap();

    println!("What is the second number?");
    stdin.read_line(&mut buffer).unwrap();

    let first  = buffer.lines().nth(0).unwrap().parse::<i64>().unwrap();
    let second = buffer.lines().nth(1).unwrap().parse::<i64>().unwrap();

    println!("{} + {} = {}", first, second, first + second);
    println!("{} - {} = {}", first, second, first - second);
    println!("{} * {} = {}", first, second, first * second);
    println!("{} / {} = {}", first, second, first / second);
}
