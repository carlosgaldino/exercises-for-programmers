use std::io;
use std::process;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    println!("What is the length of the room?");
    stdin.read_line(&mut buffer).unwrap();
    let length = match buffer.lines().nth(0).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("What is the width of the room?");
    stdin.read_line(&mut buffer).unwrap();
    let width = match buffer.lines().nth(1).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    let square_feet = length * width;
    println!("You will need to purchase {} gallons of paint to cover {} square feet.", square_feet / 350 + 1, square_feet);
}
