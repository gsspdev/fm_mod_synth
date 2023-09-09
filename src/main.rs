// mod utils;
// mod oscillator;
// use 'use' in the main.rs file
// 'mod' is for lib.rs and mod.rs files

use fm_mod_synth::utils::{parse_arg, read_param};
use fm_mod_synth::oscillator::{Oscillator, ShapeMath};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::time::{Duration, Instant};
// use std::clone;
use crate::osc::Oscillator;
use crate::osc::OscillatorShape;
use crate::lib::read_param;
use crate::lib::parse_arg;

// use std::clone;

// use crate::lib::Oscillator;
// use crate::osc::Oscillator;
// use crate::osc::OscillatorShape;
// use crate::lib::read_param;
// use crate::lib::parse_arg;
// use crate::prompt_for_params::prompt_for_params;

/// A simple oscillator.
pub fn run() {
    // Arguments are parsed
   let (osc1_amp, osc1_freq, osc1_shape, osc2_amp, osc2_freq, osc2_shape) = if std::env::args().len() == 7 {
    (
        parse_arg::<f32>(&std::env::args().nth(1).unwrap(), "oscillator 1 amplitude"),
        parse_arg::<f32>(&std::env::args().nth(2).unwrap(), "oscillator 1 frequency"),
        parse_arg::<String>(&std::env::args().nth(3).unwrap(), "oscillator 1 shape"),
        parse_arg::<f32>(&std::env::args().nth(4).unwrap(), "oscillator 2 amplitude"),
        parse_arg::<f32>(&std::env::args().nth(5).unwrap(), "oscillator 2 frequency"),
        parse_arg::<String>(&std::env::args().nth(6).unwrap(), "oscillator 2 shape"),
    )
} else {
    let mut args = std::env::args().skip(1); // Skip the program name argument
    (
        if let Some(value) = args.next() {
            parse_arg::<f32>(&value, "oscillator 1 amplitude")
        } else {
            read_param::<f32>("Enter amplitude for oscillator 1: ")
        },
        if let Some(value) = args.next() {
            parse_arg::<f32>(&value, "oscillator 1 frequency")
        } else {
            read_param::<f32>("Enter frequency for oscillator 1: ")
        },
        if let Some(value) = args.next() {
            parse_arg::<String>(&value, "oscillator 1 shape")
        } else {
            read_param::<String>("Enter shape for oscillator 1 (sin, squ, saw, tri): ")
        },
        if let Some(value) = args.next() {
            parse_arg::<f32>(&value, "oscillator 2 amplitude")
        } else {
            read_param::<f32>("Enter amplitude for oscillator 2: ")
        },
        if let Some(value) = args.next() {
            parse_arg::<f32>(&value, "oscillator 2 frequency")
        } else {
            read_param::<f32>("Enter frequency for oscillator 2: ")
        },
        if let Some(value) = args.next() {
            parse_arg::<String>(&value, "oscillator 2 shape")
        } else {
            read_param::<String>("Enter shape for oscillator 2 (sin, squ, saw, tri): ")
        },
    )
};

    // Arguments are parsed
    fn parse_shape(shape: &str) -> OscillatorShape {
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

// fn prompt_for_params() -> (f32, f32, String, f32, f32, String) {
//     let (osc1_amp, 
//         osc1_freq,
//         osc1_shape, 
//         osc2_amp, 
//         osc2_freq,
//         osc2_shape) = if std::env::args().len() == 7 {
//         (
//             parse_arg::<f32>(&std::env::args().nth(1).unwrap(), "oscillator 1 amplitude"),
//             parse_arg::<f32>(&std::env::args().nth(2).unwrap(), "oscillator 1 frequency"),
//             parse_arg::<String>(&std::env::args().nth(3).unwrap(), "oscillator 1 shape"), 
//             parse_arg::<f32>(&std::env::args().nth(4).unwrap(), "oscillator 2 amplitude"),
//             parse_arg::<f32>(&std::env::args().nth(5).unwrap(), "oscillator 2 frequency"),
//             parse_arg::<String>(&std::env::args().nth(6).unwrap(), "oscillator 2 shape"),
//         )
//     } else {
//         (
//             read_param::<f32>("Enter amplitude for oscillator 1: "),
//             read_param::<f32>("Enter frequency for oscillator 1: "),
//             read_param::<String>("Enter shape for oscillator 1 (sin, squ, saw, tri): "),
//             read_param::<f32>("Enter amplitude for oscillator 2: "),
//             read_param::<f32>("Enter frequency for oscillator 2: "),
//             read_param::<String>("Enter shape for oscillator 2 (sin, squ, saw, tri): "),
//         )
//     };
//     (osc1_amp, osc1_freq, osc1_shape, osc2_amp, osc2_freq, osc2_shape)
// }

fn main() {
    run();
}
