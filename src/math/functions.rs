use super::{Complex, Real};

pub const HALF_SPACE_SIZE: f64 = 1000.0;
pub const FULL_SPACE_SIZE: f64 = HALF_SPACE_SIZE * 2.0;

/*
    Functions f(t) → c
    where t ∈ [0, 1]
    where ℜ(c) ∈ [-HALF_SPACE_SIZE, HALF_SPACE_SIZE], ℑ(c) ∈ [-HALF_SPACE_SIZE, HALF_SPACE_SIZE]
*/

pub fn step(t: Real) -> Complex {
    if t < 0.25 {
        Complex::new(t * FULL_SPACE_SIZE - HALF_SPACE_SIZE, 0.0)
    } else if t < 0.5 {
        Complex::new(t * FULL_SPACE_SIZE - HALF_SPACE_SIZE, HALF_SPACE_SIZE)
    } else if t < 0.75 {
        Complex::new(t * FULL_SPACE_SIZE - HALF_SPACE_SIZE, -HALF_SPACE_SIZE)
    } else {
        Complex::new(t * FULL_SPACE_SIZE - HALF_SPACE_SIZE, 0.0)
    }
}

pub fn tent(t: Real) -> Complex {
    if t < 0.5 {
        Complex::new(
            t * FULL_SPACE_SIZE - HALF_SPACE_SIZE,
            t * FULL_SPACE_SIZE * 2.0 - HALF_SPACE_SIZE,
        )
    } else {
        Complex::new(
            t * FULL_SPACE_SIZE - HALF_SPACE_SIZE,
            HALF_SPACE_SIZE - (t - 0.5) * FULL_SPACE_SIZE * 2.0,
        )
    }
}

// @todo shift these functions properly

/*
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
*/
