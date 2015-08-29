use std::io;
use std::process;

fn main() {
    let mut stdin  = io::stdin();
    let mut buffer = String::new();

    println!("How many euros are you exchanging?");
    stdin.read_line(&mut buffer).unwrap();
    println!("What is the exchange rate?");
    stdin.read_line(&mut buffer).unwrap();

    let amount = match buffer.lines().nth(0).unwrap().parse::<f64>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    let rate = match buffer.lines().nth(1).unwrap().parse::<f64>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("{:.2} euros at an exchange rate of {:.2} is\n{:.2} USD", amount, rate, amount * rate);
}
