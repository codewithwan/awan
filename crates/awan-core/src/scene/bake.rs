//! The baking story: an idea strikes mid-stroll, so the buddy scampers off
//! and pushes his little oven in from the left — bowl riding on top — stirs
//! the batter, pours it in, waits through the glow (and can't help sitting
//! down)… DING! The cake hops down, gets eaten bite by bite, and he plops
//! back, stuffed, with a little heart. The heart of the "busy" show.

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::{BANG_SPRITE, BOWL, CAKE, HEART, OVEN};

pub(crate) const BAKE_TICKS: i32 = 112;
const OVEN_X: i32 = 26;

/// Where the oven is while being pushed in from the left.
fn oven_x(k: i32) -> i32 {
    (-8 + 2 * (k - 18)).min(OVEN_X)
}

pub(crate) fn bake(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Still,
        eyes: EyeMode::Right,
        ..Pose::default()
    };
    let ox = oven_x(k);

    // The bowl slides *into* the oven: drawn first so the oven occludes it.
    if (58..62).contains(&k) {
        blit(grid, BOWL, 21 + 2 * (k - 58), 10, Role::Crate);
    }
    if k >= 18 {
        blit(grid, OVEN, ox, 8, Role::Rocket);
    }
    match k {
        18..40 => blit(grid, BOWL, ox, 6, Role::Crate), // supplies ride on top
        40..44 => {
            // the bowl hops down to the mixing spot
            const HOP: [(i32, i32); 4] = [(25, 7), (23, 8), (21, 10), (21, 10)];
            let (x, y) = HOP[(k - 40) as usize];
            blit(grid, BOWL, x, y, Role::Crate);
        }
        44..58 => blit(grid, BOWL, 21, 10, Role::Crate),
        _ => {}
    }

    match k {
        ..2 => p.legs = LegsMode::Walk, // strolling…
        2..6 => {
            // …an idea strikes!
            blit(grid, BANG_SPRITE, 14, 1, Role::Spark);
            p.mouth_open = k >= 4;
        }
        6..18 => {
            // scampers off to fetch the oven
            (p.dx, p.legs, p.eyes) = (-2 * (k - 6), LegsMode::Walk, EyeMode::Left);
        }
        18..35 => {
            // pushes it in, bowl and all
            (p.dx, p.legs) = (ox - 21, LegsMode::Walk);
            if k % 2 == 0 {
                grid.set(ox - 12, 11, "░░", Role::Dust);
            }
        }
        35..40 => {
            p.dx = 5 - (k - 35); // steps back to admire the setup
            if k < 38 {
                grid.set(25, 11, "░░", Role::Dust);
                grid.set(31, 11, "░░", Role::Dust);
            }
        }
        44..58 => {
            // stirring the batter
            if k % 4 < 2 {
                grid.set(22, 8, "▓▓", Role::Tool);
            } else {
                grid.set(24, 8, "▓▓", Role::Tool);
            }
            grid.set(23, 9, "██", Role::Tool);
            if k % 6 < 2 {
                grid.set(22, 7, "░░", Role::Dust); // flour puff
            }
            if k == 50 || k == 51 {
                p.eyes = EyeMode::Closed;
            }
        }
        58..62 => p.dx = 1, // pours the batter in
        62..76 => {
            // it bakes; he waits — and can't help sitting down
            let glow = if (k / 2) % 2 == 0 {
                Role::Flame
            } else {
                Role::Spark
            };
            grid.set(27, 9, "██", glow);
            grid.set(30, 9, "██", glow);
            if k % 4 < 2 {
                grid.set(28, 6, "░░", Role::Dust);
            } else {
                grid.set(29, 7, "░░", Role::Dust);
            }
            if (66..74).contains(&k) {
                p.legs = LegsMode::Sit;
                if k == 70 || k == 71 {
                    p.eyes = EyeMode::Closed; // patient blink
                }
            }
        }
        76..78 => {
            // DING!
            grid.set(27, 6, "░░", Role::Spark);
            grid.set(30, 6, "░░", Role::Spark);
            grid.set(28, 5, "▒▒", Role::Spark);
            p.mouth_open = true;
        }
        78..96 => feast(k, grid, &mut p),
        _ => satisfied(k, grid, &mut p),
    }
    p
}

/// The cake pops out on top, hops down, and gets eaten bite by bite.
fn feast(k: i32, grid: &mut Grid, p: &mut Pose) {
    match k {
        78..82 => {
            blit(grid, CAKE, 27, 5, Role::Crate);
            if k % 4 < 2 {
                grid.set(28, 3, "░░", Role::Dust); // steam
            } else {
                grid.set(29, 4, "░░", Role::Dust);
            }
        }
        82..86 => {
            const HOP: [(i32, i32); 4] = [(27, 5), (25, 6), (23, 8), (21, 9)];
            let (x, y) = HOP[(k - 82) as usize];
            blit(grid, CAKE, x, y, Role::Crate);
        }
        _ => {
            // munch, munch — the cake shrinks bite by bite
            let n: usize = match k {
                ..90 => 3,
                90..93 => 2,
                _ => 1,
            };
            blit(grid, &CAKE[3 - n..], 21, 9 + (3 - n) as i32, Role::Crate);
            p.dx = 1;
            p.mouth_open = (k / 2) % 2 == 0;
            if k >= 90 {
                grid.set(22, 11, "░░", Role::Crate); // crumbs, right under the cake
            }
            if k >= 93 {
                grid.set(24, 11, "░░", Role::Crate);
            }
        }
    }
}

/// Stuffed and delighted: plops down, a heart floats up.
fn satisfied(k: i32, grid: &mut Grid, p: &mut Pose) {
    grid.set(22, 11, "░░", Role::Crate); // leftover crumbs where the cake was
    grid.set(24, 11, "░░", Role::Crate);
    if k < 98 {
        grid.set(10, 11, "░░", Role::Dust); // contented plop
        grid.set(21, 11, "░░", Role::Dust);
    }
    if k < 108 {
        (p.legs, p.eyes) = (LegsMode::Sit, EyeMode::Happy);
    } else {
        p.eyes = EyeMode::Auto;
    }
    if (98..108).contains(&k) {
        blit(grid, HEART, 22, 6 - (k - 98) / 2, Role::Bang);
    }
    if k >= 110 {
        p.legs = LegsMode::Walk;
    }
}
