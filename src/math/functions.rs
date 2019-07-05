use super::{Complex, Real};

pub const FULL_SPACE: f64 = 2000.0;
pub const HALF_SPACE: f64 = FULL_SPACE / 2.0;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Function {
    Step,
    Tent,
}

impl Function {
    pub fn execute(self, t: Real) -> Complex {
        match self {
            Function::Step => step(t),
            Function::Tent => tent(t),
        }
    }

    pub fn next(self) -> Self {
        match self {
            Function::Step => Function::Tent,
            Function::Tent => Function::Step,
        }
    }
}

/*
    Functions f(t) → c
    where t ∈ [0, 1]
    where ℜ(c) ∈ [-1.0, 1.0], ℑ(c) ∈ [-1.0, 1.0]
*/

pub fn step(t: Real) -> Complex {
    if t < 0.25 {
        Complex::new(t * FULL_SPACE - HALF_SPACE, 0.0)
    } else if t < 0.5 {
        Complex::new(t * FULL_SPACE - HALF_SPACE, HALF_SPACE)
    } else if t < 0.75 {
        Complex::new(t * FULL_SPACE - HALF_SPACE, -HALF_SPACE)
    } else {
        Complex::new(t * FULL_SPACE - HALF_SPACE, 0.0)
    }
}

pub fn tent(t: Real) -> Complex {
    if t < 0.5 {
        Complex::new(
            t * FULL_SPACE - HALF_SPACE,
            t * FULL_SPACE * 2.0 - HALF_SPACE,
        )
    } else {
        Complex::new(
            t * FULL_SPACE - HALF_SPACE,
            HALF_SPACE - (t - 0.5) * FULL_SPACE * 2.0,
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
