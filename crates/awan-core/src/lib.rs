//! # awan-core
//!
//! The scene engine and face system behind **awan** — a tiny living character
//! for your terminal.
//!
//! Characters are **data**: a TOML spec ([`spec`]) resolves into a
//! [`Character`], and a [`Stage`] composes it with a show (the full skit loop
//! or the "busy" making-things loop) and an intro (walk-in, or hatching from
//! an egg on first run) into deterministic frames.
//!
//! ```no_run
//! use awan_core::{Character, Stage};
//!
//! let stage = Stage::show(Character::default());
//! print!("{}", stage.frame(30, true));
//! ```
//!
//! The engine began as a 1:1 port of a battle-tested Go implementation (the
//! `idl pet` engine from the IDCloud CLI); its scene choreography is tuned
//! and verified frame-by-frame.
//!
//! ## Module map
//!
//! | Module | Contents |
//! |---|---|
//! | [`spec`] | TOML character spec: loading + validation |
//! | `character` | resolved art, palette, derived face variants |
//! | `scene` | the skit library and show tables |
//! | `stage`/`play` | frame composer, terminal renderer, play loop |

mod character;
mod color;
mod companion;
mod grid;
mod halfblock;
mod karaoke;
mod palette;
mod play;
pub mod pose;
mod reel;
mod scene;
mod seamless;
pub mod spec;
mod sprites;
mod stage;
mod statusline;

/// Layout of the `stats` act, for renderers that print the numbers onto the
/// bento cards the character sets out.
pub mod icons;

pub mod stats {
    pub use crate::scene::stats::{PANEL, SLOTS, chars_at, panel_at, typing};
}

/// Where the contribution wall hangs, how far up it is and how lit its month
/// is. The
/// squares are the renderer's job — the engine never sees anyone's numbers.
pub mod contributions {
    pub use crate::scene::contributions::{DAYS, RECENT, WALL, WEEKS, fade_pct, glow_pct};
}

pub use character::Character;
pub use companion::Companion;
pub use karaoke::Karaoke;
pub use pose::Pose;
pub use reel::{Act, Reel};
pub use stage::{Intro, Size, Stage};
pub use statusline::statusline;

#[cfg(test)]
mod tests;
