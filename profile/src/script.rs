//! The narration script. A profile is a list of scene beats (`act` + what he
//! `say`s), loaded from `awan.json` — so the reader controls the *order* and
//! the *words*, not just the text. Identity fields fill in `{placeholders}`.

use awan_core::{Act, Reel};

use crate::story::{act_of, default_story, icon_of};
use awan_core::icons::{self, Icon};

/// Ticks each lyric line holds during a singing beat.
pub const LYRIC_HOLD: i32 = 30;

/// One story beat: which scene to play and what he narrates over it.
#[derive(Clone, serde::Deserialize)]
pub struct SceneSpec {
    pub act: String,
    #[serde(default)]
    pub say: String,
    /// A second caption that takes the beat over partway through. The wall uses
    /// it for the month, so the words land on the same tick as the spotlight.
    #[serde(default)]
    pub then: String,
}

/// A profile, as loaded from `awan.json` (or built from flags).
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Profile {
    /// Your GitHub username — what he calls you by, and the account CI reads.
    ///
    /// Accepts `handle` too. That's what this was called first, and a rename
    /// that silently blanks somebody's config is not a rename, it's a trap:
    /// every field here is `#[serde(default)]`, so the old key wouldn't error,
    /// it would just quietly go missing.
    #[serde(alias = "handle")]
    pub username: String,
    /// Path to a character TOML spec. Empty = the built-in buddy.
    pub character: String,
    pub name: String,
    pub role: String,
    pub location: String,
    pub stack: String,
    pub streak: u32,
    pub song: String,
    pub artist: String,
    /// Up to five short labels for the `stats` act: the first three ride
    /// balloons, the last two sit in crates. Keep them short (≤ 10 chars).
    pub stats: Vec<String>,
    /// The contribution calendar for the `contributions` act: one character
    /// per day, `0`-`4` for GitHub's quartiles and `.` for a day the
    /// calendar doesn't cover. CI fills this in; nobody types it by hand.
    pub contributions: String,
    pub contrib_year: u32,
    pub contrib_recent: u32,
    pub lyrics: Vec<String>,
    pub output: String,
    pub scenes: Vec<SceneSpec>,
    /// Boxes for the standalone stats banner (`awan-profile stats`) — no
    /// character, just three metrics with their date context. CI fills these.
    pub stat_boxes: Vec<StatBox>,
}

/// One box on the stats banner: a headline number, what it counts, and the
/// span it covers. All three are plain strings so CI can format them (commas,
/// dates) and the renderer just lays them out.
#[derive(Clone, Default, serde::Deserialize)]
pub struct StatBox {
    /// The number, pre-formatted: `"1,247"`, `"12 days"`.
    pub value: String,
    /// What it is: `"ALL COMMITS"`, `"CURRENT STREAK"`.
    pub label: String,
    /// The date context: `"since 12 Mar 2021"`, `"3 Feb - 12 Mar 2024"`.
    pub note: String,
}

/// One narration line: an optional icon and its text.
pub struct Line {
    pub icon: Option<&'static Icon>,
    pub text: String,
}

impl Profile {
    /// The effective beats — the reader's, or a friendly default story.
    pub fn story(&self) -> Vec<SceneSpec> {
        if self.scenes.is_empty() {
            default_story()
        } else {
            self.scenes.clone()
        }
    }

    /// The reel acts for these beats.
    pub fn acts(&self) -> Vec<Act> {
        self.story().iter().map(|s| act_of(&s.act)).collect()
    }

    /// If the beat at tick `t` is a singing beat, its tick-within-scene.
    pub fn sing_at(&self, reel: &Reel, t: i32) -> Option<i32> {
        self.beat_at(reel, t, "sing")
    }

    /// If the beat at tick `t` is the stats parade, its tick-within-scene.
    pub fn stats_at(&self, reel: &Reel, t: i32) -> Option<i32> {
        self.beat_at(reel, t, "stats")
    }

    /// If the beat at tick `t` is the year wall, its tick-within-scene.
    pub fn contributions_at(&self, reel: &Reel, t: i32) -> Option<i32> {
        self.beat_at(reel, t, "contributions")
    }

    fn beat_at(&self, reel: &Reel, t: i32, act: &str) -> Option<i32> {
        let (i, k) = reel.act_at(t)?;
        (self.story().get(i).map(|s| s.act.as_str()) == Some(act)).then_some(k)
    }

    /// The bottom caption at tick `t` (used off the singing beats).
    pub fn line(&self, reel: &Reel, t: i32) -> Line {
        if reel.is_leaving(t) {
            return line(&icons::HEART, "thanks for stopping by ~");
        }
        let story = self.story();
        let i = reel.act_at(t).map_or(0, |(i, _)| i).min(story.len() - 1);
        let spec = &story[i];
        line(icon_of(&spec.act), &self.fill(self.said(reel, t, spec)))
    }

    /// A beat's caption — its `then` sentence once the wall's spotlight is up,
    /// so the month is named at the moment it lights, not a beat later.
    fn said<'a>(&self, reel: &Reel, t: i32, spec: &'a SceneSpec) -> &'a str {
        let lit = self
            .contributions_at(reel, t)
            .is_some_and(|k| awan_core::contributions::glow_pct(k) > 0);
        if lit && !spec.then.is_empty() {
            &spec.then
        } else {
            &spec.say
        }
    }

    /// One karaoke line at tick `k` of a singing beat: an intro that names the
    /// song, then the reader's lyrics, one line at a time.
    pub fn lyric(&self, k: i32) -> Line {
        let step = (k / LYRIC_HOLD) as usize;
        if step == 0 {
            let song = pick(&self.song, "an old favourite");
            let artist = pick(&self.artist, "someone great");
            return line(&icons::STAR, &format!("my fav song \"{song}\" - {artist}"));
        }
        if self.lyrics.is_empty() {
            return line(&icons::GLOBE, "la la la ~");
        }
        line(&icons::GLOBE, &self.lyrics[(step - 1) % self.lyrics.len()])
    }

    /// Substitute `{name} {role} {location} {stack} {streak} {handle}`.
    fn fill(&self, s: &str) -> String {
        let name = if self.name.is_empty() {
            &self.username
        } else {
            &self.name
        };
        s.replace("{name}", name)
            .replace("{role}", &self.role)
            .replace("{location}", &self.location)
            .replace("{stack}", &self.stack)
            .replace("{streak}", &self.streak.to_string())
            .replace("{username}", &self.username)
            // the old spelling still fills, for configs written before the rename
            .replace("{handle}", &self.username)
            .replace("{contrib_year}", &self.contrib_year.to_string())
            .replace("{contrib_recent}", &self.contrib_recent.to_string())
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
