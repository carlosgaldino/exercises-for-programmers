use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name:  String,
    salary:     i64,
}

fn parse(line: &str) -> Person {
    let fields: Vec<&str> = line.split(',').collect();

    Person { first_name: fields[1].to_string(), last_name: fields[0].to_string(), salary: fields[2].parse().unwrap() }
}

fn main() {
    let mut fread = File::open("data.csv").unwrap();
    let mut buffer = String::new();

    fread.read_to_string(&mut buffer);

    let mut people: Vec<Person> = buffer.lines().map(parse).collect();

    people.sort_by(|p, o| o.salary.cmp(&p.salary));

    println!("{:<12} {:<12} {:<8}", "Last", "First", "Salary");
    println!("{:-<12}{:-<12}{:-<8}", "-", "-", "-");

    for p in people {
        println!("{:<12} {:<12} {:<6}", p.last_name, p.first_name, p.salary);
    }
}
