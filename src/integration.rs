//! Computation of blurred rounded rectangle blur through integration

use crate::math::{approx_inv_erf, compute_erf7, solve_rr};

const N_INTEGRATE: usize = 256;
const EXTENT: f64 = 3.0;

/// Generate a blurred rounded rectangle.
pub fn gen_integrate(width: usize, height: usize, w: f64, h: f64, r: f64, s: f64) -> Vec<u8> {
    let s_inv = s.max(1e-6).recip();
    let mut buf = vec![0u8; width * height];
    let mut line = vec![0.0f64; width / 2];
    for j in 0..(height / 2) {
        for i in 0..(width / 2) {
            line[i] = 0.0;
        }
        let y = (j as f64) + 0.5;
        for k in 0..N_INTEGRATE {
            let jf = (k + 1) as f64 * (2.0 / ((N_INTEGRATE + 1) as f64)) - 1.0;
            let yf = y + s * approx_inv_erf(jf);
            let xmax = solve_rr(w, h, r, yf);
            if xmax > 0.0 {
                let imax = ((xmax + EXTENT * s).ceil() as usize).min(width / 2);
                for i in 0..imax {
                    let x = (i as f64) + 0.5;
                    line[i] += compute_erf7((xmax - x) * s_inv) + compute_erf7((xmax + x) * s_inv);
                }
            }

        }
        let off0 = (j + height / 2) * width;
        let off1 = (height / 2 - j - 1) * width;
        for i in 0..(width / 2) {
            let g = (255.0 * (0.5 / N_INTEGRATE as f64) * line[i]).round() as u8;
            buf[off0 + width / 2 + i] = g;
            buf[off0 + width / 2 - i - 1] = g;
            buf[off1 + width / 2 + i] = g;
            buf[off1 + width / 2 - i - 1] = g;
        }
    }
    buf
}
