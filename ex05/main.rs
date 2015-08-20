use std::io;
use std::process;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    println!("What is the first number?");
    stdin.read_line(&mut buffer).unwrap();
    let first = match buffer.lines().nth(0).unwrap().parse::<f64>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("What is the second number?");
    stdin.read_line(&mut buffer).unwrap();
    let second = match buffer.lines().nth(1).unwrap().parse::<f64>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("{} + {} = {}", first, second, first + second);
    println!("{} - {} = {}", first, second, first - second);
    println!("{} * {} = {}", first, second, first * second);
    println!("{} / {} = {}", first, second, first / second);
}
