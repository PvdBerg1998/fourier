use super::{Complex, Real};

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

pub fn cube(t: Real) -> Complex {
    if t < 0.25 {
        // Top
        Complex::new((t - 0.0) * 4.0, 1.0)
    } else if t < 0.5 {
        // Right side
        Complex::new(1.0, 1.0 - (t - 0.25) * 4.0)
    } else if t < 0.75 {
        // Bottom
        Complex::new(1.0 - (t - 0.5) * 4.0, 0.0)
    } else {
        // Left side
        Complex::new(0.0, (t - 0.75) * 4.0)
    }
}

pub fn diagonal(t: Real) -> Complex {
    Complex::new(t, t)
}

pub fn sin(t: Real) -> Complex {
    Complex::new(t, (t * 2.0 * std::f64::consts::PI).sin() / 2.0 + 0.5)
}

pub fn circle(t: Real) -> Complex {
    Complex::new(
        (t * 2.0 * std::f64::consts::PI).cos() / 2.0 + 0.5,
        (t * 2.0 * std::f64::consts::PI).sin() / 2.0 + 0.5,
    )
}
