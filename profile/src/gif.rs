//! Rasterise a seam-free reel to a looping GIF. Each canvas cell becomes a
//! solid coloured rectangle — no font needed for the character — and the
//! scene's caption is drawn below the ground line with a small bitmap font, so
//! the result drops straight into a README.

use std::fs::File;

use awan_core::Reel;
use font8x8::{BASIC_FONTS, UnicodeFonts};
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, Rgba, RgbaImage};

/// Pixels per canvas cell (32 cols × this ≈ 1050 px wide — safe in VHS too).
const CELL_W: u32 = 33;
const CELL_H: u32 = 30;
/// The caption strip below the ground line.
const CAPTION_H: u32 = 56;
/// Bitmap-font pixel size for the caption.
const SCALE: u32 = 3;
/// Backdrop, ground line, and caption ink.
const BG: [u8; 4] = [13, 17, 23, 255];
const GROUND: [u8; 3] = [80, 84, 96];
const INK: [u8; 3] = [150, 150, 160];
/// Frame time in milliseconds (~11 fps, matching the terminal cadence).
const FRAME_MS: u32 = 90;

/// Render one seamless loop of `reel` to a GIF at `path`.
pub fn render_gif(reel: &Reel, path: &str) -> std::io::Result<()> {
    let mut encoder = GifEncoder::new(File::create(path)?);
    let _ = encoder.set_repeat(Repeat::Infinite);
    for t in 0..reel.ticks() {
        let delay = Delay::from_numer_denom_ms(FRAME_MS, 1);
        let frame = Frame::from_parts(rasterize(reel, t), 0, 0, delay);
        encoder
            .encode_frame(frame)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
    }
    Ok(())
}

/// Paint the canvas at tick `t`: coloured cells, a ground line, then a caption.
fn rasterize(reel: &Reel, t: i32) -> RgbaImage {
    let (cols, rows, cells) = reel.pixel_grid(t);
    let w = cols as u32 * CELL_W;
    let ground = rows as u32 * CELL_H;
    let mut img = RgbaImage::from_pixel(w, ground + CAPTION_H, Rgba(BG));

    for (i, cell) in cells.iter().enumerate() {
        let Some([r, g, b]) = *cell else { continue };
        let (x0, y0) = ((i % cols) as u32 * CELL_W, (i / cols) as u32 * CELL_H);
        fill(&mut img, x0, y0, CELL_W, CELL_H, [r, g, b]);
    }
    fill(&mut img, 0, ground, w, 2, GROUND);
    if let Some(cap) = reel.caption(t) {
        draw_text(&mut img, &format!("{}: {cap}", reel.name()), ground + 20);
    }
    img
}

/// Fill a `w`×`h` rectangle at `(x0, y0)`, clipped to the image.
fn fill(img: &mut RgbaImage, x0: u32, y0: u32, w: u32, h: u32, c: [u8; 3]) {
    let px = Rgba([c[0], c[1], c[2], 255]);
    for y in y0..(y0 + h).min(img.height()) {
        for x in x0..(x0 + w).min(img.width()) {
            img.put_pixel(x, y, px);
        }
    }
}

/// Draw `text` centred horizontally at vertical offset `y`, in the ink colour.
fn draw_text(img: &mut RgbaImage, text: &str, y: u32) {
    let glyph_w = 8 * SCALE;
    let width = text.chars().count() as u32 * glyph_w;
    let mut cx = img.width().saturating_sub(width) / 2;
    for chr in text.chars() {
        if let Some(glyph) = BASIC_FONTS.get(chr) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8u32 {
                    if bits & (1 << col) != 0 {
                        fill(
                            img,
                            cx + col * SCALE,
                            y + row as u32 * SCALE,
                            SCALE,
                            SCALE,
                            INK,
                        );
                    }
                }
            }
        }
        cx += glyph_w;
    }
}
