//! The campfire scene: he settles by a little fire that sparks to life, blazes
//! up and flickers while his streak glows, then dies back down to embers.

use crate::grid::Grid;
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

const HOME: i32 = 11; // == MASCOT_HOME
const FX: i32 = HOME + 11; // the fire, clear of his body (which spans ~11..20)
const GY: i32 = 11; // == GROUND_Y
const DUR: i32 = 90;

pub(super) fn campfire(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    // The last stretch: the fire pops in a little burst, then it's gone.
    if k >= DUR - 8 {
        let d = k - (DUR - 8);
        if d < 5 && d % 2 == 0 {
            for (dx, dy, g) in [(-1, -2, "▒▒"), (0, -3, "██"), (2, -2, "▒▒"), (1, 0, "░░")]
            {
                grid.set(FX + dx, GY - 1 + dy, g, Role::Bang);
            }
            grid.set(FX, GY - 4, "* ", Role::Spark);
        }
        return Pose {
            legs: LegsMode::Still,
            eyes: EyeMode::Closed,
            mouth_open: true, // startled by the pop
            ..Pose::default()
        };
    }

    // Two logs on the ground.
    grid.set(FX, GY, "▄▄", Role::Crate);
    grid.set(FX + 1, GY, "▄▄", Role::Crate);

    // Flame height rises to a peak at mid-scene, then falls back down.
    let h = (4 - (k - DUR / 2).abs() * 8 / DUR).max(0);
    for i in 0..h {
        let y = GY - 1 - i;
        let tip = i >= h - 1;
        let flick = if (k + i) % 2 == 0 { "██" } else { "▓▓" };
        grid.set(FX, y, flick, if tip { Role::Spark } else { Role::Flame });
        if i < h - 1 {
            grid.set(FX + 1, y, "▓▓", Role::Flame);
        }
    }
    if h >= 3 && k % 4 == 0 {
        grid.set(FX + 1, GY - h - 1, " *", Role::Spark);
    }

    Pose {
        legs: LegsMode::Still,
        eyes: EyeMode::Happy,
        ..Pose::default()
    }
}
