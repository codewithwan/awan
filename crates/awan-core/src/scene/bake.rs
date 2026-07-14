//! The baking skit: stir the bowl, slide it into a little oven, wait through
//! the glow… DING — a cake, with steam. Also the heart of the "busy" show:
//! the buddy visibly making something.

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::{BOWL, CAKE, OVEN};

pub(crate) const OVEN_X: i32 = 24;
pub(crate) const DING_TICK: i32 = 30;

pub(super) fn bake(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Still,
        eyes: EyeMode::Right,
        ..Pose::default()
    };

    if k < 16 {
        // Stirring the batter: spoon swings over the bowl, flour puffs.
        blit(grid, BOWL, 22, 10, Role::Crate);
        if k % 4 < 2 {
            grid.set(24, 8, "▓▓", Role::Tool);
            grid.set(25, 9, "██", Role::Tool);
        } else {
            grid.set(26, 8, "▓▓", Role::Tool);
            grid.set(25, 9, "██", Role::Tool);
        }
        if k % 6 < 2 {
            grid.set(23, 7, "░░", Role::Dust); // flour puff
        }
        return p;
    }

    // Into the oven; the window glows while it bakes.
    blit(grid, OVEN, OVEN_X, 8, Role::Rocket);
    if k < 18 {
        grid.set(23, 11, "░░", Role::Dust); // shoved-in dust
        grid.set(30, 11, "░░", Role::Dust);
    }
    if k < DING_TICK {
        let glow = if (k / 2) % 2 == 0 {
            Role::Flame
        } else {
            Role::Spark
        };
        grid.set(25, 9, "██", glow);
        grid.set(28, 9, "██", glow);
        if k % 4 < 2 {
            grid.set(26, 6, "░░", Role::Dust); // oven smoke
        } else {
            grid.set(27, 7, "░░", Role::Dust);
        }
        if k == 24 || k == 25 {
            p.eyes = EyeMode::Closed; // patient blink
        }
        return p;
    }

    if k < DING_TICK + 2 {
        // DING!
        grid.set(25, 6, "░░", Role::Spark);
        grid.set(28, 6, "░░", Role::Spark);
        grid.set(26, 5, "▒▒", Role::Spark);
        p.mouth_open = true;
        return p;
    }

    // The cake, unveiled on top — steam curls up.
    blit(grid, CAKE, 25, 5, Role::Crate);
    if k % 4 < 2 {
        grid.set(26, 3, "░░", Role::Dust);
    } else {
        grid.set(27, 4, "░░", Role::Dust);
    }
    match k {
        34.. if k < 46 => p.eyes = EyeMode::Happy,
        _ => {}
    }
    if k == 38 || k == 39 {
        p.dy = -1; // proud hop
    }
    if (42..46).contains(&k) {
        p.dx = 1; // lean in for a sniff
    }
    if k >= 47 {
        p.legs = LegsMode::Walk; // strolls on, satisfied
    }
    p
}
