use std::io;
use std::process;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    println!("How many people?");
    stdin.read_line(&mut buffer).unwrap();
    let people = match buffer.lines().nth(0).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("How many pizzas you have?");
    stdin.read_line(&mut buffer).unwrap();
    let pizzas = match buffer.lines().nth(1).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("How many slices each pizza has?");
    stdin.read_line(&mut buffer).unwrap();
    let slices = match buffer.lines().nth(2).unwrap().parse::<i32>() {
        Ok(x) => x,
        Err(_) => {
            println!("You must enter a number.");
            process::exit(1);
        }
    };

    println!("{} people with {} pizzas", people, pizzas);

    let number_of_pizzas = pizzas * slices / people;
    println!("Each person gets {} pieces of pizza.", number_of_pizzas);

    let leftovers = pizzas * slices % people;
    println!("There are {} leftover pieces.", leftovers);
}
