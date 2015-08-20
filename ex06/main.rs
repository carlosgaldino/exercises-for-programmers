extern crate time;

use std::io;
use std::process;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    println!("What is your current age?");
    stdin.read_line(&mut buffer).unwrap();
    let age = match buffer.lines().nth(0).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("At what age would you like to retire?");
    stdin.read_line(&mut buffer).unwrap();
    let retirement_age = match buffer.lines().nth(1).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    let diff         = retirement_age - age;
    let current_year = time::now_utc().tm_year + 1900;

    if diff < 0 {
        println!("Congratulations! You can retire already.");
    } else {
        println!("You have {} years left until you can retire.", diff);
        println!("It's {}, so you can retire in {}.", current_year, current_year + diff);
    }
}
