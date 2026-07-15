//! The awan profile generator (separate, opt-in — never shipped with the core
//! binary). Plays a seamless reel narrating a profile, and can write it to a
//! looping GIF for a README. Upcoming: JSON config, streak scenes.
//!
//! - `awan-profile whoami <handle>` with optional `--name`, `--role`,
//!   `--location`, `--stack`, `--streak N`, `--lyrics "one|two|three"`.
//! - add `--gif out.gif` to write a looping GIF instead of previewing.

use std::io::{IsTerminal, Write, stdout};
use std::time::Duration;

use awan_core::{Character, Reel, Size};

mod gif;
mod icons;
mod script;

use script::Profile;

/// Pause between frames.
const FRAME_DELAY: Duration = Duration::from_millis(90);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("whoami") => {
            let handle = args.get(1).cloned().unwrap_or_default();
            let profile = Profile {
                name: flag(&args, "--name").unwrap_or_default(),
                role: flag(&args, "--role").unwrap_or_default(),
                location: flag(&args, "--location").unwrap_or_default(),
                stack: flag(&args, "--stack").unwrap_or_default(),
                streak: flag(&args, "--streak")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0),
                lyrics: flag(&args, "--lyrics")
                    .map(|s| s.split('|').map(str::trim).map(str::to_string).collect())
                    .unwrap_or_default(),
                handle,
            };
            let reel = Reel::new(Character::default()).with_size(Size::Seamless);
            match flag(&args, "--gif") {
                Some(path) => match gif::render_gif(&reel, &profile, &path) {
                    Ok(()) => eprintln!("wrote {path} ({} frames)", reel.ticks()),
                    Err(e) => {
                        eprintln!("awan-profile: {e}");
                        std::process::exit(1);
                    }
                },
                None => play(&reel, &profile),
            }
        }
        _ => {
            eprintln!("usage: awan-profile whoami <handle> [--name ..] [--gif out.gif]");
            std::process::exit(2);
        }
    }
}

/// The value following `name` in the args, if present.
fn flag(args: &[String], name: &str) -> Option<String> {
    let i = args.iter().position(|a| a == name)?;
    args.get(i + 1).cloned()
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
