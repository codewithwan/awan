//! The campfire, built step by step: he drags the wood in, tosses a spark from
//! his hand in an arc, the fire catches and grows, then pops in a little burst.

use crate::grid::{CANVAS_W, Grid};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

const HOME: i32 = 11; // == MASCOT_HOME
const FX: i32 = HOME + 11; // the fire, clear of his body
const GY: i32 = 11; // == GROUND_Y

pub(super) const DUR: i32 = 90;
const FETCH: i32 = 22; // wood slides in
const THROW: i32 = 34; // spark thrown by here
const BLOW: i32 = DUR - 14; // it pops at the end

pub(super) fn campfire(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    // The logs slide in during the fetch, then rest until the blast.
    let logx = if k < FETCH {
        CANVAS_W - (CANVAS_W - FX) * k / FETCH
    } else {
        FX
    };
    if k < BLOW {
        grid.set(logx, GY, "▄▄", Role::Crate);
        grid.set(logx + 1, GY, "▄▄", Role::Crate);
    }

    let mut p = Pose {
        legs: LegsMode::Still,
        eyes: EyeMode::Right,
        ..Pose::default()
    };

    if k < FETCH {
        // hauling the wood over
        p.dx = -1;
    } else if k < THROW {
        // a spark arcs from his hand to the wood
        let (f, span) = (k - FETCH, THROW - FETCH);
        let sx = HOME + 9 + (FX - HOME - 9) * f / span;
        let sy = (7 + 3 * f / span - 24 * f * (span - f) / (span * span)).max(0);
        grid.set(sx, sy, "* ", Role::Spark);
        p.mouth_open = true; // "hup!"
    } else if k >= BLOW {
        // the pop
        if (k - BLOW) % 2 == 0 {
            for (dx, dy, g) in [(-1, -2, "▒▒"), (0, -3, "██"), (2, -2, "▒▒"), (1, -1, "░░")]
            {
                grid.set(FX + dx, GY - 1 + dy, g, Role::Bang);
            }
        }
        (p.eyes, p.mouth_open) = (EyeMode::Closed, true);
    } else {
        // the fire catches and grows, flickering
        let (t, span) = (k - THROW, BLOW - THROW);
        let h = (1 + t * 3 / span).min(4);
        for i in 0..h {
            let y = GY - 1 - i;
            let flick = if (k + i) % 2 == 0 { "██" } else { "▓▓" };
            grid.set(
                FX,
                y,
                flick,
                if i >= h - 1 { Role::Spark } else { Role::Flame },
            );
            if i < h - 1 {
                grid.set(FX + 1, y, "▓▓", Role::Flame);
            }
        }
        p.eyes = EyeMode::Happy;
    }
    p
}
