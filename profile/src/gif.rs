//! Rasterise a seam-free reel to a looping GIF. Each canvas cell becomes a
//! solid coloured rectangle — no font needed, since the character is blocks —
//! and the encoder loops forever, so the result drops straight into a README.

use std::fs::File;

use awan_core::Reel;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, Rgba, RgbaImage};

/// Pixels per canvas cell (32 cols × this ≈ 1050 px wide — safe in VHS too).
const CELL_W: u32 = 33;
const CELL_H: u32 = 30;
/// Backdrop behind the character (GitHub dark).
const BG: [u8; 4] = [13, 17, 23, 255];
/// Frame time in milliseconds (~11 fps, matching the terminal cadence).
const FRAME_MS: u32 = 90;

/// Render one seamless loop of `reel` to a GIF at `path`.
pub fn render_gif(reel: &Reel, path: &str) -> std::io::Result<()> {
    let mut encoder = GifEncoder::new(File::create(path)?);
    let _ = encoder.set_repeat(Repeat::Infinite);
    for t in 0..reel.ticks() {
        let frame = Frame::from_parts(
            rasterize(reel, t),
            0,
            0,
            Delay::from_numer_denom_ms(FRAME_MS, 1),
        );
        encoder
            .encode_frame(frame)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
    }
    Ok(())
}

/// Paint the canvas at tick `t` onto an image, one rectangle per filled cell.
fn rasterize(reel: &Reel, t: i32) -> RgbaImage {
    let (cols, rows, cells) = reel.pixel_grid(t);
    let (w, h) = (cols as u32 * CELL_W, rows as u32 * CELL_H);
    let mut img = RgbaImage::from_pixel(w, h, Rgba(BG));
    for (i, cell) in cells.iter().enumerate() {
        let Some([r, g, b]) = *cell else { continue };
        let (x0, y0) = ((i % cols) as u32 * CELL_W, (i / cols) as u32 * CELL_H);
        for y in y0..y0 + CELL_H {
            for x in x0..x0 + CELL_W {
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
    }
    img
}
