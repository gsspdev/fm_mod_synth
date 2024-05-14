mod osc;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fm_mod_synth::{parse_arg, read_param};
use osc::{Oscillator, ShapeMath};
use std::time::{Duration, Instant};

pub fn run() {
    let (osc1_amp, osc1_freq, osc1_shape) = fetch_oscillator_params(1, "1").unwrap();
    let (osc2_amp, osc2_freq, osc2_shape) = fetch_oscillator_params(4, "2").unwrap();

    let osc1_waveshape = parse_shape(&osc1_shape);
    let osc2_waveshape = parse_shape(&osc2_shape);

    let osc1 = Oscillator::new(osc1_amp, osc1_freq, osc1_waveshape);
    let osc2 = Oscillator::with_input(osc2_amp, osc2_freq, osc2_waveshape, osc1);

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

fn fetch_oscillator_params(start_index: usize, osc: &str) -> Result<(f32, f32, String), String> {
    if std::env::args().len() >= start_index + 3 {
        Ok((
            parse_arg::<f32>(
                &std::env::args().nth(start_index).unwrap(),
                &format!("oscillator {} amplitude", osc),
            ),
            parse_arg::<f32>(
                &std::env::args().nth(start_index + 1).unwrap(),
                &format!("oscillator {} frequency", osc),
            ),
            parse_arg::<String>(
                &std::env::args().nth(start_index + 2).unwrap(),
                &format!("oscillator {} shape", osc),
            ),
        ))
    } else {
        Ok((
            read_param::<f32>(&format!("Enter amplitude for oscillator {}: ", osc)),
            read_param::<f32>(&format!("Enter frequency for oscillator {}: ", osc)),
            read_param::<String>(&format!(
                "Enter shape for oscillator {} (sin, squ, saw, tri): ",
                osc
            )),
        ))
    }
}

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

fn main() {
    run();
}
