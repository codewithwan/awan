//! The act vocabulary: the names a reader writes in `awan.json`, mapped to the
//! engine's acts and to the little icon each beat's caption carries. An unknown
//! name falls back to `present` rather than failing — a typo shouldn't cost you
//! the whole GIF.

use awan_core::Act;

use crate::script::SceneSpec;
use awan_core::icons::{self, Icon};

pub fn act_of(name: &str) -> Act {
    match name {
        "wave" => Act::Wave,
        "stroll" => Act::Stroll,
        "rocket" => Act::RocketBuild,
        "launch" => Act::RocketLaunch,
        "bake" => Act::Bake,
        "sing" => Act::Sing,
        "campfire" => Act::Campfire,
        "stats" => Act::Stats,
        "contributions" => Act::Contributions,
        "sleep" => Act::Sleep,
        "dance" => Act::Dance,
        "soccer" => Act::Soccer,
        _ => Act::Present,
    }
}

pub fn icon_of(act: &str) -> &'static Icon {
    match act {
        "wave" => &icons::HEART,
        "stroll" => &icons::PIN,
        "rocket" => &icons::CODE,
        "launch" | "dance" | "soccer" => &icons::STAR,
        "bake" | "sleep" => &icons::HEART,
        "campfire" => &icons::FIRE,
        "stats" => &icons::DIAMOND,
        "contributions" => &icons::CODE,
        "present" => &icons::BRIEFCASE,
        _ => &icons::DIAMOND,
    }
}

pub fn default_story() -> Vec<SceneSpec> {
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
        then: String::new(),
    })
    .collect()
}
