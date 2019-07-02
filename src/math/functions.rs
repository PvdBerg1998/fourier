use super::{Real, Complex};

pub fn step(t: Real) -> Complex {
    if t < 0.25 {
        Complex::new(t, 0.5)
    } else if t < 0.5 {
        Complex::new(t, 1.0)
    } else if t < 0.75 {
        Complex::new(t, 0.0)
    } else {
        Complex::new(t, 0.5)
    }
}

pub fn diagonal(t: Real) -> Complex {
    Complex::new(t, t)
}

pub fn sin(t: Real) -> Complex {
    Complex::new(t, (t * 2.0 * std::f64::consts::PI).sin() / 2.0 + 0.5)
}

pub fn circle(t: Real) -> Complex {
    Complex::new((t * 2.0 * std::f64::consts::PI).cos() / 2.0 + 0.5, (t * 2.0 * std::f64::consts::PI).sin() / 2.0 + 0.5)
}
