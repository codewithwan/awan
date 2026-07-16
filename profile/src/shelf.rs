//! Labels under the trophies — the reader's numbers, in the engine's own font.
//!
//! The engine puts the cups up and says which have landed; what each one is
//! *for* is data, so it's ours to draw. Same split as the readout and the wall.

use image::RgbaImage;

use crate::draw::{draw_text, mix};
use crate::gif::{BG, CELL_H, CELL_W};
use crate::script::Profile;

/// Small type: a cup is six cells wide and the label sits under it.
const SCALE: u32 = 2;
const GLYPH: u32 = 8 * SCALE;
const INK: [u8; 3] = [150, 150, 160];
const ACCENT: [u8; 3] = [230, 180, 100];

/// Paint the labels at tick `k` of a trophies beat.
pub fn shelf(img: &mut RgbaImage, profile: &Profile, k: i32) {
    let up = awan_core::trophies::stand_pct(k);
    let landed = awan_core::trophies::landed(k);
    if up == 0 || landed == 0 {
        return;
    }
    let (sx, sy, _) = awan_core::trophies::SHELF;
    let pitch = awan_core::trophies::PITCH as u32 * CELL_W;
    let y = (sy as u32 + 1) * CELL_H + 6;

    for (i, entry) in profile
        .trophies
        .iter()
        .take(landed.min(awan_core::trophies::SLOTS))
        .enumerate()
    {
        let (label, value) = entry.split_once(':').unwrap_or((entry.as_str(), ""));
        let x0 = (sx as u32 + 1) * CELL_W + i as u32 * pitch;
        // the number leads: it's what the cup is for
        centred(img, value, x0, y, ACCENT, up);
        centred(img, label, x0, y + GLYPH + 4, INK, up);
    }
}

/// Centre a line under a six-cell cup, fading in with the shelf.
fn centred(img: &mut RgbaImage, text: &str, x0: u32, y: u32, colour: [u8; 3], up: u32) {
    let w = 6 * CELL_W;
    let text_w = text.chars().count() as u32 * GLYPH;
    let x = x0 + w.saturating_sub(text_w) / 2;
    draw_text(img, text, x, y, SCALE, mix(BG, colour, up));
}
