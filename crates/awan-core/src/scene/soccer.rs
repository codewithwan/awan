//! The soccer skit (World Cup fever): a ball rolls in, he juggles it five
//! times, boots it skyward, it drops onto his head, he reels dizzy — then he
//! boots it high and far, off the screen.
//!
//! The ball lives to his right (open space) so the mascot, drawn on top of
//! scene props, never hides it; it only touches his head from just above.

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::BALL;

pub(crate) const SOCCER_TICKS: i32 = 66;
const HOME: i32 = 11; // == MASCOT_HOME
const JUGGLE_X: i32 = 22; // ball hovers just right of his body

/// The ball's path: roll in, five keepie-uppies, up, onto the head, ricochet
/// to his feet, then booted away. Returns `(x, y, draw)`.
pub(crate) fn ball_at(k: i32) -> (i32, i32, bool) {
    match k {
        ..8 => (28 - 12 * k / 8, 10, true), // rolls in from the right
        8..28 => {
            const BOUNCE: [i32; 4] = [10, 7, 6, 7]; // one keepie-uppie
            (JUGGLE_X, BOUNCE[((k - 8) % 4) as usize], true)
        }
        28..34 => (JUGGLE_X, 10 - 3 * (k - 28), true), // booted up and off the top
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
                (22, 10),
            ];
            let (x, y) = FALL[(k - 34) as usize];
            (x, y, true)
        }
        42..54 => (22, 10, true), // rests at his feet while he reels
        54..66 => {
            let f = k - 54;
            (22 + 2 * f, 10 - f, f < 6) // booted high and far, gone once off-edge
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
            // keepie-uppies: taps and hops, eyes on the ball
            (p.legs, p.eyes) = (LegsMode::Walk, EyeMode::Happy);
            if (k - 8) % 4 == 0 {
                p.dy = -1; // a little hop on each touch
            }
        }
        28..34 => (p.dx, p.eyes, p.mouth_open) = (1, EyeMode::Closed, true), // the big kick up
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
            grid.set(HOME + ox, oy, "✦ ", Role::Spark);
        }
        _ => (p.dx, p.mouth_open) = (2, k < 58), // boots it away, then watches it go
    }
    p
}
