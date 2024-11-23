// use std::clone;

use crate::osc::ShapeMath::{Sawwave, Sinewave, Squarewave, Trianglewave};

#[derive(Debug, Clone, Copy)]
pub enum ShapeMath {
    Sinewave,
    Squarewave,
    Sawwave,
    Trianglewave,
}

pub struct Graph {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
}

impl Graph {
    pub fn new(x: Vec<f32>, y: Vec<f32>) -> Self {
        Graph { x, y }
    }
}

pub fn sinewave_gen(freq: f32, time: f32) -> Graph {
    let mut x = Vec::new();
    let mut y = Vec::new();
    for i in 0..(time * 44100.0) as usize {
        x.push(i as f32 / 44100.0);
        y.push((2.0 * std::f32::consts::PI * freq * x[i]).sin());
    }
    Graph::new(x, y)
}

#[derive(Debug, Clone)]
pub struct Oscillator {
    amp: f32,
    freq: f32,
    shape: ShapeMath,
    input: Option<Box<Oscillator>>,
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

impl Oscillator {
    pub fn new(amp: f32, freq: f32, shape: ShapeMath) -> Self {
        Oscillator {
            amp,
            freq,
            shape,
            input: None,
        }
    }

    pub fn with_input(amp: f32, freq: f32, shape: ShapeMath, input: Oscillator) -> Self {
        Oscillator {
            amp,
            freq,
            shape,
            input: Some(Box::new(input)),
        }
    }

    //    pub fn clone_osc(&self) -> Self {
    //        Oscillator {
    //            amp: self.amp,
    //            freq: self.freq,
    //            shape: self.shape.clone(),
    //            input: self.input.clone(),
    //        }
    //    }

    pub fn frequency_modulation(&self, time: f32) -> f32 {
        let input_freq = match &self.input {
            Some(input_oscillator) => input_oscillator.frequency_modulation(time) * self.amp,
            None => 0.0,
        };
        self.shape.compute(self.freq + input_freq, time)
    }
}
