//! The standalone stats banner: three metrics in a row, split by vertical
//! rules, no character. A separate output from the reel — `awan-profile stats`
//! writes it — because it answers a different question ("the numbers, at a
//! glance") and wants the whole width to itself.
//!
//! Each box is `value` (the headline number, gold), `label` (what it counts),
//! and `note` (the span it covers). The renderer only lays them out; CI formats
//! the strings — commas in the count, the dates in the note — so nothing here
//! knows anything about GitHub.

use image::{Rgba, RgbaImage};

use crate::draw::{draw_text, fill};
use crate::gif::BG;
use crate::script::Profile;

const W: u32 = 1056;
const H: u32 = 210;
/// The headline number, in the accent every awan value uses.
const GOLD: [u8; 3] = [230, 180, 100];
/// The label under it.
const INK: [u8; 3] = [150, 150, 160];
/// The date note — quieter still.
const DIM: [u8; 3] = [96, 100, 112];
/// The vertical rules between boxes.
const RULE: [u8; 3] = [52, 56, 66];

/// Render the stats banner from `profile.stat_boxes` to a PNG at `path`.
pub fn render_stats(profile: &Profile, path: &str) -> std::io::Result<()> {
    if let Some(dir) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut img = RgbaImage::from_pixel(W, H, Rgba([BG[0], BG[1], BG[2], 255]));
    let boxes = &profile.stat_boxes;
    let n = boxes.len().max(1) as u32;
    let col = W / n;

    // vertical rules between the boxes, inset top and bottom
    for i in 1..n {
        fill(&mut img, i * col - 1, 34, 2, H - 68, RULE);
    }
    for (i, b) in boxes.iter().enumerate() {
        let cx = i as u32 * col + col / 2;
        centered(&mut img, &b.value, cx, 46, 5, GOLD);
        centered(&mut img, &b.label, cx, 122, 2, INK);
        centered(&mut img, &b.note, cx, 160, 1, DIM);
    }
    img.save(path)
        .map_err(|e| std::io::Error::other(e.to_string()))
}

/// Draw `text` centred on `cx` at `scale`. font8x8 advances a fixed 8·scale per
/// glyph, so the width is just the character count.
fn centered(img: &mut RgbaImage, text: &str, cx: u32, y: u32, scale: u32, colour: [u8; 3]) {
    let w = text.chars().count() as u32 * 8 * scale;
    draw_text(img, text, cx.saturating_sub(w / 2), y, scale, colour);
}
