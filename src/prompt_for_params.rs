use crate::parse_arg;
use crate::read_param;

struct OscillatorParams {
    amp: f32,
    freq: f32,
    shape: String,
}

impl OscillatorParams {
    fn new(&self, amp: f32, freq: f32, shape: String) -> OscillatorParams {
            OscillatorParams::amp = amp,
            OscillatorParams::freq = freq,
            OscillatorParams::shape = shape,
        }
    }

pub fn prompt_for_params() -> (OscillatorParams, OscillatorParams) {
    // let osc1 = OscillatorParams {
    //     amp: read_param("Enter amplitude for oscillator 1: "),
    //     freq: read_param("Enter frequency for oscillator 1: "),
    //     shape: read_param("Enter shape for oscillator 1 (sin, squ, saw, tri): "),
    // };
    if std::env::args().len() == 4 {
        let amp = parse_arg::<f32>(&std::env::args().nth(1).unwrap(), "oscillator 1 amplitude"),
        let freq = parse_arg::<f32>(&std::env::args().nth(2).unwrap(), "oscillator 1 frequency"),
        let shape = parse_arg::<String>(&std::env::args().nth(3).unwrap(), "oscillator 1 shape"), 
        let osc_instance = OscillatorParams::new(); {
        }
    } else {
        print!("ERROR: Invalid number of arguments. Please enter 3 arguments: amplitude, frequency, shape.")
        // let osc_instance = OscillatorParams::new() {
        //     let mut args = std::env::args().skip(1); // Skip the program name argument
        // (
        //     if let Some(value) = args.next() {
        //         parse_arg::<f32>(&value, "oscillator 1 amplitude")
        //     } else {
        //         read_param::<f32>("Enter amplitude for oscillator 1: ")
        //     },
        //     if let Some(value) = args.next() {
        //         parse_arg::<f32>(&value, "oscillator 1 frequency")
        //     } else {
        //         read_param::<f32>("Enter frequency for oscillator 1: ")
        //     },
        //     if let Some(value) = args.next() {
        //         parse_arg::<String>(&value, "oscillator 1 shape")
        //     } else {
        //         read_param::<String>("Enter shape for oscillator 1 (sin, squ, saw, tri): ")
        }
    };

    // (osc1_amp, osc1_freq, osc1_shape, osc2_amp, osc2_freq, osc2_shape)
};