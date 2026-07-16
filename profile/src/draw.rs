//! Drawing primitives for the rasteriser: filled rectangles, 8×8 bitmaps (font
//! glyphs and icons), text runs, and colour blending.

use font8x8::{BASIC_FONTS, UnicodeFonts};
use image::{Rgba, RgbaImage};

/// Fill a `w`×`h` rectangle at `(x0, y0)`, clipped to the image.
pub fn fill(img: &mut RgbaImage, x0: u32, y0: u32, w: u32, h: u32, c: [u8; 3]) {
    let px = Rgba([c[0], c[1], c[2], 255]);
    for y in y0..(y0 + h).min(img.height()) {
        for x in x0..(x0 + w).min(img.width()) {
            img.put_pixel(x, y, px);
        }
    }
}

/// Blend `pct` percent of the way from `a` to `b`.
pub fn mix(a: [u8; 3], b: [u8; 3], pct: u32) -> [u8; 3] {
    let p = pct.min(100);
    let lerp = |x: u8, y: u8| ((x as u32 * (100 - p) + y as u32 * p) / 100) as u8;
    [lerp(a[0], b[0]), lerp(a[1], b[1]), lerp(a[2], b[2])]
}

/// Draw an 8-row bitmap (font glyph or icon) at `(x, y)`, `scale` px per pixel.
pub fn draw_bits(img: &mut RgbaImage, bits: &[u8; 8], x: u32, y: u32, scale: u32, c: [u8; 3]) {
    for (row, byte) in bits.iter().enumerate() {
        for col in 0..8u32 {
            if byte & (1 << col) != 0 {
                fill(
                    img,
                    x + col * scale,
                    y + row as u32 * scale,
                    scale,
                    scale,
                    c,
                );
            }
        }
    }
}

/// Draw `text` starting at `(x, y)` at `scale`, in colour `c`.
pub fn draw_text(img: &mut RgbaImage, text: &str, x: u32, y: u32, scale: u32, c: [u8; 3]) {
    let mut cx = x;
    for chr in text.chars() {
        if let Some(glyph) = BASIC_FONTS.get(chr) {
            draw_bits(img, &glyph, cx, y, scale, c);
        }
        cx += 8 * scale;
    }
}
