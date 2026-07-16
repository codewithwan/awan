//! The engine, in the browser.
//!
//! A thin bridge over [`awan_core`] so the config editor can show a real reel
//! rather than a mock-up. The engine is a pure function of `(tick, character)`
//! — no clock, no RNG, no I/O — which is exactly why it survives the trip to
//! wasm at all, and why the frames drawn here are the same frames CI draws.
//!
//! It hands JavaScript flat buffers rather than objects: one allocation per
//! frame instead of 384, which keeps a 60fps canvas loop honest.

use awan_core::{Act, Character, Reel};
use wasm_bindgen::prelude::*;

/// A reel built from a story, ready to draw.
#[wasm_bindgen]
pub struct Preview {
    reel: Reel,
    cols: usize,
    rows: usize,
}

#[wasm_bindgen]
impl Preview {
    /// Build a reel from act names — the same strings a reader writes in
    /// `awan.json`. Unknown names fall back to `present`, so a typo costs you a
    /// beat rather than the whole preview.
    #[wasm_bindgen(constructor)]
    pub fn new(acts: Vec<String>) -> Preview {
        let acts: Vec<Act> = acts.iter().map(|a| act_of(a)).collect();
        let reel = Reel::story(Character::default(), &acts);
        let (cols, rows, _) = reel.pixel_grid(0);
        Preview { reel, cols, rows }
    }

    /// Frames in one loop. The last frame is byte-identical to the first, so
    /// the canvas can wrap without a seam.
    pub fn ticks(&self) -> i32 {
        self.reel.ticks()
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    /// The canvas at tick `t`, as `cols * rows * 4` bytes of RGBA. An empty
    /// cell is transparent, so the page's own background shows through and the
    /// caller doesn't have to know ours.
    pub fn frame(&self, t: i32) -> Vec<u8> {
        let (cols, rows, cells) = self.reel.pixel_grid(t);
        let mut out = vec![0u8; cols * rows * 4];
        for (i, cell) in cells.iter().enumerate() {
            if let Some([r, g, b]) = *cell {
                out[i * 4] = r;
                out[i * 4 + 1] = g;
                out[i * 4 + 2] = b;
                out[i * 4 + 3] = 255;
            }
        }
        out
    }

    /// Which beat is playing at tick `t`, or `-1` while he's walking on or off.
    /// The editor uses it to highlight the scene being watched.
    pub fn beat_at(&self, t: i32) -> i32 {
        self.reel.act_at(t).map_or(-1, |(i, _)| i as i32)
    }

    /// The tick within the current beat, or `-1` off-beat. Overlays that time
    /// themselves — the readout typing, the wall rising — key off this.
    pub fn beat_tick(&self, t: i32) -> i32 {
        self.reel.act_at(t).map_or(-1, |(_, k)| k)
    }

    /// True while he's walking out at the end, when the reel says its goodbye
    /// line instead of the beat's.
    pub fn is_leaving(&self, t: i32) -> bool {
        self.reel.is_leaving(t)
    }
}

/// How long a story runs, without building it — so the editor can price a beat
/// before you commit to it. That is the whole reason a reel gets too long: you
/// cannot feel the cost of an act while you're adding it.
#[wasm_bindgen]
pub fn story_ticks(acts: Vec<String>) -> i32 {
    Preview::new(acts).ticks()
}

/// Where the overlays go, and how far along they are.
///
/// The engine draws a scene's *shapes* and leaves its *words and numbers* to
/// whoever is rendering — the GIF encoder does exactly this, and so must the
/// preview, or the two headline acts play to an empty stage. These mirror what
/// `awan-core` publishes for the profile generator.
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

/// The act vocabulary, mirrored from the profile generator. Kept as strings on
/// purpose: `awan.json` is the contract, not our enum.
fn act_of(name: &str) -> Act {
    match name {
        "wave" => Act::Wave,
        "stroll" => Act::Stroll,
        "rocket" => Act::RocketBuild,
        "launch" => Act::RocketLaunch,
        "bake" => Act::Bake,
        "sing" => Act::Sing,
        "campfire" => Act::Campfire,
        "stats" => Act::Stats,
        "contributions" => Act::Contributions,
        "sleep" => Act::Sleep,
        "dance" => Act::Dance,
        "soccer" => Act::Soccer,
        _ => Act::Present,
    }
}
