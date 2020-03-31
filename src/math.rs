//! Various math functions.
//!
//! In general, these are designed to be approximations good enough for image
//! usage (error on the order of 1e-3), and suitable for SIMD and GPU acceleration.

// see https://people.maths.ox.ac.uk/gilesm/files/gems_erfinv.pdf
// Note: we just do the center because it's precise enough
pub fn approx_inv_erf(x: f64) -> f64 {
    let w = -((1.0 - x) * (1.0 + x)).ln() - 2.5;
    let mut p = 2.81022636e-08;
    p = 3.43273939e-07 + p * w;
    p = -3.5233877e-06 + p * w;
    p = -4.39150654e-06 + p * w;
    p = 0.00021858087 + p * w;
    p = -0.00125372503 + p * w;
    p = -0.00417768164 + p * w;
    p = 0.246640727 + p * w;
    p = 1.50140941 + p * w;
    p * x
}

// See https://raphlinus.github.io/audio/2018/09/05/sigmoid.html for a little
// explanation of this approximation to the erf function.
pub fn compute_erf7(x: f64) -> f64 {
    let x = x * std::f64::consts::FRAC_2_SQRT_PI;
    let xx = x * x;
    let x = x + (0.24295 + (0.03395 + 0.0104 * xx) * xx) * (x * xx);
    x / (1.0 + x * x).sqrt()
}

// Solve rounded rect for x given y. Assume center at origin.
pub fn solve_rr(w: f64, h: f64, r: f64, y: f64) -> f64 {
    let yy = y.abs() - 0.5 * h;
    if yy >= 0.0 {
        0.0
    } else if yy <= -r {
        0.5 * w
    } else {
        0.5 * w + (r * r - (yy + r).powi(2)).sqrt() - r
    }
}
