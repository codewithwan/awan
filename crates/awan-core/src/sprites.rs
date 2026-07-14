//! World sprites — props the scenes blit around whichever character is on
//! stage. Glyph language: `#` solid `█` · `+` dense `▓` · `-` light `░` ·
//! `@` eye · `%` white eye · `^` happy eye `▀` · `_` dot `▄`.

/// The crate and its "spinning" tumble variant.
pub(crate) const CRATE_FRAMES: [&[&str]; 2] = [
    &["+####+", "#----#", "#----#", "+####+"],
    &[" #### ", "##--##", "##--##", " #### "],
];

pub(crate) const CLOUD_BIG: &[&str] = &[" -++++++- ", "-++++++++-"];

pub(crate) const CLOUD_SMALL: &[&str] = &[" -+++- ", "-+++++-"];

pub(crate) const GEM_SPRITE: &[&str] = &[" + ", "+#+", " + "];

pub(crate) const QMARK_SPRITE: &[&str] = &["+##+", "   #", "  # ", "    ", "  _ "];

pub(crate) const BANG_SPRITE: &[&str] = &["#", "#", " ", "_"];

/// Butterfly flutters between wings-up and wings-folded.
pub(crate) const BUTTERFLY_FRAMES: [&[&str]; 2] = [&["+ +", " # "], &["   ", "+#+"]];

/// The little rocket he builds and launches.
pub(crate) const ROCKET_ROWS: &[&str] = &[
    " ## ", "####", "#--#", // window
    "####", "+##+", // fins
];

/// The egg the buddy hatches from on first run (same footprint as the
/// mascot: 10 wide, 6 tall, sitting on the ground).
pub(crate) const EGG: &[&str] = &[
    "  +####+  ",
    " +######+ ",
    "+########+",
    "##########",
    "+########+",
    " +######+ ",
];

/// Top of the shell (pops off) and the split bottom halves — shallow enough
/// to leave the hatched buddy's face visible.
pub(crate) const SHELL_TOP: &[&str] = &["  +####+  ", " +######+ ", "+########+"];
pub(crate) const SHELL_BOTTOM: &[&str] = &["+########+", " +######+ "];
pub(crate) const SHELL_LEFT: &[&str] = &["+####", " +###"];
pub(crate) const SHELL_RIGHT: &[&str] = &["####+", "###+ "];

/// Baking props: mixing bowl, little oven, and the cake that pops out.
pub(crate) const BOWL: &[&str] = &["#---#", "+###+"];
pub(crate) const OVEN: &[&str] = &["+####+", "#-##-#", "#----#", "+####+"];
pub(crate) const CAKE: &[&str] = &["-##-", "####", "+##+"];

/// A little heart, floated up when the buddy is happy and full.
pub(crate) const HEART: &[&str] = &["# #", "###", " # "];

/// A handheld mic he holds up to his mouth: rounded grille head, short handle.
pub(crate) const MIC: &[&str] = &["-#", "##", "+ "];

/// A small soccer ball for the juggling skit: a 2×2 nub with a dark patch.
pub(crate) const BALL: &[&str] = &["+#", "#@"];

/// The classic gag: a bit of shell worn as a hat after hatching.
pub(crate) const SHELL_CAP: &[&str] = &[" ## ", "+##+"];
