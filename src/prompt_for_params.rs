
// crate::lib::O
// use lib::read_param;
// use lib::parse_arg;
// use std::clone::std;

// pub mod prompt_for_params;
pub use lib::{parse_arg, read_param};
// pub use std::clone::std;

// pub mod prompt_for_params;

pub fn prompt_for_params() -> () {

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
            println!("Not enough arguments, prompting for parameters");
            (
                read_param::<f32>("Enter amplitude for oscillator 1: "),
                read_param::<f32>("Enter frequency for oscillator 1: "),
                read_param::<String>("Enter shape for oscillator 1 (sin, squ, saw, tri): "),
                read_param::<f32>("Enter amplitude for oscillator 2: "),
                read_param::<f32>("Enter frequency for oscillator 2: "),
                read_param::<String>("Enter shape for oscillator 2 (sin, squ, saw, tri): "),
            )
        };
        (osc1_amp, osc1_freq, osc1_shape, osc2_amp, osc2_freq, osc2_shape)
    }
}