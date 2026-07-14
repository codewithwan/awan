//! Behavioral tests — ported from the original Go engine's test suite, plus
//! coverage for the hatch intro, the bake skit, and the busy show.

use std::time::Duration;

use crate::character::{Character, MASCOT_W};
use crate::grid::{CANVAS_W, Grid};
use crate::scene::street::{BONK_TICK, CRATE_FLIGHT, crate_at};
use crate::scene::wander::{CONFUSED_TICKS, butterfly_at, pace_dx};
use crate::scene::{FULL_SHOW, bake, gem::gem_state, rocket, show_ticks};
use crate::stage::{Intro, MASCOT_HOME, Stage, WALK_IN_TICKS};

fn stage() -> Stage {
    Stage::show(Character::default())
}

#[test]
fn frame_has_blocks_and_shading() {
    let f = stage().frame(30, false);
    assert!(
        f.contains('█') && f.contains('▓') && f.contains('░'),
        "expected █/▓/░ pixels in the frame, got:\n{f}"
    );
}

#[test]
fn mascot_on_screen_through_every_scene() {
    let s = stage();
    let mut ticks: Vec<i32> = vec![5, WALK_IN_TICKS + show_ticks(FULL_SHOW) + 20];
    let mut acc = WALK_IN_TICKS;
    for sc in FULL_SHOW {
        ticks.push(acc + sc.dur / 2); // the middle of every scene
        acc += sc.dur;
    }
    for t in ticks {
        assert!(
            s.frame(t, false).contains("██"),
            "expected the mascot on screen at t={t}"
        );
    }
}

#[test]
fn scenes_differ() {
    let s = stage();
    let crate_scene = s.frame(WALK_IN_TICKS + 10, false);
    let sit_scene = s.frame(WALK_IN_TICKS + 50, false);
    assert_ne!(crate_scene, sit_scene);
}

#[test]
fn play_finite_terminates() {
    let mut buf = Vec::new();
    stage().play(&mut buf, false, 1, Duration::ZERO, None, &|| false);
    assert!(!buf.is_empty(), "expected animation output");
}

#[test]
fn crate_approaches_then_leaves() {
    let (x1, _, _, draw) = crate_at(4);
    assert!(draw, "crate should be approaching early in the scene");
    let (x2, _, _, _) = crate_at(12);
    assert!(x2 < x1, "crate should move toward the mascot");
    let (_, _, _, draw) = crate_at(BONK_TICK + CRATE_FLIGHT.len() as i32 + 2);
    assert!(!draw, "crate should be gone after its tumble");
}

#[test]
fn pacing_returns_home_and_stays_on_screen() {
    assert_eq!(
        pace_dx(CONFUSED_TICKS - 1),
        0,
        "pacing should end back home"
    );
    for k in 0..CONFUSED_TICKS {
        let dx = pace_dx(k);
        assert!(
            MASCOT_HOME + dx >= 0 && MASCOT_HOME + dx + MASCOT_W <= CANVAS_W,
            "pacing leaves the screen at k={k} (dx={dx})"
        );
    }
}

#[test]
fn butterfly_visits_then_leaves() {
    assert!(butterfly_at(5).2, "butterfly should be on screen early");
    let (x1, _, _) = butterfly_at(15);
    let (x2, _, _) = butterfly_at(19);
    assert!(x2 < x1, "butterfly should dart left mid-scene");
    assert!(!butterfly_at(40).2, "butterfly should be gone at the end");
}

#[test]
fn gem_falls_rests_then_vanishes() {
    let (y, draw) = gem_state(0);
    assert!(draw && y < 0, "gem should start above the screen");
    let (y, draw) = gem_state(20);
    assert!(draw && y == 9, "gem should rest on the ground mid-scene");
    assert!(!gem_state(40).1, "gem should vanish after its sparkle");
}

#[test]
fn rocket_builds_up_then_explodes() {
    assert!(rocket::build_stage(5) < rocket::build_stage(30));
    let charred = rocket::launch(25, 0, &mut Grid::new());
    assert!(
        charred.charred,
        "the buddy should be charred after the boom"
    );
    let recovered = rocket::launch(48, 0, &mut Grid::new());
    assert!(
        !recovered.charred,
        "the soot should be shaken off by the end"
    );
}

#[test]
fn hatch_starts_as_an_egg_and_ends_standing() {
    let s = stage().with_intro(Intro::Hatch);
    let egg = s.frame(2, false);
    let out = s.frame(36, false);
    assert_ne!(egg, out, "the egg and the hatched buddy should differ");
    assert!(
        egg.contains("▓▓"),
        "the egg shell should be on screen early"
    );
    assert!(out.contains("██"), "the buddy should be out near the end");
}

#[test]
fn bake_tells_a_full_story() {
    let s = stage();
    let bake_start = WALK_IN_TICKS + FULL_SHOW[..9].iter().map(|sc| sc.dur).sum::<i32>();
    let stirring = s.frame(bake_start + 50, false);
    let eating = s.frame(bake_start + 88, false);
    assert_ne!(stirring, eating, "stirring and eating should differ");

    let fetching = bake::bake(12, 0, &mut Grid::new());
    assert!(fetching.dx < -8, "he should scamper off to fetch the oven");
    let munching = bake::bake(92, 0, &mut Grid::new());
    assert!(munching.mouth_open, "he should be mid-bite at k=92");
    let full = bake::bake(106, 0, &mut Grid::new());
    assert_eq!(
        full.legs,
        crate::pose::LegsMode::Sit,
        "he should plop down, stuffed, after the cake"
    );
    let boom = bake::bake(112, 0, &mut Grid::new());
    assert!(
        !boom.charred && boom.mouth_open,
        "the oven blows up but he never gets charred"
    );
}

#[test]
fn busy_show_loops_and_captions() {
    let s = Stage::busy(Character::default());
    let mut buf = Vec::new();
    s.play(
        &mut buf,
        false,
        1,
        Duration::ZERO,
        Some("compiling"),
        &|| false,
    );
    let out = String::from_utf8(buf).unwrap();
    assert!(out.contains("compiling"), "caption should be rendered");
    assert!(out.contains("██"), "the buddy should be working");
}

#[test]
fn characters_recolor_but_share_choreography() {
    let oyen_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../characters/oyen.toml");
    let spec = crate::spec::load(std::path::Path::new(oyen_path)).expect("oyen loads");
    let oyen = Character::from_spec(&spec).unwrap();
    let a = Stage::show(Character::default()).frame(100, false);
    let o = Stage::show(oyen).frame(100, false);
    // Same scene beats; different body art.
    assert_ne!(a, o);
}

#[test]
fn soccer_ball_rolls_in_juggles_then_is_booted_away() {
    use crate::scene::soccer::ball_at;
    assert!(ball_at(2).2, "ball rolls in early");
    assert!(ball_at(10).1 < 10, "ball bounces up while juggling");
    assert!(!ball_at(64).2, "ball booted off-screen by the end");
}

#[test]
fn scenes_narrate_themselves_with_captions() {
    let mut buf = Vec::new();
    stage().play(&mut buf, false, 1, Duration::ZERO, None, &|| false);
    let out = String::from_utf8(buf).unwrap();
    // Each scene shows its own cute line, read like the character talking.
    assert!(out.contains("Awan: juggle juggle~") && out.contains("Awan: cake time~"));
}
