//! The awan profile generator (separate, opt-in — never shipped with the core
//! binary). Plays a seamless reel narrating a profile, and can write it to a
//! looping GIF for a README.
//!
//! - `awan-profile whoami --config awan.json` — the recommended way: edit one
//!   JSON file, no long command lines.
//! - `awan-profile whoami <handle>` with optional `--name`, `--role`,
//!   `--location`, `--stack`, `--streak N`, `--lyrics "one|two|three"`, `--gif`.

use std::io::{IsTerminal, Write, stdout};
use std::time::Duration;

use awan_core::{Character, Reel, Size};

mod draw;
mod gif;
mod script;
mod statsbanner;
mod story;
mod wall;

use script::Profile;

/// Pause between frames.
const FRAME_DELAY: Duration = Duration::from_millis(90);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("whoami") => whoami(&args),
        Some("stats") => stats(&args),
        _ => {
            eprintln!("usage: awan-profile whoami --config awan.json [--gif out.gif]");
            eprintln!("   or: awan-profile stats  --config awan.json [--out banner.png]");
            std::process::exit(2);
        }
    }
}

/// Render the standalone stats banner (no character) to a PNG.
fn stats(args: &[String]) {
    let path = flag(args, "--config").unwrap_or_else(|| fail("stats needs --config awan.json"));
    let profile = load(&path);
    let out = flag(args, "--out").unwrap_or_else(|| "stats.png".to_string());
    match statsbanner::render_stats(&profile, &out) {
        Ok(()) => eprintln!("wrote {out}"),
        Err(e) => fail(&e.to_string()),
    }
}

/// Play or render the character reel.
fn whoami(args: &[String]) {
    let (profile, output) = match flag(args, "--config") {
        Some(path) => {
            let p = load(&path);
            let out = flag(args, "--gif").or_else(|| non_empty(&p.output));
            (p, out)
        }
        None => (from_flags(args), flag(args, "--gif")),
    };

    let reel = Reel::story(character_of(&profile), &profile.acts()).with_size(Size::Seamless);
    match output {
        Some(path) => match gif::render_gif(&reel, &profile, &path) {
            Ok(()) => eprintln!("wrote {path} ({} frames)", reel.ticks()),
            Err(e) => fail(&e.to_string()),
        },
        None => play(&reel, &profile),
    }
}

/// The character to star in the reel: a TOML spec if given, else the built-in.
fn character_of(profile: &Profile) -> Character {
    if profile.character.is_empty() {
        return Character::default();
    }
    let spec = awan_core::spec::load(std::path::Path::new(&profile.character))
        .unwrap_or_else(|e| fail(&format!("{}: {e}", profile.character)));
    Character::from_spec(&spec).unwrap_or_else(|e| fail(&format!("{}: {e}", profile.character)))
}

/// Load and parse an `awan.json` profile, exiting with a message on error.
fn load(path: &str) -> Profile {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| fail(&format!("{path}: {e}")));
    serde_json::from_str(&text).unwrap_or_else(|e| fail(&format!("{path}: {e}")))
}

/// Build a profile from command-line flags.
fn from_flags(args: &[String]) -> Profile {
    Profile {
        username: args.get(1).cloned().unwrap_or_default(),
        character: flag(args, "-c").unwrap_or_default(),
        name: flag(args, "--name").unwrap_or_default(),
        role: flag(args, "--role").unwrap_or_default(),
        location: flag(args, "--location").unwrap_or_default(),
        stack: flag(args, "--stack").unwrap_or_default(),
        streak: flag(args, "--streak")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        song: flag(args, "--song").unwrap_or_default(),
        artist: flag(args, "--artist").unwrap_or_default(),
        lyrics: flag(args, "--lyrics")
            .map(|s| s.split('|').map(str::trim).map(str::to_string).collect())
            .unwrap_or_default(),
        // the rest are config-only — no flag reaches for them
        ..Profile::default()
    }
}

/// The value following `name` in the args, if present.
fn flag(args: &[String], name: &str) -> Option<String> {
    let i = args.iter().position(|a| a == name)?;
    args.get(i + 1).cloned()
}

fn non_empty(s: &str) -> Option<String> {
    (!s.is_empty()).then(|| s.to_string())
}

fn fail(msg: &str) -> ! {
    eprintln!("awan-profile: {msg}");
    std::process::exit(1)
}

/// Play exactly one loop in the terminal, then exit — no signal handler.
fn play(reel: &Reel, profile: &Profile) {
    let color = stdout().is_terminal();
    let mut out = stdout().lock();
    let _ = write!(out, "\x1b[?25l\x1b[2J");
    for t in 0..reel.ticks() {
        let _ = write!(out, "\x1b[H");
        for line in reel.frame(t, color).split('\n') {
            let _ = writeln!(out, "{line}\x1b[K");
        }
        let _ = writeln!(out, "  {}\x1b[K", profile.line(reel, t).text);
        let _ = out.flush();
        std::thread::sleep(FRAME_DELAY);
    }
    let _ = writeln!(out, "\x1b[?25h");
}
