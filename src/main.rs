mod lib;
mod osc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::time::{Duration, Instant};
use std::clone;

// use crate::lib::Oscillator;
use crate::osc::Oscillator;
use crate::osc::ShapeMath;
use crate::lib::read_param;
use crate::lib::parse_arg;

pub fn run() {
    let (
        osc1_amp, 
        osc1_freq, 
        osc1_shape, 
        osc2_amp, 
        osc2_freq, 
        osc2_shape) = 

        if std::env::args().len() == 7 {
        (
            parse_arg::<f32>(&std::env::args().nth(1).unwrap(), "oscillator 1 amplitude"),
            parse_arg::<f32>(&std::env::args().nth(2).unwrap(), "oscillator 1 frequency"),
            parse_arg::<String>(&std::env::args().nth(3).unwrap(), "oscillator 1 shape"), 
            parse_arg::<f32>(&std::env::args().nth(4).unwrap(), "oscillator 2 amplitude"),
            parse_arg::<f32>(&std::env::args().nth(5).unwrap(), "oscillator 2 frequency"),
            parse_arg::<String>(&std::env::args().nth(6).unwrap(), "oscillator 2 shape"),
        )
        } else {
        (
            read_param::<f32>("Enter amplitude for oscillator 1: "),
            read_param::<f32>("Enter frequency for oscillator 1: "),
            read_param::<String>("Enter shape for oscillator 1 (sin, squ, saw, tri): "),
            read_param::<f32>("Enter amplitude for oscillator 2: "),
            read_param::<f32>("Enter frequency for oscillator 2: "),
            read_param::<String>("Enter shape for oscillator 2 (sin, squ, saw, tri): "),
        )
    };

    fn parse_shape(shape: &str) -> ShapeMath {
        match shape {
            "sin" => ShapeMath::Sinewave,
            "squ" => ShapeMath::Squarewave,
            "saw" => ShapeMath::Sawwave,
            "tri" => ShapeMath::Trianglewave,
            _ => {
                eprintln!("Invalid shape: {}", shape);
                std::process::exit(1);
            }
        }
    }

    let osc1_waveshape = parse_shape(&osc1_shape);
    let osc2_waveshape = parse_shape(&osc2_shape);

    let osc1 = Oscillator::new(osc1_amp, osc1_freq, osc1_waveshape);
    let osc2 = Oscillator::with_input(osc2_amp, osc2_freq, osc2_waveshape, osc1);

    let update_interval = Duration::from_millis(1);
    let mut time = 0.0;

    // ... rest of the code is same

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Failed to find default output device.");
    let config = device
        .default_output_config()
        .expect("Failed to get default output config")
        .config();

    let osc2_clone = osc2.clone_osc();
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

fn main() {
    run();
}