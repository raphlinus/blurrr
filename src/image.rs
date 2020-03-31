use druid::piet::{Image, ImageFormat};
use druid::{PaintCtx, RenderContext};

// Make a grayscale image from a single intensity map.
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
