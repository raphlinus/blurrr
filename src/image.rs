use druid::piet::{Image, ImageFormat};
use druid::{PaintCtx, RenderContext};

// Make a grayscale image from a single intensity map.
#[allow(unused)]
pub fn make_image_one(ctx: &mut PaintCtx, w: usize, h: usize, data: &[u8]) -> Image {
    let mut buf = vec![255u8; w * h * 4];
    for i in 0..(w * h) {
        let g = data[i];
        buf[i * 4] = g;
        buf[i * 4 + 1] = g;
        buf[i * 4 + 2] = g;
    }
    ctx.make_image(w, h, &buf, ImageFormat::RgbaPremul).unwrap()
}

const N_STEPS: f64 = 6.0;

fn quantize(b: u8) -> u8 {
    ((b as f64 * (N_STEPS / 255.0)).round() * (255.0 / N_STEPS)).round() as u8
}

pub fn make_image_two(ctx: &mut PaintCtx, w: usize, h: usize, d0: &[u8], d1: &[u8]) -> Image {
    let mut buf = vec![255u8; w * h * 4];
    for i in 0..(w * h) {
        let r = quantize(d0[i]);
        let g = quantize(d1[i]);
        buf[i * 4] = r;
        buf[i * 4 + 1] = g;
        buf[i * 4 + 2] = g;
    }
    ctx.make_image(w, h, &buf, ImageFormat::RgbaPremul).unwrap()
}
