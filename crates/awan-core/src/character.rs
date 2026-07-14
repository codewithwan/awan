//! The runtime character: spec data resolved into render-ready art. Face
//! variants are *derived* from the base rows (blink `@`→`-`, happy `@`→`^`,
//! glances shift a pixel, the startled mouth opens, charred `@`→`%`), so
//! contributing a character stays zero-Rust.

use std::collections::BTreeMap;

use crate::palette::hex_to_sgr;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::spec::{CharacterSpec, SpecError};

/// The buddy's width in pixels (pinned by spec v1).
pub(crate) const MASCOT_W: i32 = 10;

#[derive(Clone)]
pub(crate) struct Colors {
    pub body: String,
    pub eye: String,
    pub eye_white: String,
    pub charred: String,
}

/// A character ready to render: resolved palette plus precomputed row
/// variants, so composing a mascot frame allocates nothing.
#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub(crate) colors: Colors,
    base: Vec<String>,
    sit: Vec<String>,
    legs: Vec<String>,
    shimmer: Option<String>,
    eyes: [String; 5],
    eyes_charred: [String; 5],
    mouth_open: String,
    eye_row: usize,
    mouth_row: usize,
    legs_row: usize,
    blink_period: i32, // idle-blink period from personality.blink_rate (1.0 → 26)
    reactions: BTreeMap<String, String>, // event name → scene name
}

/// The built-in reference character (identical to `characters/awan.toml`).
const AWAN_SPEC: &str = include_str!("awan.toml");

impl Default for Character {
    fn default() -> Self {
        let spec = toml::from_str(AWAN_SPEC).expect("built-in spec parses");
        Character::from_spec(&spec).expect("built-in spec is valid")
    }
}

fn eye_variants(base: &str) -> [String; 5] {
    let chars: Vec<char> = base.chars().collect();
    let (first, last) = (chars[0], chars[chars.len() - 1]);
    let left: String = chars[1..].iter().chain([&last]).collect();
    let right: String = [&first]
        .into_iter()
        .chain(&chars[..chars.len() - 1])
        .collect();
    [
        base.to_string(),       // EyeMode::Auto
        base.replace('@', "-"), // EyeMode::Closed
        left,                   // EyeMode::Left
        right,                  // EyeMode::Right
        base.replace('@', "^"), // EyeMode::Happy
    ]
}

impl Character {
    /// Resolve a validated spec into render-ready art.
    pub fn from_spec(spec: &CharacterSpec) -> Result<Self, SpecError> {
        spec.validate()?;
        let color = |key: &str, fallback: &str| -> Result<String, SpecError> {
            match spec.character.palette.get(key) {
                Some(hex) => hex_to_sgr(hex)
                    .ok_or_else(|| SpecError::Invalid(format!("palette.{key}: bad hex {hex:?}"))),
                None => Ok(fallback.to_string()),
            }
        };
        let colors = Colors {
            body: color("body", "38;2;235;238;245")?,
            eye: color("eye", "38;2;45;48;60")?,
            eye_white: color("eye_white", "38;2;250;250;245")?,
            charred: color("charred", "38;2;85;80;78")?,
        };

        let eye_base = &spec.sprite.rows[spec.sprite.eye_row];
        let eyes = eye_variants(eye_base);
        let eyes_charred = eyes.clone().map(|row| row.replace('@', "%"));

        let mouth_base = &spec.sprite.rows[spec.sprite.mouth_row];
        let mid = mouth_base.chars().count() / 2;
        let mouth_open: String = mouth_base
            .chars()
            .enumerate()
            .map(|(i, c)| if i == mid - 1 || i == mid { '@' } else { c })
            .collect();

        let bp = 26.0 / spec.personality.blink_rate;
        let blink_period = bp.round().clamp(4.0, 60.0) as i32;

        Ok(Self {
            name: spec.character.name.clone(),
            colors,
            base: spec.sprite.rows.clone(),
            sit: spec.sprite.sit_rows.clone(),
            legs: spec.sprite.leg_frames.clone(),
            shimmer: spec.sprite.shimmer_row.clone(),
            eyes,
            eyes_charred,
            mouth_open,
            eye_row: spec.sprite.eye_row,
            mouth_row: spec.sprite.mouth_row,
            legs_row: spec.sprite.legs_row,
            blink_period,
            reactions: spec.reactions.clone(),
        })
    }

    /// The scene name this character reacts to `event` with, if any.
    pub fn reaction(&self, event: &str) -> Option<&str> {
        self.reactions.get(event).map(String::as_str)
    }

    /// Assemble the mascot rows for a pose at tick `t` — borrowed, zero-alloc.
    pub(crate) fn mascot_rows(&self, p: Pose, t: i32) -> [&str; 6] {
        let mut rows = [""; 6];
        let sitting = p.legs == LegsMode::Sit;
        let (source, eye_idx, mouth_idx) = if sitting {
            (&self.sit, self.eye_row + 1, self.mouth_row + 1)
        } else {
            (&self.base, self.eye_row, self.mouth_row)
        };
        for (slot, row) in rows.iter_mut().zip(source) {
            *slot = row;
        }
        if !sitting {
            if let Some(shimmer) = &self.shimmer {
                if (t / 4) % 2 == 1 {
                    rows[0] = shimmer; // fluff shimmer
                }
            }
            if p.legs == LegsMode::Walk {
                rows[self.legs_row] = &self.legs[((t / 2) % 4) as usize];
            }
        }

        let mut eyes = p.eyes;
        if eyes == EyeMode::Auto && t % self.blink_period < 2 {
            eyes = EyeMode::Closed; // idle blink, paced by personality.blink_rate
        }
        let variants = if p.charred {
            &self.eyes_charred
        } else {
            &self.eyes
        };
        rows[eye_idx] = &variants[eyes as usize];
        if p.mouth_open {
            rows[mouth_idx] = &self.mouth_open;
        }
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_eye_rows_match_the_original_hand_drawn_variants() {
        let eyes = eye_variants("##@@##@@##");
        assert_eq!(
            eyes,
            [
                "##@@##@@##",
                "##--##--##",
                "#@@##@@###",
                "###@@##@@#",
                "##^^##^^##"
            ]
        );
    }

    #[test]
    fn builtin_awan_matches_the_reference_spec_file() {
        let repo = concat!(env!("CARGO_MANIFEST_DIR"), "/../../characters/awan.toml");
        let file = std::fs::read_to_string(repo).expect("characters/awan.toml exists");
        assert_eq!(
            file, AWAN_SPEC,
            "characters/awan.toml and the embedded copy must stay identical"
        );
    }

    #[test]
    fn derived_open_mouth_matches_the_original() {
        let ch = Character::default();
        assert_eq!(ch.mouth_open, "####@@####");
    }
}
