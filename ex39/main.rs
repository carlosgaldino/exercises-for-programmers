#[derive(Debug, Clone)]
struct Person {
    position:        String,
    last_name:       String,
    first_name:      String,
    separation_date: Option<String>,
}

fn generate_people() -> [Person; 6] {
    [
        Person { first_name: String::from("John"),      last_name: String::from("Johnson"),    position: String::from("Manager"),            separation_date: Some(String::from("2016-12-31")) },
        Person { first_name: String::from("Tou"),       last_name: String::from("Xiong"),      position: String::from("Software Developer"), separation_date: Some(String::from("2016-10-05")) },
        Person { first_name: String::from("Michaela"),  last_name: String::from("Michaelson"), position: String::from("District Manager"),   separation_date: Some(String::from("2015-12-19")) },
        Person { first_name: String::from("Jake"),      last_name: String::from("Jacobson"),   position: String::from("Programmer"),         separation_date: None },
        Person { first_name: String::from("Jacquelyn"), last_name: String::from("Jackson"),    position: String::from("DBA"),                separation_date: None },
        Person { first_name: String::from("Sally"),     last_name: String::from("Weber"),      position: String::from("Web Developer"),      separation_date: Some(String::from("2015-12-18")) },
    ]
}

fn main() {
    let mut people = generate_people();

    people.sort_by(|p, b| p.last_name.cmp(&b.last_name));

    println!("{:<20}| {:<20}| {:<20}", "Name", "Position", "Separation Date");
    println!("{:-<20}|{:-<21}|{:-<20}", "-", "-", "-");

    for p in people.iter() {
        let name = String::from("") + &p.first_name + " " + &p.last_name;
        let pp = p.clone();

        println!("{:<20}| {:<20}| {:<20}", name, p.position, pp.separation_date.unwrap_or(String::new()));
    }
}
