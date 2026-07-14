//! Wandering skits: confused pacing and the butterfly chase.

use std::sync::LazyLock;

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::{BUTTERFLY_FRAMES, QMARK_SPRITE};
use crate::stage::MASCOT_HOME;

pub(crate) const CONFUSED_TICKS: i32 = 44;

struct PaceTables {
    dx: [i32; CONFUSED_TICKS as usize],
    dir: [i32; CONFUSED_TICKS as usize],
}

/// Pacing keyframes: pause, walk right, pause (turn), walk left past home,
/// pause (turn), walk back. Net displacement is zero.
static PACE: LazyLock<PaceTables> = LazyLock::new(|| {
    let segs: [(i32, i32); 6] = [(6, 0), (8, 1), (6, 0), (14, -1), (4, 0), (6, 1)];
    let mut tables = PaceTables {
        dx: [0; CONFUSED_TICKS as usize],
        dir: [0; CONFUSED_TICKS as usize],
    };
    let (mut x, mut i) = (0, 0);
    for (si, &(dur, mv)) in segs.iter().enumerate() {
        for j in 0..dur {
            let mut dir = mv;
            if dir == 0 {
                for &(_, next_mv) in &segs[si + 1..] {
                    if next_mv != 0 {
                        dir = next_mv;
                        break;
                    }
                }
            }
            if mv != 0 && j % 2 == 1 {
                x += mv;
            }
            tables.dx[i] = x;
            tables.dir[i] = dir;
            i += 1;
        }
    }
    tables
});

pub(crate) fn pace_dx(k: i32) -> i32 {
    PACE.dx[k as usize]
}

fn pace_dir(k: i32) -> i32 {
    PACE.dir[k as usize]
}

pub(super) fn confused(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        dx: pace_dx(k),
        legs: LegsMode::Still,
        ..Pose::default()
    };
    let moving = (k > 0 && pace_dx(k) != pace_dx(k - 1))
        || (k + 1 < CONFUSED_TICKS && pace_dx(k + 1) != pace_dx(k));
    if moving {
        p.legs = LegsMode::Walk;
    }
    p.eyes = if pace_dir(k) < 0 {
        EyeMode::Left
    } else {
        EyeMode::Right
    };
    if k < 6 || (14..20).contains(&k) {
        blit(grid, QMARK_SPRITE, MASCOT_HOME + p.dx + 8, 0, Role::Spark);
    }
    p
}

/// Maps a scene tick to the butterfly's fluttering path: in from the right,
/// a teasing hover, a dart left, a swing back overhead, and out.
pub(crate) fn butterfly_at(k: i32) -> (i32, i32, bool) {
    match k {
        ..8 => (31 - k, 4 - k % 2, true),
        8..14 => {
            const HOVER: [(i32, i32); 6] = [(23, 3), (22, 2), (21, 2), (21, 3), (22, 3), (23, 2)];
            let (x, y) = HOVER[(k - 8) as usize];
            (x, y, true)
        }
        14..21 => (20 - 2 * (k - 13), 3 - k % 2, true),
        21..30 => (6 + 2 * (k - 20), 1 + k % 2, true),
        30..36 => (24 + 2 * (k - 29), 2, true),
        _ => (0, 0, false),
    }
}

pub(super) fn butterfly(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let (bx, by, draw) = butterfly_at(k);
    if draw {
        blit(
            grid,
            BUTTERFLY_FRAMES[((k / 2) % 2) as usize],
            bx,
            by,
            Role::Butterfly,
        );
    }

    let mut p = Pose {
        legs: LegsMode::Walk,
        ..Pose::default()
    };
    match k {
        ..6 => {} // strolling, hasn't noticed yet
        6..14 => {
            // it hovers around his head — eyes track it
            (p.legs, p.eyes) = (LegsMode::Still, EyeMode::Right);
            if draw && bx < MASCOT_HOME + 5 {
                p.eyes = EyeMode::Left;
            }
        }
        14..16 => (p.legs, p.eyes) = (LegsMode::Still, EyeMode::Left), // it darted left!
        16..22 => (p.dx, p.eyes) = (-(k - 15), EyeMode::Left),         // chase it left
        22..24 => (p.dx, p.eyes) = (-6, EyeMode::Right),               // it swings back overhead
        24..26 => (p.dx, p.dy, p.eyes) = (-6, -1, EyeMode::Right),     // jump for it!
        26..35 => {
            p.dx = (-6 + (k - 25)).min(3); // chase it right
            p.eyes = EyeMode::Right;
        }
        35..40 => (p.dx, p.legs, p.eyes) = (3, LegsMode::Still, EyeMode::Right), // it gets away
        _ => {
            p.dx = (3 - (k - 39)).max(0); // trots back home
            p.eyes = if k >= 44 {
                EyeMode::Auto
            } else {
                EyeMode::Left
            };
        }
    }
    p
}
