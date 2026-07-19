//! The standalone stats banner: three metrics in a row, split by vertical
//! rules, no character. A separate output from the reel — `awan-profile stats`
//! writes it — because it answers a different question ("the numbers, at a
//! glance") and wants the whole width to itself.
//!
//! Each box is `value` (the headline number, gold), `label` (what it counts),
//! and `note` (the span it covers). The renderer only lays them out; CI formats
//! the strings — commas in the count, the dates in the note — so nothing here
//! knows anything about GitHub.
//!
//! A `.gif` path drifts soft clouds behind the numbers and loops; a `.png` is
//! the still. The clouds are the one nod to the character's world — awan means
//! cloud — without a character walking through the readout.

use std::fs::File;

use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, Rgba, RgbaImage};

use crate::draw::{draw_text, draw_text_frac, fill, text_w_frac};
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
/// Drifting clouds — barely above the backdrop, so they read as depth, not data.
/// `#` is the core, `+` a fainter edge that softens the silhouette.
const CLOUD: [u8; 3] = [30, 37, 50];
const CLOUD_EDGE: [u8; 3] = [20, 26, 36];
/// Frames in one loop, and the per-frame delay (~11 fps, matching the reel).
const LOOP: i32 = 90;
const FRAME_MS: u32 = 90;

/// A soft cloud sprite, blitted big and faint behind the numbers — rounded, with
/// a `+` edge so it doesn't read as a staircase of grey blocks.
const PUFF: &[&str] = &[
    "   ++####++   ",
    "  +########+  ",
    " +##########+ ",
    "+############+",
    " +##########+ ",
    "   ++####++   ",
];

/// Render the stats banner from `profile.stat_boxes`. A `.gif` path animates
/// (drifting clouds, looping); anything else is written as a still PNG.
pub fn render_stats(profile: &Profile, path: &str) -> std::io::Result<()> {
    if let Some(dir) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(dir)?;
    }
    if path.ends_with(".gif") {
        let mut enc = GifEncoder::new(File::create(path)?);
        let _ = enc.set_repeat(Repeat::Infinite);
        for t in 0..LOOP {
            let delay = Delay::from_numer_denom_ms(FRAME_MS, 1);
            let frame = Frame::from_parts(banner(profile, Some(t)), 0, 0, delay);
            enc.encode_frame(frame).map_err(err)?;
        }
        Ok(())
    } else {
        banner(profile, None).save(path).map_err(err)
    }
}

fn err(e: image::ImageError) -> std::io::Error {
    std::io::Error::other(e.to_string())
}

/// One frame: the backdrop, drifting clouds if animated, the rules, the numbers.
fn banner(profile: &Profile, t: Option<i32>) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(W, H, Rgba([BG[0], BG[1], BG[2], 255]));
    if let Some(t) = t {
        // two clouds, parallax: the near one crosses twice per loop. Each drifts
        // a whole number of spans over LOOP, so frame 0 and frame LOOP match.
        cloud(&mut img, t, 1, 22, 17, W / 5);
        cloud(&mut img, t, 2, 116, 13, 3 * W / 5);
    }
    let boxes = &profile.stat_boxes;
    let n = boxes.len().max(1) as u32;
    let col = W / n;
    for i in 1..n {
        fill(&mut img, i * col - 1, 34, 2, H - 68, RULE);
    }
    for (i, b) in boxes.iter().enumerate() {
        let cx = i as u32 * col + col / 2;
        centered(&mut img, &b.value, cx, 42, 5, GOLD);
        centered(&mut img, &b.label, cx, 118, 2, INK);
        // the note sits at 1.5× — between the font's 8px and 16px steps
        let nw = text_w_frac(&b.note, 3, 2);
        draw_text_frac(&mut img, &b.note, cx.saturating_sub(nw / 2), 158, 3, 2, DIM);
    }
    img
}

/// One drifting cloud: `spans` crossings per loop, top at `y`, `px` per art
/// pixel, starting at `start`. Wraps seamlessly because the drift over `LOOP` is
/// exactly `spans` whole spans.
fn cloud(img: &mut RgbaImage, t: i32, spans: i32, y: u32, px: u32, start: u32) {
    let cw = PUFF[0].len() as u32 * px;
    let span = (W + cw) as i32;
    let x = (start as i32 - spans * span * t / LOOP).rem_euclid(span) - cw as i32;
    for (r, row) in PUFF.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            let colour = match ch {
                '#' => CLOUD,
                '+' => CLOUD_EDGE,
                _ => continue,
            };
            let px0 = x + (c as u32 * px) as i32;
            if px0 + (px as i32) <= 0 || px0 >= W as i32 {
                continue;
            }
            let x0 = px0.max(0) as u32;
            let w = (px0 + px as i32).min(W as i32) as u32 - x0;
            fill(img, x0, y + r as u32 * px, w, px, colour);
        }
    }
}

/// Draw `text` centred on `cx` at `scale`. font8x8 advances a fixed 8·scale per
/// glyph, so the width is just the character count.
fn centered(img: &mut RgbaImage, text: &str, cx: u32, y: u32, scale: u32, colour: [u8; 3]) {
    let w = text.chars().count() as u32 * 8 * scale;
    draw_text(img, text, cx.saturating_sub(w / 2), y, scale, colour);
}
