//! Scenes — little skits composed into shows. Scenes marked `walking` scroll
//! the world past the buddy (treadmill); the rest freeze it while he acts.
//!
//! Scenes are pure functions of `(k, t)` so frames stay deterministic and
//! snapshot-testable. Tick counts and coordinates are tuned and verified
//! frame-by-frame — keep every constant identical when touching choreography.

pub(crate) mod bake;
pub(crate) mod campfire;
pub(crate) mod contributions;
pub(crate) mod dance;
pub(crate) mod gem;
pub(crate) mod greet;
pub(crate) mod hatch;
pub(crate) mod react;
pub(crate) mod rocket;
pub(crate) mod sleep;
pub(crate) mod soccer;
pub(crate) mod song;
pub(crate) mod stats;
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
    /// A cute one-liner shown under the canvas, like he's narrating himself.
    pub cap: &'static str,
}

const fn scene(dur: i32, walking: bool, run: SceneFn, cap: &'static str) -> Scene {
    Scene {
        dur,
        walking,
        run,
        cap,
    }
}

/// The full looping show.
pub(crate) static FULL_SHOW: &[Scene] = &[
    scene(36, true, street::crate_bonk, "ooh, a box!"),
    scene(40, false, street::sit, "nap time~"),
    scene(12, true, stroll, "da-da-da~"),
    scene(
        wander::CONFUSED_TICKS,
        false,
        wander::confused,
        "hmm, where to?",
    ),
    scene(12, true, stroll, "da-da-da~"),
    scene(46, false, wander::butterfly, "a butterfly!!"),
    scene(12, true, stroll, "da-da-da~"),
    scene(42, false, gem::fall, "so shiny!!"),
    scene(12, true, stroll, "da-da-da~"),
    scene(bake::BAKE_TICKS, false, bake::bake, "cake time~"),
    scene(12, true, stroll, "da-da-da~"),
    scene(40, false, rocket::build, "building a rocket!"),
    scene(50, false, rocket::launch, "3.. 2.. 1..!"),
    scene(12, true, stroll, "da-da-da~"),
    scene(dance::DANCE_TICKS, false, dance::dance, "la la la~"),
    scene(12, true, stroll, "da-da-da~"),
    scene(
        soccer::SOCCER_TICKS,
        false,
        soccer::soccer,
        "juggle juggle~",
    ),
    scene(12, true, stroll, "da-da-da~"),
];

/// Build the scene for one reel act. Durations are tuned so each story beat has
/// room to breathe (and the whole reel runs a comfortable ~60s).
pub(crate) fn scene_for(act: crate::reel::Act) -> Scene {
    use crate::reel::Act::*;
    match act {
        Wave => scene(30, false, greet::wave, ""),
        Present => scene(60, false, greet::present, ""),
        Stroll => scene(30, true, stroll, ""),
        RocketBuild => scene(40, false, rocket::build, ""),
        RocketLaunch => scene(50, false, rocket::launch, ""),
        Bake => scene(bake::BAKE_TICKS, false, bake::bake, ""),
        Sing => scene(song::DUR, false, song::sing, ""),
        Campfire => scene(campfire::DUR, false, campfire::campfire, ""),
        Stats => scene(stats::DUR, false, stats::stats, ""),
        Contributions => scene(contributions::DUR, false, contributions::contributions, ""),
        Sleep => scene(sleep::DUR, false, sleep::sleep, ""),
        Dance => scene(dance::DANCE_TICKS, false, dance::dance, ""),
        Soccer => scene(soccer::SOCCER_TICKS, false, soccer::soccer, ""),
    }
}

/// The "working…" loop: just the making-things skits, for busy indicators.
pub(crate) static BUSY_SHOW: &[Scene] = &[
    scene(bake::BAKE_TICKS, false, bake::bake, "cake time~"),
    scene(40, false, rocket::build, "building a rocket!"),
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
