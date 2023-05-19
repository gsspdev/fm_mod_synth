// cpal is a cross-platform audio library for Rust.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::io::{self, Write};
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::env;

// An oscillator struct that can be used to generate a sine wave.
#[derive(Clone)]
struct Oscillator {
    amp: f32,
    freq: f32,
    // on_off: bool,
    input: Option<Box<Oscillator>>,
}

// Implement the oscillator struct.
impl Oscillator {
    fn new(amp: f32, freq: f32) -> Oscillator {
        Oscillator {
            amp,
            freq,
            // on_off: false,
            input: None,
        }
    }

    // a function that takes an oscillator as an input and returns a new oscillator.
    fn with_input(amp: f32, freq: f32, input: Oscillator) -> Oscillator {
        Oscillator {
            amp,
            freq,
            // on_off: false,
            input: Some(Box::new(input)),
        }
    }

    // a function for frequency modulation.
    fn frequency_modulation(&self, time: f32) -> f32 {
        let input_freq = match &self.input {
            Some(input_oscillator) => input_oscillator.frequency_modulation(time) * self.amp,
            None => 0.0,
        };

        (2.0 * std::f32::consts::PI * (self.freq + input_freq) * time).sin()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (osc1_amp, osc1_freq, osc2_amp, osc2_freq) = if args.len() == 5 {
        (
            parse_arg::<f32>(&args[1], "oscillator 1 amplitude"),
            parse_arg::<f32>(&args[2], "oscillator 1 frequency"),
            parse_arg::<f32>(&args[3], "oscillator 2 amplitude"),
            parse_arg::<f32>(&args[4], "oscillator 2 frequency"),
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

fn parse_arg<T: FromStr>(arg: &str, description: &str) -> T {
    arg.parse::<T>().unwrap_or_else(|_| {
        eprintln!("Invalid {}: {}", description, arg);
        std::process::exit(1);
    })
}

fn read_param<T: FromStr>(prompt: &str) -> T {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => {
                print!("Invalid input. Please try again: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
