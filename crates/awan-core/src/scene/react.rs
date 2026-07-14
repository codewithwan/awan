//! One-shot reaction scenes, played when an event maps to one through a
//! character's `[reactions]` table (e.g. `cmd.failed = "charred"`). Each is a
//! short skit shown once, then the companion returns to what it was doing.

use super::{Scene, scene};
use crate::grid::{Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::HEART;

const HOME: i32 = 11; // == MASCOT_HOME

/// A happy hop with a heart floating up and sparkles at his sides.
fn celebrate(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        eyes: EyeMode::Happy,
        ..Pose::default()
    };
    if let 2 | 3 | 8 | 9 | 14 | 15 = k {
        p.dy = -1; // little hops
    }
    if k >= 4 {
        blit(grid, HEART, HOME + 3, 4 - (k.min(16) - 4) / 4, Role::Bang);
    }
    let spark = if k % 4 < 2 {
        (HOME - 1, 4)
    } else {
        (HOME + 10, 3)
    };
    grid.set(spark.0, spark.1, "░░", Role::Spark);
    p
}

/// Charred and blinking the soot off after something went wrong.
fn charred(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        charred: true,
        mouth_open: k < 6,
        ..Pose::default()
    };
    if let 6 | 7 | 12 | 13 | 18 | 19 = k {
        p.eyes = EyeMode::Closed; // blink it off
    }
    if k % 4 < 2 {
        grid.set(HOME + 2, 4, "░░", Role::Dust);
    } else {
        grid.set(HOME + 6, 3, "░░", Role::Dust);
    }
    p
}

/// A short nap: sits, dozes with drifting zZ, wakes near the end.
fn sleep(k: i32, _t: i32, grid: &mut Grid) -> Pose {
    let mut p = Pose {
        legs: LegsMode::Sit,
        eyes: EyeMode::Closed,
        ..Pose::default()
    };
    if k >= 6 {
        grid.set(HOME + 10, 5, "z ", Role::Dust);
    }
    if k >= 12 {
        grid.set(HOME + 12, 4, "z ", Role::Dust);
    }
    if k >= 18 {
        grid.set(HOME + 14, 3, "Z ", Role::Dust);
    }
    if k >= 34 {
        p.eyes = EyeMode::Auto; // waking, blinking
    }
    p
}

static CELEBRATE: [Scene; 1] = [scene(20, false, celebrate, "yay!")];
static CHARRED: [Scene; 1] = [scene(26, false, charred, "oof...")];
static SLEEP: [Scene; 1] = [scene(40, false, sleep, "zzz")];

/// The one-shot show for a reaction scene name, if it is a known one.
pub(crate) fn show_by_name(name: &str) -> Option<&'static [Scene]> {
    match name {
        "celebrate" => Some(&CELEBRATE),
        "charred" => Some(&CHARRED),
        "sleep" => Some(&SLEEP),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::charred;
    use crate::character::Character;
    use crate::grid::Grid;
    use crate::pose::{EyeMode, Pose};
    use crate::stage::Stage;

    #[test]
    fn reactions_dispatch_and_the_failure_scene_is_sooty() {
        assert!(Stage::react(Character::default(), "task.done").is_some());
        assert!(Stage::react(Character::default(), "nope").is_none());
        assert!(
            charred(4, 0, &mut Grid::new()).charred,
            "cmd.failed → charred"
        );
    }

    #[test]
    fn blink_rate_changes_the_blink_schedule() {
        let awan = Character::default(); // blink_rate 1.0 → period 26
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../characters/oyen.toml");
        let spec = crate::spec::load(std::path::Path::new(path)).unwrap();
        let oyen = Character::from_spec(&spec).unwrap(); // blink_rate 0.8 → period 33
        let p = Pose {
            eyes: EyeMode::Auto,
            ..Pose::default()
        };
        assert!(
            awan.mascot_rows(p, 26)[2].contains('-'),
            "awan blinks at 26"
        );
        assert!(
            !oyen.mascot_rows(p, 26)[2].contains('-'),
            "oyen doesn't at 26"
        );
        assert!(
            oyen.mascot_rows(p, 33)[2].contains('-'),
            "oyen blinks at 33"
        );
    }
}
