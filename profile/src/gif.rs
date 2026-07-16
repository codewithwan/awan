//! Rasterise a seam-free reel to a looping GIF. Each canvas cell becomes a
//! solid coloured rectangle — no font needed for the character — and the
//! profile narration (icon + text) is drawn below the ground line with a small
//! bitmap font, so the result drops straight into a README.

use std::fs::File;

use awan_core::Reel;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, Rgba, RgbaImage};

use crate::draw::{draw_bits, draw_text, fill};
use crate::script::{Line, Profile};
use crate::wall::wall;
use awan_core::icons;

/// Pixels per canvas cell (32 cols × this ≈ 1050 px wide — safe in VHS too).
pub const CELL_W: u32 = 33;
pub const CELL_H: u32 = 30;
/// The caption strip below the ground line.
const CAPTION_H: u32 = 56;
/// Bitmap-font / icon pixel size for the caption, and the smaller lyric size.
const SCALE: u32 = 3;
const LYRIC_SCALE: u32 = 2;
/// The big number on a bento card.
const STAT_SCALE: u32 = 3;
/// Rightmost x the lyric panel may reach, before the character.
const LYRIC_LIMIT: u32 = 18 * CELL_W;
/// Backdrop, ground line, caption ink, and icon accent.
pub const BG: [u8; 3] = [13, 17, 23];
const GROUND: [u8; 3] = [80, 84, 96];
const INK: [u8; 3] = [150, 150, 160];
const ACCENT: [u8; 3] = [230, 180, 100];
/// Frame time in milliseconds (~11 fps, matching the terminal cadence).
const FRAME_MS: u32 = 90;

/// Render one seamless loop of `reel`, narrating `profile`, to a GIF at `path`.
pub fn render_gif(reel: &Reel, profile: &Profile, path: &str) -> std::io::Result<()> {
    if let Some(dir) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(dir)?; // e.g. create assets/ on first run
    }
    let mut encoder = GifEncoder::new(File::create(path)?);
    let _ = encoder.set_repeat(Repeat::Infinite);
    for t in 0..reel.ticks() {
        let delay = Delay::from_numer_denom_ms(FRAME_MS, 1);
        let frame = Frame::from_parts(rasterize(reel, profile, t), 0, 0, delay);
        encoder
            .encode_frame(frame)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
    }
    Ok(())
}

/// Paint the canvas at tick `t`: coloured cells, a ground line, then the
/// current narration line (icon + text), centred.
fn rasterize(reel: &Reel, profile: &Profile, t: i32) -> RgbaImage {
    let (cols, rows, cells) = reel.pixel_grid(t);
    let w = cols as u32 * CELL_W;
    let ground = rows as u32 * CELL_H;
    let mut img = RgbaImage::from_pixel(w, ground + CAPTION_H, Rgba([BG[0], BG[1], BG[2], 255]));

    for (i, cell) in cells.iter().enumerate() {
        let Some([r, g, b]) = *cell else { continue };
        let (x0, y0) = ((i % cols) as u32 * CELL_W, (i / cols) as u32 * CELL_H);
        fill(&mut img, x0, y0, CELL_W, CELL_H, [r, g, b]);
    }
    fill(&mut img, 0, ground, w, 2, GROUND);
    match profile.contributions_at(reel, t) {
        // the sign rolls straight through the badge's corner, so the badge
        // stands down for the one beat it would collide with
        Some(k) => wall(&mut img, profile, k),
        None => streak_badge(&mut img, profile.streak, w),
    }
    if let Some(k) = profile.stats_at(reel, t) {
        stat_labels(&mut img, profile, k);
    }

    match profile.sing_at(reel, t) {
        Some(k) => karaoke(&mut img, profile, k, ground),
        None => caption(&mut img, &profile.line(reel, t), w, ground),
    }
    img
}

/// Print the readout into the little terminal window: the label, a dotted
/// leader, the value pushed right — each line typing itself out, cursor and
/// all. Values come from the config as `"label:value"`.
fn stat_labels(img: &mut RgbaImage, profile: &Profile, k: i32) {
    let (px, py, pw, ph) = awan_core::stats::PANEL;
    let (left, top) = (px as u32 * CELL_W, py as u32 * CELL_H);
    let glyph = 8 * STAT_SCALE;
    // Fit the readout inside the window's border cells, keeping a glyph of air
    // on every side, then centre what's left over.
    let (inner_w, inner_h) = ((pw as u32 - 2) * CELL_W, (ph as u32 - 2) * CELL_H);
    let room = (inner_w / glyph).saturating_sub(2) as usize;
    let x = left + CELL_W + (inner_w - room as u32 * glyph) / 2;
    let step = glyph + 12;
    let block = (awan_core::stats::SLOTS as u32 - 1) * step + glyph;
    let y0 = top + CELL_H + (inner_h.saturating_sub(block)) / 2;

    for (i, entry) in profile
        .stats
        .iter()
        .take(awan_core::stats::SLOTS)
        .enumerate()
    {
        let shown = awan_core::stats::chars_at(k, i);
        if shown == 0 {
            continue;
        }
        let (label, value) = entry.split_once(':').unwrap_or(("", entry.as_str()));
        // dotted leader, so the values line up down the right edge
        let gap = room.saturating_sub(label.chars().count() + value.chars().count() + 1);
        let line = format!("{label}{} {value}", ".".repeat(gap));
        let text: String = line.chars().take(shown).collect();

        let y = y0 + i as u32 * step;
        draw_text(img, &text, x, y, STAT_SCALE, INK);
        if awan_core::stats::typing(k, i) {
            let cx = x + text.chars().count() as u32 * glyph;
            fill(img, cx, y, glyph / 2, glyph, INK);
        }
    }
}

/// A centred narration line (icon + text) below the ground.
fn caption(img: &mut RgbaImage, line: &Line, w: u32, ground: u32) {
    let gap = SCALE * 3;
    let icon_w = line.icon.map_or(0, |_| 8 * SCALE + gap);
    let text_w = line.text.chars().count() as u32 * 8 * SCALE;
    let mut x = w.saturating_sub(icon_w + text_w) / 2;
    let y = ground + 20;
    if let Some(icon) = line.icon {
        draw_bits(img, &icon.0, x, y, SCALE, ACCENT);
        x += icon_w;
    }
    draw_text(img, &line.text, x, y, SCALE, INK);
}

/// One karaoke line down the left while he sings on the right — small type,
/// clipped so it never runs under him.
fn karaoke(img: &mut RgbaImage, profile: &Profile, k: i32, ground: u32) {
    let line = profile.lyric(k);
    let fit = (LYRIC_LIMIT.saturating_sub(24) / (8 * LYRIC_SCALE)) as usize;
    let text: String = line.text.chars().take(fit).collect();
    let y = ground / 2 - 4 * LYRIC_SCALE;
    if let Some(icon) = line.icon {
        draw_bits(img, &icon.0, 24, y, LYRIC_SCALE, ACCENT);
    }
    draw_text(img, &text, 24 + 8 * LYRIC_SCALE + 6, y, LYRIC_SCALE, INK);
}

/// A pinned `🔥 N` streak badge in the top-right corner.
fn streak_badge(img: &mut RgbaImage, streak: u32, w: u32) {
    if streak == 0 {
        return;
    }
    let num = streak.to_string();
    let text_w = num.chars().count() as u32 * 8 * SCALE;
    let x = w.saturating_sub(8 * SCALE + SCALE * 2 + text_w + 14);
    let y = 12;
    draw_bits(img, &icons::FIRE.0, x, y, SCALE, ACCENT);
    draw_text(img, &num, x + 8 * SCALE + SCALE * 2, y, SCALE, ACCENT);
}
