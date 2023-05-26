// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::io::prelude::*;

// TODO : Seperate out the library into a crate format

// An oscillator struct that can be used to generate a sine wave.
#[derive(Clone)]
pub struct Oscillator {
    amp: f32,
    freq: f32,
    input: Option<Box<Oscillator>>,
}

// Implement the oscillator struct.
impl Oscillator {
    pub fn new(amp: f32, freq: f32) -> Oscillator {
        Oscillator {
            amp,
            freq,
            input: None,
        }
    }

    // a function that takes an oscillator as an input and returns a new oscillator.
    pub fn with_input(amp: f32, freq: f32, input: Oscillator) -> Oscillator {
        Oscillator {
            amp,
            freq,
            input: Some(Box::new(input)),
        }
    }

    // a function for frequency modulation.
    pub fn frequency_modulation(&self, time: f32) -> f32 {
        let input_freq = match &self.input {
            Some(input_oscillator) => input_oscillator.frequency_modulation(time) * self.amp,
            None => 0.0,
        };

        (2.0 * std::f32::consts::PI * (self.freq + input_freq) * time).sin()
    }
}



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
