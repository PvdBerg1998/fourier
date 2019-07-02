use num_complex::Complex64;
use quadrature::integrate;

pub type Real = f64;
pub type Complex = Complex64;

/// Calculates the `n`th Fourier coefficient for the function `f(t): ℝ → ℂ, t ∈ (0,1)`.
pub fn calculate_fourier_coefficient(f: impl Fn(Real) -> Complex, n: isize) -> Complex {
    // Let f(t) = a(t) + i b(t) = r(t) exp(i φ(t))
    //  1                           1
    // ∫ f(t) exp(-2 π i n t) dt = ∫ r(t) exp[i(φ(t) - 2 π n t)] dt
    // 0                           0
    //
    // Define λ(t) = φ(t) - 2 π n t. Then:
    //
    //  1                       1                       1
    // ∫ r(t) exp(i λ(t)) dt = ∫ r(t) cos(λ(t)) dt + i ∫ r(t) sin(λ(t)) dt
    // 0                       0                       0

    // λ(t) = φ(t) - 2 π n t
    let lambda = |t: Real, phi: Real| phi - 2.0 * std::f64::consts::PI * n as Real * t;

    // ∫ r(t) cos(λ(t)) dt
    let real_integral = |t: Real| {
        let (r, phi) = f(t).to_polar();
        r * lambda(t, phi).cos()
    };

    // ∫ r(t) sin(λ(t))
    let imaginary_integral = |t: Real| {
        let (r, phi) = f(t).to_polar();
        r * lambda(t, phi).sin()
    };

    const T_START: Real = 0.0;
    const T_END: Real = 1.0;
    const TARGET_ERR: Real = 1e-12;

    let real_part = integrate(real_integral, T_START, T_END, TARGET_ERR).integral;
    let imaginary_part = integrate(imaginary_integral, T_START, T_END, TARGET_ERR).integral;
    Complex::new(real_part, imaginary_part)
}

pub fn step(t: Real) -> Complex {
    if t < 0.25 {
        Complex::new(t, 0.0)
    } else if t < 0.5 {
        Complex::new(t, 1.0)
    } else if t < 0.75 {
        Complex::new(t, -1.0)
    } else {
        Complex::new(t, 0.0)
    }
}

pub fn superposition(coefficients: &[(isize, Complex)], t: Real) -> Complex {
    let mut sum = Complex::new(0.0, 0.0);
    for (n, coefficient) in coefficients {
        sum +=
            coefficient * Complex::from_polar(&1.0, &(2.0 * std::f64::consts::PI * *n as Real * t));
    }
    sum
}
