use std::io;

fn calculate_rate(x: i32) -> Result<i32, ()> {
    if x > 0 {
        Ok(72 / x)
    } else {
        Err(())
    }
}

fn main() {
    let error_message = "Sorry. That's not a valid input.";

    loop {
        let mut stdin  = io::stdin();
        let mut buffer = String::new();

        println!("What is the rate of return?");
        stdin.read_line(&mut buffer).unwrap();

        match buffer.lines().next().unwrap().parse::<i32>() {
            Ok(x) => match calculate_rate(x) {
                Ok(y) => {
                    println!("It will take {} years to double your initial investment.", y);
                    break;
                }
                Err(_) => {
                    println!("{}", error_message);
                    continue;
                },
            },
            Err(_) => {
                println!("{}", error_message);
                continue;
            },
        }
    }
}
