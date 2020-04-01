//! Approximate solution based on distance fields.

use crate::math::compute_erf7;

/// Generate a blurred rounded rectangle using distance field approximation.
pub fn gen_distfield(width: usize, height: usize, w: f64, h: f64, r: f64, s: f64, r_mul: f64, exponent: f64) -> Vec<u8> {
    // Try to calculate parameters
    let r_orig = r;
    let r = r.hypot(s * 1.25);

    let r = r * r_mul;

    let r = r.min(0.5 * w.min(h));

    let s_inv = s.max(1e-6).recip();
    let recip_exponent = exponent.recip();
    let mut buf = vec![0u8; width * height];
    let min_edge = w.min(h);
    let scale = 0.5 * compute_erf7(s_inv * 0.5 * (w.max(h) - 0.5 * r_orig));
    for j in 0..height {
        let y = (j as f64) + 0.5 - 0.5 * (height as f64);
        let y0 = y.abs() - (h * 0.5 - r);
        let y1 = y0.max(0.0);
        for i in 0..width {
            let x = (i as f64) + 0.5 - 0.5 * (width as f64);
            let x0 = x.abs() - (w * 0.5 - r);
            let x1 = x0.max(0.0);
            let d_pos = (x1.powf(exponent) + y1.powf(exponent)).powf(recip_exponent);
            let d_neg = x0.max(y0).min(0.0);
            let d = d_pos + d_neg - r;
            let z = scale * (compute_erf7(s_inv * (min_edge + d)) - compute_erf7(s_inv * d));
            buf[j * width + i] = (z * 255.0).round() as u8;
        }
    }
    buf
}
