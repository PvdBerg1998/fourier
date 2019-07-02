use num_complex::Complex64;
use quadrature::integrate;

pub type Scalar = f64;

/// Calculates the `n`th Fourier coefficient for the function `f(t): ℝ → ℂ, t ∈ (0,1)`.
pub fn calculate_fourier_coefficient(f: impl Fn(Scalar) -> Complex64, n: isize) -> Complex64 {
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
    let lambda = |t: Scalar, phi: Scalar| phi - 2.0 * std::f64::consts::PI * n as Scalar * t;

    // ∫ r(t) cos(λ(t)) dt
    let real_integral = |t: Scalar| {
        let (r, phi) = f(t).to_polar();
        r * lambda(t, phi).cos()
    };

    // ∫ r(t) sin(λ(t))
    let imaginary_integral = |t: Scalar| {
        let (r, phi) = f(t).to_polar();
        r * lambda(t, phi).sin()
    };

    const T_START: Scalar = 0.0;
    const T_END: Scalar = 1.0;
    const TARGET_ERR: Scalar = 1e-12;

    let real_part = integrate(real_integral, T_START, T_END, TARGET_ERR).integral;
    let imaginary_part = integrate(imaginary_integral, T_START, T_END, TARGET_ERR).integral;
    Complex64::new(real_part, imaginary_part)
}

pub fn step(t: Scalar) -> Complex64 {
    if t < 0.25 {
        Complex64::new(t, 0.0)
    } else if t < 0.5 {
        Complex64::new(t, 1.0)
    } else if t < 0.75 {
        Complex64::new(t, -1.0)
    } else {
        Complex64::new(t, 0.0)
    }
}

pub fn superposition(coefficients: &[(isize, Complex64)], t: Scalar) -> Complex64 {
    let mut sum = Complex64::new(0.0, 0.0);
    for (n, coefficient) in coefficients {
        sum += coefficient
            * Complex64::from_polar(&1.0, &(2.0 * std::f64::consts::PI * *n as Scalar * t));
    }
    sum
}
