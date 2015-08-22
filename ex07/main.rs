use std::io;
use std::process;

fn main() {
    const CONVERSION_FACTOR: f64 = 0.09290304;
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    println!("What is the length of the room in feet?");
    stdin.read_line(&mut buffer).unwrap();
    let length = match buffer.lines().nth(0).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("What is the width of the room in feet?");
    stdin.read_line(&mut buffer).unwrap();
    let width = match buffer.lines().nth(1).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("You entered dimensions of {} feet by {} feet.", length, width);

    let square_feet   = length as f64 * width as f64;
    let square_meters = square_feet * CONVERSION_FACTOR;
    println!("The area is:\n{} square feet\n{} square meters", square_feet, square_meters);
}
