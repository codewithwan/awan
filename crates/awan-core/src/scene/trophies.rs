//! The trophy shelf: he steps aside, a plank drops in beside him, and cups
//! land on it one at a time while he watches. Then it folds away and he strolls
//! back — the same shape as the stats readout, because it's the same problem:
//! something to show him, with nowhere to put it if he's standing in front.
//!
//! Trophies are *objects*, not cards. That's the whole reason this scene exists
//! in his grammar rather than beside it: the engine draws crates, ovens, cakes
//! and rockets, and a grid of boxes with letter-ranks in them is a language
//! from somebody else's app. He can't pick up a card.
//!
//! The engine builds the shelf and says which cups have landed ([`SHELF`],
//! [`landed`], [`stand_pct`]); the labels under them are the reader's numbers,
//! so the renderer owns those — the same split the readout and the wall use.

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

pub(super) const DUR: i32 = 128;
const DROP: i32 = 8; // the plank arrives
const SET: i32 = 16; // he sets off — from home, because every scene starts there
const STEP: i32 = 2; // ticks per cell — never skip, never stutter
const SHIFT: i32 = 10; // how far LEFT he steps, to leave the shelf room
const WALK_END: i32 = SET + SHIFT * STEP;
/// Ticks between one cup landing and the next.
const LAND: i32 = 12;
/// He's seen enough. The shelf folds *here* — before he walks back, because
/// home is underneath the labels and he'd walk straight through them.
const LOOK_END: i32 = SET + LAND * SLOTS as i32 + 12;
const FOLD: i32 = 6;
const HOME: i32 = LOOK_END + FOLD;
const HOME_END: i32 = HOME + SHIFT * STEP;

/// How many cups the shelf holds.
///
/// Four, and that's arithmetic rather than taste. He takes eleven of the
/// stage's thirty-two cells, leaving nineteen. A cup is three cells and needs
/// one of air, so four cups is seventeen — and each label gets 132px, which is
/// eight glyphs, which is "commits" with room to spare. Five cups fits nothing
/// legible at any spacing.
pub const SLOTS: usize = 4;
/// The shelf, in cells: `(x, y, w)` — beside him, not above. Labels need the
/// rows underneath, and his head was already using them.
pub const SHELF: (i32, i32, i32) = (13, 5, 17);
/// Cells between one cup and the next: three for the cup, one for air.
pub const PITCH: i32 = 4;

/// A little cup: bowl, stem, base. Three cells wide, because five of anything
/// six cells wide is thirty cells of a thirty-two cell stage — a gold fence,
/// not a shelf. The engine's props are this size for the same reason.
const CUP: &[&str] = &["###", "#-#", " # ", "###"];

/// How far right of home he is at tick `k`. He starts *at* home — a scene that
/// opens with him already somewhere else has teleported him, and the reel's
/// seam is the thing that notices.
fn shift_at(k: i32) -> i32 {
    match k {
        k if k < SET => 0,
        k if k < WALK_END => -((k - SET) / STEP),
        k if k < HOME => -SHIFT,
        k if k < HOME_END => -SHIFT + (k - HOME) / STEP,
        _ => 0,
    }
}

/// How many cups are on the shelf at tick `k` — they land one at a time while
/// he watches, rather than all at once, so there's something to watch.
pub fn landed(k: i32) -> usize {
    if k < SET {
        return 0;
    }
    let since = k - SET;
    ((since / LAND).clamp(0, SLOTS as i32)) as usize
}

/// How far the shelf itself has settled, 0–100 — the renderer fades the labels
/// with it so nothing is legible before there's anything to read.
pub fn stand_pct(k: i32) -> u32 {
    match k {
        k if k < DROP => 0,
        k if k < DROP + FOLD => (100 * (k - DROP) / FOLD) as u32,
        k if k < LOOK_END => 100,
        k if k < HOME => (100 * (HOME - k) / FOLD) as u32,
        _ => 0,
    }
}

pub(super) fn trophies(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let (sx, sy, sw) = SHELF;
    if stand_pct(k) > 0 {
        // the plank, then whatever he's got up so far
        for x in sx..sx + sw {
            grid.set(x, sy, "▓▓", Role::Crate);
        }
        for i in 0..landed(k) {
            blit(grid, CUP, sx + 1 + i as i32 * PITCH, sy - 4, Role::Spark);
        }
    }

    let walking = (SET..WALK_END).contains(&k) || (HOME..HOME_END).contains(&k);
    Pose {
        dx: shift_at(k),
        // he watches the cup he's carrying land, then looks back down the shelf
        eyes: match k {
            k if k >= HOME_END => EyeMode::Auto,
            k if k >= HOME => EyeMode::Left, // heading home
            _ => EyeMode::Right,             // the shelf is on his right
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

    /// Open and close on an empty stage, and put him back where he started, or
    /// the reel's seam breaks and the act can't be reordered.
    #[test]
    fn it_opens_and_closes_empty() {
        for k in [0, DUR] {
            assert_eq!(stand_pct(k), 0, "shelf still up at k={k}");
            assert_eq!(shift_at(k), 0, "he's not home at k={k}");
        }
    }

    /// He walks; he never teleports. Exactly the bug the stats act shipped once.
    #[test]
    fn he_never_skips_a_cell() {
        for k in 0..DUR {
            assert!(
                (shift_at(k + 1) - shift_at(k)).abs() <= 1,
                "jumped at k={k}"
            );
        }
    }

    /// Every cup must be up before the shelf starts folding away, or the last
    /// number is one nobody ever reads.
    #[test]
    fn every_cup_gets_its_moment() {
        assert_eq!(
            landed(LOOK_END),
            SLOTS,
            "still landing when the shelf folds"
        );
    }

    /// The shelf must be gone before he walks home — home is under the labels.
    #[test]
    fn the_shelf_clears_before_he_walks_back() {
        assert_eq!(stand_pct(HOME), 0, "shelf still up as he sets off home");
        for k in HOME..=DUR {
            assert_eq!(stand_pct(k), 0, "shelf came back at k={k}");
        }
    }

    /// Four cups at four cells apart must fit the shelf they sit on.
    #[test]
    fn the_cups_fit_the_shelf() {
        let (_, _, w) = SHELF;
        assert!(PITCH * (SLOTS as i32) < w, "cups overrun the plank");
    }
}
