use std::io;

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    println!("Enter a noun: ");
    stdin.read_line(&mut buffer).unwrap();

    println!("Enter a verb: ");
    stdin.read_line(&mut buffer).unwrap();

    println!("Enter an adjective: ");
    stdin.read_line(&mut buffer).unwrap();

    println!("Enter an adverb: ");
    stdin.read_line(&mut buffer).unwrap();

    let noun      = buffer.lines().nth(0).unwrap();
    let verb      = buffer.lines().nth(1).unwrap();
    let adjective = buffer.lines().nth(2).unwrap();
    let adverb    = buffer.lines().nth(3).unwrap();

    println!("Do you {} your {} {} {}? That's hilarious!", verb, adjective, noun, adverb);
}
