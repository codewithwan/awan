//! The narration script: a person's profile turned into a per-scene sequence of
//! captioned lines (icon + text). Each line is tied to a story beat so it lasts
//! the whole scene — no rushing — and the singing beat cycles the lyrics.

use awan_core::Reel;

use crate::icons::{self, Icon};

/// The Sing beat's index in the default story (see awan_core's `STORY`).
const SING_BEAT: usize = 8;
/// Ticks each lyric line holds during the singing beat.
const LYRIC_HOLD: i32 = 30;

/// A person's profile. Empty fields fall back to friendly defaults.
#[derive(Default)]
pub struct Profile {
    pub handle: String,
    pub name: String,
    pub role: String,
    pub location: String,
    pub stack: String,
    pub streak: u32,
    pub lyrics: Vec<String>,
}

/// One narration line: an optional icon and its text.
pub struct Line {
    pub icon: Option<&'static Icon>,
    pub text: String,
}

impl Profile {
    /// The line to show at tick `t` of the reel.
    pub fn line(&self, reel: &Reel, t: i32) -> Line {
        if reel.is_leaving(t) {
            return line(&icons::HEART, "thanks for stopping by ~");
        }
        match reel.act_at(t) {
            None => self.beat(0),
            Some((SING_BEAT, k)) => self.lyric(k),
            Some((i, _)) => self.beat(i),
        }
    }

    /// The narration for story beat `i`.
    fn beat(&self, i: usize) -> Line {
        let name = pick(&self.name, &self.handle);
        match i {
            0 => line(&icons::DIAMOND, &format!("hi there! i'm {name}")),
            1 => line(&icons::BRIEFCASE, pick(&self.role, "a developer")),
            2 => line(&icons::PIN, &located(&self.location)),
            3 => line(
                &icons::CODE,
                &format!("i build things, with {}", pick(&self.stack, "code")),
            ),
            4 => line(&icons::STAR, "…then watch 'em take off!"),
            5 => line(&icons::STAR, "always shipping something"),
            6 => line(&icons::HEART, "and when i'm hungry, i bake"),
            7 => line(&icons::HEART, "gotta refuel, right?"),
            9 => line(&icons::FIRE, &self.streak_line()),
            _ => line(&icons::GLOBE, &format!("@{}", self.handle)),
        }
    }

    /// During the singing beat: an intro, then the lyrics, line by line.
    fn lyric(&self, k: i32) -> Line {
        let step = (k / LYRIC_HOLD) as usize;
        if step == 0 || self.lyrics.is_empty() {
            return line(&icons::STAR, "i love music — my fav:");
        }
        line(&icons::GLOBE, &self.lyrics[(step - 1) % self.lyrics.len()])
    }

    fn streak_line(&self) -> String {
        if self.streak > 0 {
            format!("{}-day streak, still going", self.streak)
        } else {
            "still shipping, day after day".to_string()
        }
    }
}

fn located(loc: &str) -> String {
    if loc.is_empty() {
        "somewhere on earth".to_string()
    } else {
        format!("based in {loc}")
    }
}

fn pick<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if value.is_empty() { fallback } else { value }
}

fn line(icon: &'static Icon, text: &str) -> Line {
    Line {
        icon: Some(icon),
        text: text.to_string(),
    }
}
