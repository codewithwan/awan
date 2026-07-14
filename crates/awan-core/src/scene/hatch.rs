//! The hatch intro: the egg settles, hops, and cracks spread until — POP —
//! the top bursts off in a shower of sparks. The buddy wakes wearing a bit of
//! shell as a hat, looks around, watches the shell halves tumble away, shakes
//! the cap off, and hops for joy. The leftover shell bits blink away rather
//! than vanishing outright.

use super::blink_out;
use crate::character::Character;
use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::{EGG, SHELL_BOTTOM, SHELL_CAP, SHELL_LEFT, SHELL_RIGHT, SHELL_TOP};
use crate::stage::MASCOT_HOME;

pub(crate) const HATCH_TICKS: i32 = 56;
const POP: i32 = 22;

pub(crate) fn hatch_frame(k: i32, grid: &mut Grid, ch: &Character) {
    let home = MASCOT_HOME;
    if k < POP {
        egg_phase(k, grid, home);
        return;
    }

    let p = Pose {
        legs: match k {
            ..42 => LegsMode::Sit,
            54.. => LegsMode::Walk,
            _ => LegsMode::Still,
        },
        eyes: match k {
            ..26 => EyeMode::Closed,
            30..33 | 36..39 => EyeMode::Left, // looking around, then tracking the shells
            33..36 | 39..42 => EyeMode::Right,
            48..54 => EyeMode::Happy,
            _ => EyeMode::Auto,
        },
        dx: match k {
            44..48 if k % 2 == 0 => -1, // shaking the cap off
            44..48 => 1,
            _ => 0,
        },
        dy: if let 48 | 49 | 52 | 53 = k { -1 } else { 0 },
        mouth_open: k < 25,
        ..Pose::default()
    };
    blit(
        grid,
        &ch.mascot_rows(p, k),
        home + p.dx,
        6 + p.dy,
        Role::Body,
    );

    // The top shell sails up and away in a burst of sparks.
    if let 22..26 = k {
        const FLIGHT: [(i32, i32); 4] = [(0, 4), (1, 2), (2, 0), (3, -2)];
        let (fx, fy) = FLIGHT[(k - POP) as usize];
        blit(grid, SHELL_TOP, home + fx, fy, Role::EyeWhite);
    }
    if k < 26 {
        grid.set(home - 1, 5, "░░", Role::Spark);
        grid.set(home + 10, 5, "░░", Role::Spark);
        if k >= 23 {
            grid.set(home + 1, 3, "▒▒", Role::Spark);
            grid.set(home + 7, 3, "▒▒", Role::Spark);
        }
        if k >= 24 {
            grid.set(home + 4, 2, "░░", Role::Spark);
        }
    }
    shell_bottom(k, grid, home);
    cap(k, grid, home + p.dx);
}

/// The egg settles, hops, then cracks spread until it gives way.
fn egg_phase(k: i32, grid: &mut Grid, home: i32) {
    let hop = matches!(k, 8..14 if k % 3 == 0);
    let jx = if (14..POP).contains(&k) && k % 2 == 1 {
        1
    } else {
        0
    };
    blit(
        grid,
        EGG,
        home + jx,
        if hop { 5 } else { 6 },
        Role::EyeWhite,
    );
    if matches!(k, 8..14 if k % 3 == 1) {
        grid.set(home - 1, 11, "░░", Role::Dust);
        grid.set(home + 10, 11, "░░", Role::Dust);
    }
    const CRACKS: [(i32, i32); 5] = [(3, 8), (5, 9), (6, 8), (2, 9), (4, 7)];
    let n = if k < 14 {
        0
    } else {
        (((k - 10) / 2) as usize).min(5)
    };
    for &(cx, cy) in &CRACKS[..n] {
        grid.set(home + jx + cx, cy, "▓▓", Role::Eye);
    }
    if k >= 20 {
        grid.set(home + 2, 11, "░░", Role::EyeWhite);
        grid.set(home + 7, 11, "░░", Role::EyeWhite);
    }
}

/// The bottom shell: worn like a nest, split halves tumble to the edges, then
/// blink away.
fn shell_bottom(k: i32, grid: &mut Grid, home: i32) {
    match k {
        ..36 => blit(grid, SHELL_BOTTOM, home, 10, Role::EyeWhite),
        36..42 => {
            const ARC: [(i32, i32); 6] = [(2, -1), (4, -2), (6, -1), (8, 0), (10, 0), (11, 0)];
            let (off, dy) = ARC[(k - 36) as usize];
            blit(grid, SHELL_LEFT, home - off, 10 + dy, Role::EyeWhite);
            blit(grid, SHELL_RIGHT, home + 5 + off, 10 + dy, Role::EyeWhite);
        }
        _ => {
            blink_out(grid, SHELL_LEFT, (home - 11, 10), Role::EyeWhite, k, 48, 8);
            blink_out(grid, SHELL_RIGHT, (home + 16, 10), Role::EyeWhite, k, 48, 8);
        }
    }
}

/// The shell cap: worn since the pop, shaken off in an arc, then blinks away.
fn cap(k: i32, grid: &mut Grid, wx: i32) {
    match k {
        ..42 => blit(grid, SHELL_CAP, wx + 3, 5, Role::EyeWhite),
        42..47 => blit(grid, SHELL_CAP, wx + 3, 4, Role::EyeWhite),
        47..52 => {
            const ARC: [(i32, i32); 5] = [(5, 3), (7, 3), (9, 5), (11, 7), (12, 9)];
            let (fx, fy) = ARC[(k - 47) as usize];
            blit(grid, SHELL_CAP, MASCOT_HOME + fx, fy, Role::EyeWhite);
        }
        _ => {
            if k == 52 {
                grid.set(MASCOT_HOME + 11, 11, "░░", Role::Dust); // landing poof
            }
            blink_out(
                grid,
                SHELL_CAP,
                (MASCOT_HOME + 12, 10),
                Role::EyeWhite,
                k,
                52,
                4,
            );
        }
    }
}
