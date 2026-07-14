//! The gem drop: freeze… blink-blink… lean in… sparkle, gone.

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::GEM_SPRITE;

/// Maps a scene tick to the falling gem: drop, land, bounce, rest.
pub(crate) fn gem_state(k: i32) -> (i32, bool) {
    match k {
        ..6 => (-3 + 2 * k, true),
        7 => (8, true), // little bounce
        6 | 8..38 => (9, true),
        _ => (0, false),
    }
}

pub(super) fn fall(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Walk,
        ..Pose::default()
    };

    let (y, draw) = gem_state(k);
    if draw {
        let flicker = k >= 32 && k % 2 == 1; // sparkle-fade before vanishing
        if !flicker {
            blit(grid, GEM_SPRITE, 23, y, Role::Gem);
        }
        if k < 6 && y > 0 {
            grid.set(24, y - 1, "░░", Role::Gem); // falling trail
        }
    }
    if k == 6 || k == 7 {
        // landing puff
        grid.set(22, 11, "░░", Role::Dust);
        grid.set(26, 11, "░░", Role::Dust);
    }
    if (32..38).contains(&k) {
        // sparkles
        grid.set(24, 7, "░░", Role::Spark);
        grid.set(22, 9, "░░", Role::Spark);
        grid.set(26, 8, "▒▒", Role::Spark);
    }

    match k {
        ..4 => {} // hasn't noticed yet
        4..38 => {
            (p.legs, p.eyes) = (LegsMode::Still, EyeMode::Right); // freeze, stare
            if let 12 | 13 | 16 | 17 | 22 | 23 = k {
                p.eyes = EyeMode::Closed; // blink… blink-blink
            }
            if (26..32).contains(&k) {
                p.dx = 1; // lean in
            }
        }
        38..42 => {
            p.eyes = EyeMode::Happy;
            p.legs = LegsMode::Still;
            if k < 40 {
                p.dy = -1; // delighted hop
            }
        }
        _ => {}
    }
    p
}
