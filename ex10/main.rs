use std::io;
use std::process;

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

    println!("Enter the price of item 1: ");
    stdin.read_line(&mut buffer).unwrap();
    println!("Enter the quantity of item 1: ");
    stdin.read_line(&mut buffer).unwrap();

    println!("Enter the price of item 2: ");
    stdin.read_line(&mut buffer).unwrap();
    println!("Enter the quantity of item 2: ");
    stdin.read_line(&mut buffer).unwrap();

    println!("Enter the price of item 3: ");
    stdin.read_line(&mut buffer).unwrap();
    println!("Enter the quantity of item 3: ");
    stdin.read_line(&mut buffer).unwrap();

    let items: Vec<i32> = buffer.lines().map(parse_input).collect();
    let price           = items.chunks(2).map(|xs| xs.into_iter().fold(1, |acc, n| acc * n)).fold(0, |acc, n| acc + n);
    let tax             = price as f64 * 0.055;

    println!("Subtotal: ${}", price);
    println!("Tax: ${}", tax);
    println!("Total: ${}", price as f64 + tax);
}
