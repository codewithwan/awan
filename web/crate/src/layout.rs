//! Where a scene's overlays go, and how far along they are.
//!
//! The engine draws a scene's *shapes* and leaves its *words and numbers* to
//! whoever is rendering — the GIF encoder does exactly this, and so must the
//! preview, or the two headline acts play to an empty stage. These mirror what
//! `awan-core` publishes for the profile generator; nothing is decided here.

use awan_core::icons;
use font8x8::{BASIC_FONTS, UnicodeFonts};
use wasm_bindgen::prelude::*;

/// The readout's window, in cells: `[x, y, w, h]`.
#[wasm_bindgen]
pub fn stats_panel() -> Vec<i32> {
    let (x, y, w, h) = awan_core::stats::PANEL;
    vec![x, y, w, h]
}

/// How many characters of readout line `i` have typed out at tick `k`.
#[wasm_bindgen]
pub fn stats_chars_at(k: i32, i: usize) -> usize {
    awan_core::stats::chars_at(k, i)
}

/// True while line `i` is still typing, so the preview parks a cursor there.
#[wasm_bindgen]
pub fn stats_typing(k: i32, i: usize) -> bool {
    awan_core::stats::typing(k, i)
}

#[wasm_bindgen]
pub fn stats_slots() -> usize {
    awan_core::stats::SLOTS
}

/// The wall's band, in cells: `[x, y, w, h]`.
#[wasm_bindgen]
pub fn wall_band() -> Vec<i32> {
    let (x, y, w, h) = awan_core::contributions::WALL;
    vec![x, y, w, h]
}

/// How far up the wall is at tick `k`, 0-100.
#[wasm_bindgen]
pub fn wall_fade(k: i32) -> u32 {
    awan_core::contributions::fade_pct(k)
}

/// How lit the last thirty days are at tick `k`, 0-100.
#[wasm_bindgen]
pub fn wall_glow(k: i32) -> u32 {
    awan_core::contributions::glow_pct(k)
}

/// Columns, rows, and how many days on the end get the spotlight.
#[wasm_bindgen]
pub fn wall_shape() -> Vec<usize> {
    vec![
        awan_core::contributions::WEEKS,
        awan_core::contributions::DAYS,
        awan_core::contributions::RECENT,
    ]
}

/// The 8×8 bitmap for `ch` — the exact glyph the GIF renderer draws, from the
/// same font crate. Eight bytes, one per row; bit `1 << col` is a lit pixel.
/// Empty for a character the font doesn't carry, which is what the renderer
/// does too: it skips the glyph and still advances the cursor.
#[wasm_bindgen]
pub fn glyph(ch: char) -> Vec<u8> {
    BASIC_FONTS.get(ch).map(|g| g.to_vec()).unwrap_or_default()
}

/// The 8×8 icon a caption carries, by name. Same bitmaps the GIF draws.
#[wasm_bindgen]
pub fn icon(name: &str) -> Vec<u8> {
    let i = match name {
        "heart" => icons::HEART,
        "pin" => icons::PIN,
        "code" => icons::CODE,
        "star" => icons::STAR,
        "fire" => icons::FIRE,
        "briefcase" => icons::BRIEFCASE,
        "globe" => icons::GLOBE,
        _ => icons::DIAMOND,
    };
    i.0.to_vec()
}
