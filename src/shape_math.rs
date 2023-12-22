pub mod shape_math {
    // use std::clone;
    #[derive(Clone)]
    pub enum ShapeMath {
        Sinewave,
        Squarewave,
        Sawwave,
        Trianglewave
    }

    impl ShapeMath {
        pub fn compute(&self, freq: f32, time: f32) -> f32 {
            match self {
                ShapeMath::Sinewave => (2.0 * std::f32::consts::PI * freq * time).sin(),
                ShapeMath::Squarewave => (2.0 * std::f32::consts::PI * freq * time).sin().signum(),
                ShapeMath::Sawwave => 2.0 * (freq * time - freq * time.floor()) - 1.0,
                ShapeMath::Trianglewave => (2.0 * (freq * time - 0.5)).abs() - 1.0,
            }
        }
    }
}