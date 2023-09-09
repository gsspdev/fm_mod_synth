use std::io::{self, Write};
use std::str::FromStr;

pub fn parse_arg<T: FromStr>(arg: &str, description: &str) -> T {
    match arg.parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("Failed to parse {}.", description),
    }
}

pub fn read_param<T: FromStr>(prompt: &str) -> T {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("Failed to read input."),
    }
}