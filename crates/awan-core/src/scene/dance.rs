//! The dance scene: the buddy bounces and sways to a silent beat, tapping his
//! feet while music notes drift up around him.

use crate::grid::Grid;
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};

pub(crate) const DANCE_TICKS: i32 = 48;

/// One 8-tick bar of movement: `(dx, dy)`. Hops on the down-beats (0 and 4),
/// leans left then right in between.
const STEP: [(i32, i32); 8] = [
    (0, -1),
    (-1, 0),
    (-1, 0),
    (0, 0),
    (0, -1),
    (1, 0),
    (1, 0),
    (0, 0),
];

/// Notes drift up around him: `(x, role)`, each on its own phase.
const NOTES: [(i32, Role); 3] = [(8, Role::Spark), (22, Role::Gem), (15, Role::Butterfly)];

pub(super) fn dance(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let (dx, dy) = STEP[(k % 8) as usize];
    let mut p = Pose {
        dx,
        dy,
        eyes: EyeMode::Happy,
        legs: LegsMode::Walk, // tapping feet
        ..Pose::default()
    };
    if k % 8 == 0 || k % 8 == 4 {
        p.mouth_open = true; // "woo!"
    }
    for (i, &(nx, role)) in NOTES.iter().enumerate() {
        let phase = (k + i as i32 * 5) % 15;
        if phase < 10 {
            let ny = 6 - phase / 2; // rises from y6 up to y1
            let glyph = if phase % 4 < 2 { "♪ " } else { "♫ " };
            grid.set(nx, ny, glyph, role);
        }
    }
    p
}
