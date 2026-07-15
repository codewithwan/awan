//! Greeting scenes for the profile reel: an excited `wave` hello and a calm
//! `present` idle where he just stands so the narration does the talking.

use crate::grid::Grid;
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

const HOME: i32 = 11; // == MASCOT_HOME

/// An enthusiastic hello: happy little jumps, a gentle sway, and sparks of
/// excitement popping by his head on each hop.
pub(super) fn wave(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let up = (k / 4) % 2 == 0;
    if up {
        grid.set(HOME - 1, 4, "* ", Role::Spark);
        grid.set(HOME + 6, 4, " *", Role::Spark);
    }
    Pose {
        legs: LegsMode::Still,
        eyes: EyeMode::Happy,
        dy: if up { -2 } else { 0 },
        dx: match (k / 4) % 4 {
            1 => -1,
            3 => 1,
            _ => 0,
        },
        ..Pose::default()
    }
}

/// A calm idle while the narration introduces him — he stands and breathes
/// (a slow bob and the odd blink), so the ground stays still under him. The
/// travelling happens in the `stroll` beats between scenes.
pub(super) fn present(k: i32, _t: i32, _grid: &mut Grid) -> Pose {
    Pose {
        legs: LegsMode::Still,
        dy: if (k / 6) % 4 == 0 { -1 } else { 0 },
        ..Pose::default()
    }
}
