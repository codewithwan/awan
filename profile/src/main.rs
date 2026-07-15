//! The awan profile generator (separate, opt-in — never shipped with the core
//! binary). For now it plays a seamless reel once and exits; upcoming phases
//! add JSON config, profile/streak scenes, and a built-in GIF encoder.
//!
//! - `awan-profile whoami <handle>` — preview one loop of the reel, then exit.
//! - `awan-profile whoami <handle> --gif out.gif` — write a looping GIF instead.

use std::io::{IsTerminal, Write, stdout};
use std::time::Duration;

use awan_core::{Character, Reel, Size};

mod gif;

/// Pause between frames.
const FRAME_DELAY: Duration = Duration::from_millis(90);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("whoami") => {
            let handle = args.get(1).cloned().unwrap_or_default();
            let gif_out = flag(&args, "--gif");
            let reel = Reel::new(Character::default()).with_size(Size::Seamless);
            match gif_out {
                Some(path) => match gif::render_gif(&reel, &path) {
                    Ok(()) => eprintln!("wrote {path} ({} frames)", reel.ticks()),
                    Err(e) => {
                        eprintln!("awan-profile: {e}");
                        std::process::exit(1);
                    }
                },
                None => play(&reel, &handle),
            }
        }
        _ => {
            eprintln!("usage: awan-profile whoami <handle> [--gif out.gif]");
            std::process::exit(2);
        }
    }
}

/// The value following `name` in the args, if present.
fn flag(args: &[String], name: &str) -> Option<String> {
    let i = args.iter().position(|a| a == name)?;
    args.get(i + 1).cloned()
}

/// Play exactly one loop, then exit — no signal handler, no infinite loop.
fn play(reel: &Reel, _handle: &str) {
    let color = stdout().is_terminal();
    let mut out = stdout().lock();
    let _ = write!(out, "\x1b[?25l\x1b[2J");
    for t in 0..reel.ticks() {
        let _ = write!(out, "\x1b[H");
        for line in reel.frame(t, color).split('\n') {
            let _ = writeln!(out, "{line}\x1b[K");
        }
        if let Some(cap) = reel.caption(t) {
            let _ = writeln!(out, "  {}: {cap}\x1b[K", reel.name());
        }
        let _ = out.flush();
        std::thread::sleep(FRAME_DELAY);
    }
    let _ = writeln!(out, "\x1b[?25h");
}
