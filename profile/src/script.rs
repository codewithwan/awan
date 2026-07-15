//! The narration script. A profile is a list of scene beats (`act` + what he
//! `say`s), loaded from `awan.json` — so the reader controls the *order* and
//! the *words*, not just the text. Identity fields fill in `{placeholders}`.

use awan_core::{Act, Reel};

use crate::icons::{self, Icon};

/// Ticks each lyric line holds during a singing beat.
pub const LYRIC_HOLD: i32 = 30;

/// One story beat: which scene to play and what he narrates over it.
#[derive(Clone, serde::Deserialize)]
pub struct SceneSpec {
    pub act: String,
    #[serde(default)]
    pub say: String,
}

/// A profile, as loaded from `awan.json` (or built from flags).
#[derive(Default, serde::Deserialize)]
#[serde(default)]
pub struct Profile {
    pub handle: String,
    pub name: String,
    pub role: String,
    pub location: String,
    pub stack: String,
    pub streak: u32,
    pub song: String,
    pub artist: String,
    pub lyrics: Vec<String>,
    pub output: String,
    pub scenes: Vec<SceneSpec>,
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
        let (i, k) = reel.act_at(t)?;
        (self.story().get(i).map(|s| s.act.as_str()) == Some("sing")).then_some(k)
    }

    /// The bottom caption at tick `t` (used off the singing beats).
    pub fn line(&self, reel: &Reel, t: i32) -> Line {
        if reel.is_leaving(t) {
            return line(&icons::HEART, "thanks for stopping by ~");
        }
        let story = self.story();
        let i = reel.act_at(t).map_or(0, |(i, _)| i).min(story.len() - 1);
        line(icon_of(&story[i].act), &self.fill(&story[i].say))
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
            &self.handle
        } else {
            &self.name
        };
        s.replace("{name}", name)
            .replace("{role}", &self.role)
            .replace("{location}", &self.location)
            .replace("{stack}", &self.stack)
            .replace("{streak}", &self.streak.to_string())
            .replace("{handle}", &self.handle)
    }
}

fn act_of(name: &str) -> Act {
    match name {
        "wave" => Act::Wave,
        "stroll" => Act::Stroll,
        "rocket" => Act::RocketBuild,
        "launch" => Act::RocketLaunch,
        "bake" => Act::Bake,
        "sing" => Act::Sing,
        "campfire" => Act::Campfire,
        "sleep" => Act::Sleep,
        "dance" => Act::Dance,
        "soccer" => Act::Soccer,
        _ => Act::Present,
    }
}

fn icon_of(act: &str) -> &'static Icon {
    match act {
        "wave" => &icons::HEART,
        "stroll" => &icons::PIN,
        "rocket" => &icons::CODE,
        "launch" | "dance" | "soccer" => &icons::STAR,
        "bake" | "sleep" => &icons::HEART,
        "campfire" => &icons::FIRE,
        "present" => &icons::BRIEFCASE,
        _ => &icons::DIAMOND,
    }
}

fn default_story() -> Vec<SceneSpec> {
    [
        ("wave", "hi there! i'm {name}"),
        ("present", "{role}"),
        ("stroll", "based in {location}"),
        ("rocket", "i build things, with {stack}"),
        ("launch", "...then watch 'em take off!"),
        ("bake", "and when i'm hungry, i bake"),
        ("campfire", "{streak}-day streak, still going"),
        ("sing", ""),
        ("soccer", "then a bit of football"),
        ("sleep", "okay... nap time, zzz"),
    ]
    .iter()
    .map(|(a, s)| SceneSpec {
        act: a.to_string(),
        say: s.to_string(),
    })
    .collect()
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
