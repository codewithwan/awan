//! The engine, in the browser.
//!
//! A thin bridge over [`awan_core`] so the config editor can show a real reel
//! rather than a mock-up. The engine is a pure function of `(tick, character)`
//! — no clock, no RNG, no I/O — which is exactly why it survives the trip to
//! wasm at all, and why the frames drawn here are the same frames CI draws.
//!
//! It hands JavaScript flat buffers rather than objects: one allocation per
//! frame instead of 384, which keeps a 60fps canvas loop honest.

mod layout;

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
    pub fn new(acts: Vec<String>, character_toml: Option<String>) -> Preview {
        let acts: Vec<Act> = acts.iter().map(|a| act_of(a)).collect();
        let reel = Reel::story(character_of(character_toml.as_deref()), &acts);
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

/// Why a spec won't load, in the engine's own words — or `None` if it will.
///
/// The editor used to re-implement these rules in JavaScript, which meant two
/// sources of truth and one of them wrong: a character with no eyes fell back
/// to the built-in buddy *silently*, so you drew a cat and watched a cloud walk
/// past. The rules live in one place. Ask them.
#[wasm_bindgen]
pub fn check_spec(toml: &str) -> Option<String> {
    match awan_core::spec::parse(toml) {
        Err(e) => Some(e.to_string()),
        Ok(spec) => Character::from_spec(&spec).err().map(|e| e.to_string()),
    }
}

/// A character from its TOML spec, or the built-in buddy. A spec that doesn't
/// parse falls back rather than failing — callers who care whether that
/// happened should ask [`check_spec`] first, which is what the editor does.
fn character_of(toml: Option<&str>) -> Character {
    toml.and_then(|t| awan_core::spec::parse(t).ok())
        .and_then(|s| Character::from_spec(&s).ok())
        .unwrap_or_default()
}

/// How long a story runs, without building it — so the editor can price a beat
/// before you commit to it. That is the whole reason a reel gets too long: you
/// cannot feel the cost of an act while you're adding it.
#[wasm_bindgen]
pub fn story_ticks(acts: Vec<String>) -> i32 {
    Preview::new(acts, None).ticks()
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
