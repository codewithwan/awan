//! The baking story: an idea strikes mid-stroll, so the buddy pushes his oven
//! in from the left (bowl riding on top), stirs, bakes, and eats the cake bite
//! by bite. Then the overworked oven overheats and pops itself out of the
//! scene — a harmless bang, no soot. The heart of the "busy" show.

use super::blink_out;
use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::{BANG_SPRITE, BOWL, CAKE, HEART, OVEN};

pub(crate) const BAKE_TICKS: i32 = 118;
const OVEN_X: i32 = 26;
const EXPLODE: i32 = 110;

/// Where the oven is as it slides in from the left (early, so no empty screen).
fn oven_x(k: i32) -> i32 {
    (-8 + 2 * (k - 14)).min(OVEN_X)
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
    if (14..EXPLODE).contains(&k) {
        blit(grid, OVEN, ox, 8, Role::Rocket);
    }
    if (107..EXPLODE).contains(&k) {
        grid.set(27, 9, "██", Role::Flame); // overheating — glows an anxious hot
        grid.set(30, 9, "██", Role::Flame);
    }
    match k {
        14..40 => blit(grid, BOWL, ox, 6, Role::Crate), // supplies ride on top
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
        6..14 => (p.dx, p.legs, p.eyes) = (-2 * (k - 6), LegsMode::Walk, EyeMode::Left),
        14..35 => {
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
        40..44 => {} // stands watching the bowl hop down to the mixing spot
        44..58 => stir(k, grid, &mut p),
        58..62 => p.dx = 1, // pours the batter in
        62..76 => wait_for_bake(k, grid, &mut p),
        76..78 => {
            grid.set(27, 6, "░░", Role::Spark); // DING!
            grid.set(30, 6, "░░", Role::Spark);
            grid.set(28, 5, "▒▒", Role::Spark);
            p.mouth_open = true;
        }
        78..98 => feast(k, grid, &mut p),
        98..EXPLODE => satisfied(k, grid, &mut p),
        // explode only past EXPLODE; unhandled beats stay put (never fly off).
        _ if k >= EXPLODE => explode(k, grid, &mut p),
        _ => {}
    }
    p.dx = p.dx.max(-11); // the fetch never carries him fully off-screen
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

/// The cake pops out, glides down, and is eaten bite by bite.
fn feast(k: i32, grid: &mut Grid, p: &mut Pose) {
    match k {
        78..82 => {
            blit(grid, CAKE, 27, 5, Role::Crate);
            let (sx, sy) = if k % 4 < 2 { (28, 3) } else { (29, 4) };
            grid.set(sx, sy, "░░", Role::Dust); // steam
        }
        82..90 => {
            let d = k - 82; // glides down one pixel at a time
            blit(grid, CAKE, (27 - d).max(21), 5 + (d + 1) / 2, Role::Crate);
        }
        _ => {
            let n = (k < 94) as usize + (k < 96) as usize + 1; // shrinks 3→2→1
            blit(grid, &CAKE[3 - n..], 21, 9 + (3 - n) as i32, Role::Crate);
            p.dx = 1;
            p.mouth_open = (k / 2) % 2 == 0;
            if k >= 94 {
                grid.set(22, 11, "░░", Role::Crate); // crumbs under the cake
            }
            if k >= 96 {
                grid.set(24, 11, "░░", Role::Crate);
            }
        }
    }
}

/// Licks the crumbs clean, plops down with a heart, then eyes the hot oven.
fn satisfied(k: i32, grid: &mut Grid, p: &mut Pose) {
    blink_out(grid, &["-"], (22, 11), Role::Crate, k, 98, 6);
    blink_out(grid, &["-"], (24, 11), Role::Crate, k, 100, 6);
    if k < 104 {
        p.dx = 1; // leans in to lick them up
        p.mouth_open = (k / 2) % 2 == 0;
        return;
    }
    if k < 106 {
        grid.set(10, 11, "░░", Role::Dust); // contented plop
        grid.set(21, 11, "░░", Role::Dust);
    }
    (p.legs, p.eyes) = (LegsMode::Sit, EyeMode::Happy);
    if (104..107).contains(&k) {
        blit(grid, HEART, 22, 7 - (k - 104), Role::Bang);
    }
    if k >= 107 {
        p.eyes = EyeMode::Right; // the oven?!
        p.mouth_open = k >= 108;
    }
}

/// The oven blows itself up — a harmless pop; he flinches but never chars.
fn explode(k: i32, grid: &mut Grid, p: &mut Pose) {
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
