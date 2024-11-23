#[derive(Debug, Clone, Copy)]
pub enum ShapeMath {
    Sinewave,
    Squarewave,
    Sawwave,
    Trianglewave,
}

impl ShapeMath {
    pub fn compute(&self, freq: f32, time: f32) -> f32 {
        match self {
            Self::Sinewave => (2.0 * std::f32::consts::PI * freq * time).sin(),
            Self::Squarewave => (2.0 * std::f32::consts::PI * freq * time).sin().signum(),
            Self::Sawwave => 2.0 * (freq * time - freq * time.floor()) - 1.0,
            Self::Trianglewave => (2.0 * (freq * time - 0.5)).abs() - 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Oscillator {
    amp: f32,
    freq: f32,
    shape: ShapeMath,
    input: Option<Box<Oscillator>>,
}

impl Oscillator {
    pub fn new(amp: f32, freq: f32, shape: ShapeMath) -> Self {
        Oscillator {
            amp,
            freq,
            shape,
            input: None,
        }
    }

    pub fn new_with_input(amp: f32, freq: f32, shape: ShapeMath, input: Oscillator) -> Self {
        Oscillator {
            amp,
            freq,
            shape,
            input: Some(Box::new(input)),
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
