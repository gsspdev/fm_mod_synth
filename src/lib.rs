// cpal is a cross-platform audio library for Rust.
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::str::FromStr;
use std::time::{Duration, Instant};
// use std::io::stdout;
use std::io::prelude::*;
// use std::fs::File;

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

pub fn run() {
    let (osc1_amp, osc1_freq, osc2_amp, osc2_freq) = if std::env::args().len() == 5 {
        (
            parse_arg::<f32>(&std::env::args().nth(1).unwrap(), "oscillator 1 amplitude"),
            parse_arg::<f32>(&std::env::args().nth(2).unwrap(), "oscillator 1 frequency"),
            parse_arg::<f32>(&std::env::args().nth(3).unwrap(), "oscillator 2 amplitude"),
            parse_arg::<f32>(&std::env::args().nth(4).unwrap(), "oscillator 2 frequency"),
        )
    } else {
        (
            read_param::<f32>("Enter amplitude for oscillator 1: "),
            read_param::<f32>("Enter frequency for oscillator 1: "),
            read_param::<f32>("Enter amplitude for oscillator 2: "),
            read_param::<f32>("Enter frequency for oscillator 2: "),
        )
    };

    let osc1 = Oscillator::new(osc1_amp, osc1_freq);
    let osc2 = Oscillator::with_input(osc2_amp, osc2_freq, osc1);

    let update_interval = Duration::from_millis(1);
    let mut time = 0.0;

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Failed to find default output device.");
    let config = device
        .default_output_config()
        .expect("Failed to get default output config")
        .config();

    let osc2_clone = osc2.clone();
    let sample_rate = config.sample_rate.0 as f32;
    let mut samples_played = 0f32;

    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    let time = samples_played / sample_rate;
                    *sample = osc2_clone.frequency_modulation(time);
                    samples_played += 1.0;
                }
            },
            err_fn,
        )
        .expect("Failed to build output stream.");

    stream.play().expect("Failed to start audio stream.");

    println!("Press Enter to stop the audio.");
    let _ = std::io::stdin().read_line(&mut String::new());

    loop {
        let start = Instant::now();
        let output = osc2.frequency_modulation(time);
        println!("Output at time {}: {}", time, output);

        let elapsed = start.elapsed();
        if elapsed < update_interval {
            std::thread::sleep(update_interval - elapsed);
        }

        time += update_interval.as_secs_f32();
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
