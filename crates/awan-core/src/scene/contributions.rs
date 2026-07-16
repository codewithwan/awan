//! The year wall: his GitHub calendar rises behind him, he walks over to the
//! newest end of it, and stands there while the last thirty days light up and
//! the rest of the year steps back. The year is the context; the month is the
//! point. Then he strolls home and it sinks away.
//!
//! *He* moves; the wall doesn't. A calendar gliding past a standing character
//! reads as the wall being the subject, when he is. And it fades rather than
//! popping in, because a scene has to start on an empty stage — this is the
//! smoothest way to honour that without making the wall the one that travels.
//!
//! The engine says where the wall hangs, how far up it is and how lit the month
//! is ([`WALL`], [`fade_pct`], [`glow_pct`]). It paints nothing: the squares are
//! the reader's data and the engine never sees them — the same split the stats
//! readout uses.

use crate::grid::Grid;
use crate::pose::{EyeMode, LegsMode, Pose};

pub(super) const DUR: i32 = 150;
const FADE: i32 = 14; // the wall rises, and later sinks
const WALK: i32 = 16; // he sets off towards it
const STEP: i32 = 2; // ticks per cell, so the walk never skips
const ARRIVE: i32 = WALK + SHIFT * STEP;
const GLOW: i32 = ARRIVE + 4;
const GLOW_TICKS: i32 = 10;
const BACK: i32 = 108; // he heads home
const HOME: i32 = BACK + SHIFT * STEP;
const OUT: i32 = 134;

/// How far right he walks — far enough to stand under the newest weeks.
const SHIFT: i32 = 10;

/// Columns and rows of the calendar — a GitHub year.
pub const WEEKS: usize = 53;
pub const DAYS: usize = 7;
/// How many days on the end keep the spotlight.
pub const RECENT: usize = 30;
/// The wall's band, in cells: `(x, y, w, h)`. It hangs above his head, clear of
/// him, and spans the full width so no cloud is left sliced in half.
pub const WALL: (i32, i32, i32, i32) = (0, 1, 32, 5);

/// How far up the wall is at tick `k`, 0–100. The renderer mixes both the
/// squares and the sky behind them by this, so the clouds sink away with it
/// instead of blinking out.
pub fn fade_pct(k: i32) -> u32 {
    match k {
        k if k < 0 => 0,
        k if k < FADE => (100 * k / FADE) as u32,
        k if k < OUT => 100,
        k if k < OUT + FADE => (100 * (OUT + FADE - k) / FADE) as u32,
        _ => 0,
    }
}

/// How far the spotlight on the last [`RECENT`] days has come up, 0–100. The
/// renderer fades the rest of the year back by the same amount. It only drops
/// once the wall has sunk, so nobody sees it go.
pub fn glow_pct(k: i32) -> u32 {
    match k {
        k if k < GLOW => 0,
        k if k < GLOW + GLOW_TICKS => (100 * (k - GLOW) / GLOW_TICKS) as u32,
        k if k < OUT + FADE => 100,
        _ => 0,
    }
}

/// How far right of home he has walked at tick `k`, in cells.
fn shift_at(k: i32) -> i32 {
    match k {
        k if k < WALK => 0,
        k if k < ARRIVE => (k - WALK) / STEP,
        k if k < BACK => SHIFT,
        k if k < HOME => SHIFT - (k - BACK) / STEP,
        _ => 0,
    }
}

pub(super) fn contributions(k: i32, _t: i32, _grid: &mut Grid) -> Pose {
    let walking = (WALK..ARRIVE).contains(&k) || (BACK..HOME).contains(&k);
    Pose {
        dx: shift_at(k),
        // No bob, however tempting. The wall's bottom row sits one cell above
        // his head, so lifting him even a little walks him into it.
        eyes: match k {
            k if k >= HOME => EyeMode::Auto,
            k if k >= BACK => EyeMode::Left, // heading home
            _ => EyeMode::Right,             // on his way to the newest weeks
        },
        legs: if walking {
            LegsMode::Walk
        } else {
            LegsMode::Still
        },
        ..Pose::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The scene has to open and close on an empty stage, and leave him back
    /// where it found him, or the reel's seam breaks and the act can't be
    /// reordered.
    #[test]
    fn it_opens_and_closes_on_an_empty_stage() {
        for k in [0, DUR] {
            assert_eq!(fade_pct(k), 0, "wall still up at k={k}");
            assert_eq!(glow_pct(k), 0, "spotlight still lit at k={k}");
            assert_eq!(shift_at(k), 0, "he's not home at k={k}");
        }
    }

    /// He must walk, never teleport: a jump of more than a cell reads as a
    /// stutter, which is exactly how the stats walk broke once.
    #[test]
    fn he_never_skips_a_cell() {
        for k in 0..DUR {
            let step = (shift_at(k + 1) - shift_at(k)).abs();
            assert!(step <= 1, "he jumped {step} cells at k={k}");
        }
    }

    /// The spotlight must wait for the wall to finish rising — lighting a month
    /// on a half-there calendar reads as a flicker. On the way out they sink
    /// together, since the renderer mixes both by [`fade_pct`].
    #[test]
    fn the_spotlight_waits_for_the_wall_to_rise() {
        for k in 0..OUT {
            if glow_pct(k) > 0 {
                assert_eq!(fade_pct(k), 100, "spotlight on a half-risen wall at k={k}");
            }
        }
    }
}
