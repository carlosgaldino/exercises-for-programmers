use std::process;
use std::io;

fn parse_input(x: &str) -> i32 {
    match x.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("You must enter only numbers.");
            process::exit(1);
        }
    }
}

fn main() {
    let mut stdin  = io::stdin();
    let mut buffer = String::new();

    println!("Enter a list of numbers, separated by spaces: ");
    stdin.read_line(&mut buffer).unwrap();

    let even_numbers: Vec<i32> = buffer.split_whitespace().map(parse_input).filter(|x| x % 2 == 0).collect();

    println!("The even numbers are {:?}", even_numbers);
}
