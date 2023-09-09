use std::io::prelude::*;
use std::str::FromStr;

pub fn parse_arg<T: FromStr>(arg: &str, description: &str) -> T {
    arg.parse::<T>().unwrap_or_else(|_| {
        eprintln!("Invalid {}: {}", description, arg);
        std::process::exit(1);
    })
}

pub fn read_param<T: FromStr>(prompt: &str) -> T {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => {
                print!("Invalid input. Please try again: ");
                std::io::stdout().flush().unwrap();
            }
        }
    }
}
