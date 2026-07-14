//! The baking story: an idea strikes mid-stroll, so the buddy scampers off
//! and pushes his little oven in from the left — bowl riding on top — stirs
//! the batter, pours it in, waits through the glow (and can't help sitting
//! down)… DING! The cake hops down and gets eaten bite by bite. Then the
//! overworked oven overheats and blows itself up — a harmless pop, no soot —
//! leaving him to stroll on. The heart of the "busy" show.

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::{BANG_SPRITE, BOWL, CAKE, HEART, OVEN};

pub(crate) const BAKE_TICKS: i32 = 116;
const OVEN_X: i32 = 26;
const EXPLODE: i32 = 106;

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
    if (18..EXPLODE).contains(&k) {
        blit(grid, OVEN, ox, 8, Role::Rocket);
    }
    if (103..EXPLODE).contains(&k) {
        grid.set(27, 9, "██", Role::Flame); // overheating — glows an anxious hot
        grid.set(30, 9, "██", Role::Flame);
    }
    match k {
        18..40 => blit(grid, BOWL, ox, 6, Role::Crate), // supplies ride on top
        40..44 => {
            const HOP: [(i32, i32); 4] = [(25, 7), (23, 8), (21, 10), (21, 10)];
            let (x, y) = HOP[(k - 40) as usize];
            blit(grid, BOWL, x, y, Role::Crate);
        }
        44..58 => blit(grid, BOWL, 21, 10, Role::Crate),
        _ => {}
    }

    match k {
        ..2 => p.legs = LegsMode::Walk,
        2..6 => {
            blit(grid, BANG_SPRITE, 14, 1, Role::Spark); // an idea strikes!
            p.mouth_open = k >= 4;
        }
        6..18 => (p.dx, p.legs, p.eyes) = (-2 * (k - 6), LegsMode::Walk, EyeMode::Left),
        18..35 => {
            (p.dx, p.legs) = (ox - 21, LegsMode::Walk); // pushes it in, bowl and all
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
        44..58 => stir(k, grid, &mut p),
        58..62 => p.dx = 1, // pours the batter in
        62..76 => wait_for_bake(k, grid, &mut p),
        76..78 => {
            grid.set(27, 6, "░░", Role::Spark); // DING!
            grid.set(30, 6, "░░", Role::Spark);
            grid.set(28, 5, "▒▒", Role::Spark);
            p.mouth_open = true;
        }
        78..96 => feast(k, grid, &mut p),
        96..EXPLODE => satisfied(k, grid, &mut p),
        _ => explode(k, grid, &mut p),
    }
    p
}

fn stir(k: i32, grid: &mut Grid, p: &mut Pose) {
    grid.set(if k % 4 < 2 { 22 } else { 24 }, 8, "▓▓", Role::Tool);
    grid.set(23, 9, "██", Role::Tool);
    if k % 6 < 2 {
        grid.set(22, 7, "░░", Role::Dust); // flour puff
    }
    if k == 50 || k == 51 {
        p.eyes = EyeMode::Closed;
    }
}

fn wait_for_bake(k: i32, grid: &mut Grid, p: &mut Pose) {
    let glow = [Role::Flame, Role::Spark][((k / 2) % 2) as usize];
    grid.set(27, 9, "██", glow);
    grid.set(30, 9, "██", glow);
    let (sx, sy) = if k % 4 < 2 { (28, 6) } else { (29, 7) };
    grid.set(sx, sy, "░░", Role::Dust);
    if (66..74).contains(&k) {
        p.legs = LegsMode::Sit; // can't help sitting down to wait
        if k == 70 || k == 71 {
            p.eyes = EyeMode::Closed;
        }
    }
}

/// The cake pops out on top, hops down, and gets eaten bite by bite.
fn feast(k: i32, grid: &mut Grid, p: &mut Pose) {
    match k {
        78..82 => {
            blit(grid, CAKE, 27, 5, Role::Crate);
            let (sx, sy) = if k % 4 < 2 { (28, 3) } else { (29, 4) };
            grid.set(sx, sy, "░░", Role::Dust); // steam
        }
        82..86 => {
            const HOP: [(i32, i32); 4] = [(27, 5), (25, 6), (23, 8), (21, 9)];
            let (x, y) = HOP[(k - 82) as usize];
            blit(grid, CAKE, x, y, Role::Crate);
        }
        _ => {
            // munch — the cake shrinks 3 → 2 → 1 rows as he eats
            let n = (k < 90) as usize + (k < 93) as usize + 1;
            blit(grid, &CAKE[3 - n..], 21, 9 + (3 - n) as i32, Role::Crate);
            p.dx = 1;
            p.mouth_open = (k / 2) % 2 == 0;
            if k >= 90 {
                grid.set(22, 11, "░░", Role::Crate); // crumbs under the cake
            }
            if k >= 93 {
                grid.set(24, 11, "░░", Role::Crate);
            }
        }
    }
}

/// Stuffed and delighted: plops down, a heart floats up — until the oven
/// starts to overheat and he notices.
fn satisfied(k: i32, grid: &mut Grid, p: &mut Pose) {
    grid.set(22, 11, "░░", Role::Crate);
    grid.set(24, 11, "░░", Role::Crate);
    if k < 98 {
        grid.set(10, 11, "░░", Role::Dust); // contented plop
        grid.set(21, 11, "░░", Role::Dust);
    }
    (p.legs, p.eyes) = (LegsMode::Sit, EyeMode::Happy);
    if (98..104).contains(&k) {
        blit(grid, HEART, 22, 6 - (k - 98) / 2, Role::Bang);
    }
    if k >= 103 {
        p.eyes = EyeMode::Right; // uh — the oven?
        p.mouth_open = k >= 104;
    }
}

/// The oven overheats and blows itself up — a harmless pop. He flinches back,
/// but never gets charred; the smoke clears and he strolls on.
fn explode(k: i32, grid: &mut Grid, p: &mut Pose) {
    grid.set(22, 11, "░░", Role::Crate); // crumbs linger
    grid.set(24, 11, "░░", Role::Crate);
    match k - EXPLODE {
        0..3 => {
            oven_blast(grid, k - EXPLODE);
            (p.dx, p.mouth_open) = (-2, true);
            if k == EXPLODE {
                p.eyes = EyeMode::Closed; // flinch at the flash
            }
        }
        3..8 => {
            p.dx = -2;
            if k % 2 == 1 {
                grid.set(28, 9 - (k - EXPLODE), "░░", Role::Dust); // smoke clears
            }
        }
        _ => (p.dx, p.eyes) = ((-2 + (k - EXPLODE - 8)).min(0), EyeMode::Auto),
    }
}

// A three-frame fireball where the oven stood (x≈26–31, y≈8–11); '#'→██,
// '+'→▓▓, '-'→░░, with a couple of accent cells for colour.
fn oven_blast(grid: &mut Grid, f: i32) {
    match f {
        0 => {
            blit(grid, &["-##-", "####", "+##+"], 26, 8, Role::Flame);
            grid.set(27, 8, "▒▒", Role::Bang);
            grid.set(30, 8, "▒▒", Role::Spark);
        }
        1 => {
            blit(grid, &["-  -", "+##+", " -- "], 25, 7, Role::Bang);
            grid.set(28, 9, "██", Role::Flame);
            grid.set(31, 8, "░░", Role::Spark);
        }
        _ => blit(grid, &["-  -", "  - ", "-   "], 26, 6, Role::Dust),
    }
}
