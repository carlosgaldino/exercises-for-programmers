use std::io;
use std::process;

fn parse_input(x: &str) -> f64 {
    match x.parse::<f64>() {
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

    let items: Vec<f64> = buffer.lines().map(parse_input).collect();
    let price           = items.chunks(2).map(|xs| xs.into_iter().fold(1.0, |acc, n| acc * n)).fold(0.0, |acc, n| acc + n);
    let tax             = price as f64 * 0.055;

    println!("Subtotal: ${:.2}", price);
    println!("Tax: ${:.2}", tax);
    println!("Total: ${:.2}", price as f64 + tax);
}
