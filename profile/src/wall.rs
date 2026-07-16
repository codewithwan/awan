//! The year wall's face — a GitHub year, one square per day.
//!
//! The engine hands over the wall's band, how far up it is and how lit the
//! month is; everything drawn here is the reader's data. Levels arrive from
//! `awan.json` as one character per day (`0`–`4`, GitHub's own quartiles, `.`
//! for a day the calendar doesn't cover), so a whole year is one tidy string
//! rather than 371 numbers.

use image::{Rgba, RgbaImage};

use crate::draw::{fill, mix};
use crate::gif::{BG, CELL_H, CELL_W};
use crate::script::Profile;

/// GitHub's five contribution shades, quietest first.
const SHADES: [[u8; 3]; 5] = [
    [22, 27, 34],
    [14, 68, 41],
    [0, 109, 50],
    [38, 166, 65],
    [57, 211, 83],
];
/// Square pitch and size — 53 weeks land inside the band with a margin.
const PITCH: u32 = 18;
const SQUARE: u32 = 14;
/// The lifted sky behind the spotlit month, and the air around it.
const SPOT: [u8; 3] = [32, 40, 52];
const PAD: u32 = 5;
/// How far the older year sinks toward the backdrop once the month lights up.
/// Enough to make the month the subject, gentle enough that a quiet day is
/// still visibly a day.
const FADE: u32 = 45;

/// Paint the wall at tick `k` of a contributions beat.
pub fn wall(img: &mut RgbaImage, profile: &Profile, k: i32) {
    let up = awan_core::contributions::fade_pct(k);
    let days = levels(&profile.contributions);
    if up == 0 || days.is_empty() {
        return;
    }

    let (wx, wy, ww, wh) = awan_core::contributions::WALL;
    let (weeks, rows) = (
        awan_core::contributions::WEEKS,
        awan_core::contributions::DAYS,
    );

    // Sink the band toward the page, so clouds behind the wall fade away with
    // it rather than blinking out the tick it arrives.
    veil(
        img,
        wx as u32 * CELL_W,
        wy as u32 * CELL_H,
        ww as u32 * CELL_W,
        wh as u32 * CELL_H,
        up,
    );

    let x0 = wx as u32 * CELL_W + (ww as u32 * CELL_W - weeks as u32 * PITCH) / 2;
    let y0 = wy as u32 * CELL_H + (wh as u32 * CELL_H - rows as u32 * PITCH) / 2;
    let glow = awan_core::contributions::glow_pct(k);
    let recent = recent_from(&days);
    let first = recent / rows;

    // Lift the sky behind the last thirty days. Four bright columns can't win
    // against fifty-three on their own, and the month is the whole point.
    if glow > 0 {
        fill(
            img,
            x0 + first as u32 * PITCH - PAD,
            y0 - PAD,
            (weeks - first) as u32 * PITCH + PAD,
            rows as u32 * PITCH + PAD,
            mix(BG, mix(BG, SPOT, glow), up),
        );
    }

    for w in 0..weeks {
        for d in 0..rows {
            let Some(Some(level)) = days.get(w * rows + d).copied() else {
                continue; // a day outside the calendar — leave the sky alone
            };
            let mut c = SHADES[level.min(SHADES.len() - 1)];
            if w * rows + d < recent {
                c = mix(c, BG, glow * FADE / 100); // the year steps back
            }
            let (x, y) = (x0 + w as u32 * PITCH, y0 + d as u32 * PITCH);
            fill(img, x, y, SQUARE, SQUARE, mix(BG, c, up));
        }
    }
}

/// Mix a whole region `pct` of the way to the page background.
fn veil(img: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32, pct: u32) {
    for py in y..(y + h).min(img.height()) {
        for px in x..(x + w).min(img.width()) {
            let p = img.get_pixel(px, py).0;
            let c = mix([p[0], p[1], p[2]], BG, pct);
            img.put_pixel(px, py, Rgba([c[0], c[1], c[2], 255]));
        }
    }
}

/// One entry per day: its quartile, or `None` where the calendar has no day.
fn levels(spec: &str) -> Vec<Option<usize>> {
    spec.chars()
        .map(|c| c.to_digit(10).map(|d| d as usize))
        .collect()
}

/// Where the last [`RECENT`](awan_core::contributions::RECENT) real days begin —
/// counted from the end, skipping the padding at the edges of the calendar.
fn recent_from(days: &[Option<usize>]) -> usize {
    let mut seen = 0;
    for (i, day) in days.iter().enumerate().rev() {
        if day.is_some() {
            seen += 1;
            if seen == awan_core::contributions::RECENT {
                return i;
            }
        }
    }
    0 // a short calendar: the whole thing is "recent"
}
