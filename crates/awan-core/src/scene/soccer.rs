//! The soccer skit (World Cup fever): a ball rolls in, he juggles it side to
//! side — booting it foot to foot in crossing arcs over his head — then kicks
//! it skyward, it drops onto his head, he reels dizzy, and he boots it away.
//!
//! Every juggle arc peaks above his head (`y < 6`, clear of the body) and only
//! touches down beside his feet (`x` outside the body), so the mascot — drawn
//! on top of scene props — never hides the ball while it crosses.

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::BALL;

pub(crate) const SOCCER_TICKS: i32 = 66;
const HOME: i32 = 11; // == MASCOT_HOME
const XL: i32 = 9; // ball touches down by his left foot
const XR: i32 = 21; // …and by his right foot

/// One side-to-side juggle arc (4 ticks): a foot boots the ball up and over
/// his head to the other foot. Even arcs strike from the right, odd from the
/// left. Returns the ball offset `(x, y)`.
fn juggle(j: i32) -> (i32, i32) {
    const ARC: [(i32, i32); 4] = [(0, 10), (4, 5), (7, 1), (10, 5)];
    let (dx, y) = ARC[(j % 4) as usize];
    let x = if (j / 4) % 2 == 0 { XR - dx } else { XL + dx };
    (x, y)
}

/// The ball's path: roll in, five crossing juggle arcs, a big boot up, onto
/// the head, ricochet to his feet, then booted away. Returns `(x, y, draw)`.
pub(crate) fn ball_at(k: i32) -> (i32, i32, bool) {
    match k {
        ..8 => (29 - k, 10, true), // rolls in from the right to his right foot
        8..28 => {
            let (x, y) = juggle(k - 8); // five kicks, foot to foot
            (x, y, true)
        }
        28..34 => (XL + 2 * (k - 28), 10 - 3 * (k - 28), true), // big boot up off the top
        34..42 => {
            // down onto his head at k=38, then ricochets to his feet
            const FALL: [(i32, i32); 8] = [
                (18, -2),
                (17, 0),
                (16, 2),
                (15, 4),
                (15, 5),
                (18, 3),
                (21, 6),
                (21, 10),
            ];
            let (x, y) = FALL[(k - 34) as usize];
            (x, y, true)
        }
        42..54 => (XR, 10, true), // rests at his feet while he reels
        54..66 => {
            let f = k - 54;
            (XR + 2 * f, 10 - f, f < 6) // booted high and far, gone once off-edge
        }
        _ => (0, 0, false),
    }
}

pub(super) fn soccer(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Still,
        eyes: EyeMode::Right,
        ..Pose::default()
    };
    let (bx, by, draw) = ball_at(k);
    if draw {
        blit(grid, BALL, bx, by, Role::EyeWhite);
    }
    match k {
        ..8 => {} // watches it roll in
        8..28 => {
            // juggling: legs pumping, a hop and a lean into each kicking foot
            p.legs = LegsMode::Walk;
            if (k - 8) % 4 == 0 {
                p.dx = if ((k - 8) / 4) % 2 == 0 { 1 } else { -1 };
                p.dy = -1;
            }
            // eyes track the ball: up at the peak, else toward its side
            p.eyes = if by <= 2 {
                EyeMode::Happy
            } else if bx >= HOME + 4 {
                EyeMode::Right
            } else {
                EyeMode::Left
            };
        }
        28..34 => (p.dx, p.eyes, p.mouth_open) = (-1, EyeMode::Closed, true), // the big kick up
        34..42 => {
            p.eyes = EyeMode::Closed; // braces as it drops
            if k == 38 {
                p.mouth_open = true; // BONK
                grid.set(HOME + 3, 4, "░░", Role::Spark);
                grid.set(HOME + 5, 3, "▒▒", Role::Bang);
                grid.set(HOME + 6, 4, "░░", Role::Spark);
            }
        }
        42..54 => {
            // dizzy: wobbles, eyes rolling, stars circling overhead
            p.dx = if k % 2 == 0 { -1 } else { 1 };
            p.eyes = if k % 2 == 0 {
                EyeMode::Left
            } else {
                EyeMode::Right
            };
            const ORBIT: [(i32, i32); 4] = [(2, 3), (7, 2), (5, 4), (3, 3)];
            let (ox, oy) = ORBIT[((k - 42) % 4) as usize];
            grid.set(HOME + ox, oy, "* ", Role::Spark);
        }
        _ => (p.dx, p.mouth_open) = (2, k < 58), // boots it away, then watches it go
    }
    p
}
