//! Scenes — little skits composed into shows. Scenes marked `walking` scroll
//! the world past the buddy (treadmill); the rest freeze it while he acts.
//!
//! Scenes are pure functions of `(k, t)` so frames stay deterministic and
//! snapshot-testable. Tick counts and coordinates are tuned and verified
//! frame-by-frame — keep every constant identical when touching choreography.

pub(crate) mod bake;
pub(crate) mod dance;
pub(crate) mod gem;
pub(crate) mod hatch;
pub(crate) mod rocket;
pub(crate) mod soccer;
pub(crate) mod street;
pub(crate) mod wander;

use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{LegsMode, Pose};

pub(crate) type SceneFn = fn(k: i32, t: i32, grid: &mut Grid) -> Pose;

pub(crate) struct Scene {
    pub dur: i32,
    pub walking: bool,
    pub run: SceneFn,
}

const fn scene(dur: i32, walking: bool, run: SceneFn) -> Scene {
    Scene { dur, walking, run }
}

/// The full looping show.
pub(crate) static FULL_SHOW: &[Scene] = &[
    scene(36, true, street::crate_bonk), // crate rolls in → BONK → tumbles away
    scene(40, false, street::sit),       // plops down, dozes off zZ, wakes blinking
    scene(12, true, stroll),             // strolls on
    scene(wander::CONFUSED_TICKS, false, wander::confused), // "?" — paces back and forth
    scene(12, true, stroll),
    scene(46, false, wander::butterfly), // a butterfly teases him into a chase
    scene(12, true, stroll),
    scene(42, false, gem::fall), // a gem drops; freeze… blink-blink… sparkle
    scene(12, true, stroll),
    scene(bake::BAKE_TICKS, false, bake::bake), // fetches his oven, bakes a cake, eats it all
    scene(12, true, stroll),
    scene(40, false, rocket::build), // hammers a little rocket together
    scene(50, false, rocket::launch), // liftoff → BOOM → charred, blinking
    scene(12, true, stroll),
    scene(dance::DANCE_TICKS, false, dance::dance), // bounces to a silent beat
    scene(12, true, stroll),
    scene(soccer::SOCCER_TICKS, false, soccer::soccer), // juggles a ball, bonks his head
    scene(12, true, stroll),
];

/// The "working…" loop: just the making-things skits, for busy indicators.
pub(crate) static BUSY_SHOW: &[Scene] = &[
    scene(bake::BAKE_TICKS, false, bake::bake),
    scene(40, false, rocket::build),
];

pub(crate) fn show_ticks(show: &[Scene]) -> i32 {
    show.iter().map(|sc| sc.dur).sum()
}

pub(crate) fn show_walk_ticks(show: &[Scene]) -> i32 {
    show.iter().filter(|sc| sc.walking).map(|sc| sc.dur).sum()
}

/// Maps a tick within a show to its scene index, the tick inside that scene,
/// and how many walking ticks came before it (for ground scroll).
pub(crate) fn locate(show: &[Scene], tt: i32) -> (usize, i32, i32) {
    let mut acc = 0;
    let mut walk_before = 0;
    for (i, sc) in show.iter().enumerate() {
        if tt < acc + sc.dur {
            return (i, tt - acc, walk_before);
        }
        acc += sc.dur;
        if sc.walking {
            walk_before += sc.dur;
        }
    }
    (0, 0, 0)
}

fn stroll(_k: i32, _t: i32, _grid: &mut Grid) -> Pose {
    Pose {
        legs: LegsMode::Walk,
        ..Pose::default()
    }
}

/// Draw a sprite that fades out smoothly: solid until `fade`, then flickering
/// on alternating ticks for `span` ticks, then gone — so leftovers blink away
/// instead of popping out of existence.
pub(crate) fn blink_out<S: AsRef<str>>(
    grid: &mut Grid,
    sprite: &[S],
    at: (i32, i32),
    role: Role,
    k: i32,
    fade: i32,
    span: i32,
) {
    let d = k - fade;
    if d < 0 || (d < span && d % 2 == 0) {
        blit(grid, sprite, at.0, at.1, role);
    }
}
