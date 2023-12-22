
use crate::parse_arg;
use crate::read_param;
// use lib::{parse_arg, read_param};




pub fn prompt_for_params() -> (f32, f32, String, f32, f32, String) {
    let (osc1_amp, 
        osc1_freq,
        osc1_shape, 
        osc2_amp, 
        osc2_freq,
        osc2_shape) = if std::env::args().len() == 7 {
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

    (osc1_amp, osc1_freq, osc1_shape, osc2_amp, osc2_freq, osc2_shape)
}