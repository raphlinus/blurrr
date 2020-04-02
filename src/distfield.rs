//! Approximate solution based on distance fields.

use std::f64::consts::{FRAC_1_SQRT_2, LN_2};
use crate::math::compute_erf7;

/// Generate a blurred rounded rectangle using distance field approximation.
pub fn gen_distfield(width: usize, height: usize, w: f64, h: f64, r: f64, s: f64, r_mul: f64, exponent: f64) -> Vec<u8> {
    // Try to calculate parameters
    let r_orig = r;

    let min_edge = w.min(h);
    let rmax = 0.5 * min_edge;
    let r0 = r.hypot(s * 1.15).min(rmax);
    let r1 = r.hypot(s * 2.0).min(rmax);

    //let exponent = (-LN_2) / (1.0 - (1.0 - FRAC_1_SQRT_2) * r0 / r1).ln();
    //let exponent = 6.0 * r1 / r0 - 4.0;
    let exponent = 2.0 * r1 / r0;
    println!("r1/r {}, exponent {}", r1 / r, exponent);
    let r = r1;

    let s_inv = s.max(1e-6).recip();
    let recip_exponent = exponent.recip();
    let mut buf = vec![0u8; width * height];
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
