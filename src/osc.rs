// use std::clone;

#[derive(Clone)]
pub enum OscillatorShape {
    Sinewave,
    Squarewave,
    Sawwave,
    Trianglewave
}

#[derive(Clone)]
pub struct Oscillator {
    amp: f32,
    freq: f32,
    shape: OscillatorShape, 
    input: Option<Box<Oscillator>>,
}

impl OscillatorShape {
    pub fn compute(&self, freq: f32, time: f32) -> f32 {
        match self {
            OscillatorShape::Sinewave => (2.0 * std::f32::consts::PI * freq * time).sin(),
            OscillatorShape::Squarewave => (2.0 * std::f32::consts::PI * freq * time).sin().signum(),
            OscillatorShape::Sawwave => 2.0 * (freq * time - freq * time.floor()) - 1.0,
            OscillatorShape::Trianglewave => (2.0 * (freq * time - 0.5)).abs() - 1.0,
        }
    }
}

impl Oscillator {
    pub fn new(amp: f32, freq: f32, shape: OscillatorShape) -> Oscillator {
        Oscillator {
            amp,
            freq,
            shape,
            input: None,
        }
    }

    pub fn with_input(amp: f32, freq: f32, shape: OscillatorShape, input: Oscillator) -> Oscillator {
        Oscillator {
            amp,
            freq,
            shape,
            input: Some(Box::new(input)),
        }
    }

    pub fn clone_osc(&self) -> Oscillator {
        Oscillator {
            amp: self.amp,
            freq: self.freq,
            shape: self.shape.clone(),
            input: self.input.clone(),
        }
    }

    pub fn frequency_modulation(&self, time: f32) -> f32 {
        let input_freq = match &self.input {
            Some(input_oscillator) => input_oscillator.frequency_modulation(time) * self.amp,
            None => 0.0,
        };
self.shape.compute(self.freq + input_freq, time)
    }
}
