#[derive(Clone)]
pub struct Oscillator {
    amp: f32,
    freq: f32,
    input: Option<Box<Oscillator>>,
}

impl Oscillator {

    pub fn new(amp: f32, freq: f32) -> Oscillator {
        Oscillator {
            amp,
            freq,
            input: None,
        }
    }

    pub fn with_input(amp: f32, freq: f32, input: Oscillator) -> Oscillator {
        Oscillator {
            amp,
            freq,
            input: Some(Box::new(input)),
        }
    }

    pub fn frequency_modulation(&self, time: f32) -> f32 {
        let input_freq = match &self.input {
            Some(input_oscillator) => input_oscillator.frequency_modulation(time) * self.amp,
            None => 0.0,
        };

        (2.0 * std::f32::consts::PI * (self.freq + input_freq) * time).sin()
    }
}