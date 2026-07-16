//! Character spec — characters are **data** (TOML + pixel maps), not code.
//!
//! Contributing a character requires zero Rust: a spec file binds art and
//! personality onto the engine's shared scene library. See
//! `characters/awan.toml` for the reference character.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

mod validate;

use serde::Deserialize;

/// Supported spec revision; declared by every character so old specs never
/// silently break. Version 1 pins the body to 10×6 shared-choreography pixels.
pub const SPEC_VERSION: u32 = 1;

const SPEC_W: usize = 10;
const SPEC_H: usize = 6;

/// A full character definition, loaded from a `*.toml` spec file.
#[derive(Debug, Clone, Deserialize)]
pub struct CharacterSpec {
    /// Must equal [`SPEC_VERSION`].
    pub spec_version: u32,
    pub character: CharacterMeta,
    pub sprite: SpriteSpec,
    #[serde(default)]
    pub personality: Personality,
    /// Event name → scene name overrides (e.g. `"cmd.failed" = "charred"`).
    #[serde(default)]
    pub reactions: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterMeta {
    pub name: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub description: String,
    /// Named colors, hex `#RRGGBB`: `body`, `eye`, and optionally
    /// `eye_white` and `charred`.
    pub palette: BTreeMap<String, String>,
}

/// Cell-art tier: a pixel map in the sprite glyph language.
/// `'#'` solid · `'+'` dense · `'-'` light · `'@'` eye · `' '` empty.
#[derive(Debug, Clone, Deserialize)]
pub struct SpriteSpec {
    /// Standing body, one string per pixel row (10×6).
    pub rows: Vec<String>,
    /// Sitting body (same size; face rows sit one row lower).
    pub sit_rows: Vec<String>,
    /// Walk-cycle variants for the legs row (4 frames).
    pub leg_frames: Vec<String>,
    /// Optional idle variant of row 0 (e.g. Awan's fluff shimmer).
    #[serde(default)]
    pub shimmer_row: Option<String>,
    /// Row index (into `rows`) animated by the face engine.
    pub eye_row: usize,
    pub mouth_row: usize,
    pub legs_row: usize,
}

/// Multipliers on engine defaults; `1.0` = neutral.
#[derive(Debug, Clone, Deserialize)]
pub struct Personality {
    #[serde(default = "one")]
    pub blink_rate: f32,
    #[serde(default = "one")]
    pub walk_speed: f32,
    /// Chance of mischief idle scenes (0.0–1.0).
    #[serde(default)]
    pub chaos: f32,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            blink_rate: 1.0,
            walk_speed: 1.0,
            chaos: 0.0,
        }
    }
}

fn one() -> f32 {
    1.0
}

/// Errors produced while loading or validating a character spec.
#[derive(Debug)]
pub enum SpecError {
    Io(std::io::Error),
    Parse(toml::de::Error),
    /// Human-readable validation failure.
    Invalid(String),
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpecError::Io(e) => write!(f, "failed to read spec: {e}"),
            SpecError::Parse(e) => write!(f, "failed to parse spec: {e}"),
            SpecError::Invalid(msg) => write!(f, "invalid spec: {msg}"),
        }
    }
}

impl std::error::Error for SpecError {}

/// Parse and validate a character spec from TOML text.
///
/// Split out from [`load`] because the browser has the text but no filesystem,
/// and a preview that can't restyle itself can't show what a character does.
pub fn parse(raw: &str) -> Result<CharacterSpec, SpecError> {
    let spec: CharacterSpec = toml::from_str(raw).map_err(SpecError::Parse)?;
    spec.validate()?;
    Ok(spec)
}

/// Load and validate a character spec from a TOML file.
pub fn load(path: &Path) -> Result<CharacterSpec, SpecError> {
    parse(&std::fs::read_to_string(path).map_err(SpecError::Io)?)
}
