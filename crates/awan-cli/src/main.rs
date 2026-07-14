//! The `awan` binary. Subcommand scope:
//!
//! - `awan demo`       — play the show on a loop (hatches from an egg on first run)
//! - `awan busy`       — the "working…" loop with an animated caption
//! - `awan sing`       — karaoke: he steps to a mic and sings the lines you give
//! - `awan watch`      — ambient companion for a tmux/zellij pane (planned)
//! - `awan statusline` — one-line output for Claude Code / starship / tmux (planned)

use std::io::{IsTerminal, stdout};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use awan_core::{Character, Intro, Karaoke, Stage};

/// Pause between frames.
const FRAME_DELAY: Duration = Duration::from_millis(90);

/// Original placeholder song used when `awan sing` is given no lyrics.
const DEFAULT_LYRICS: &[&str] = &[
    "up where the little clouds play",
    "i drift along on the breeze",
    "nothing to carry today",
    "just me and the wide open sky",
];

/// Parsed command line: subcommand, `-c/--character` path, flags, free args.
struct Args {
    cmd: String,
    character: Option<PathBuf>,
    hatch: bool,
    rest: Vec<String>,
}

fn parse_args() -> Args {
    let mut args = std::env::args().skip(1);
    let mut parsed = Args {
        cmd: args.next().unwrap_or_default(),
        character: None,
        hatch: false,
        rest: Vec::new(),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-c" | "--character" => parsed.character = args.next().map(PathBuf::from),
            "--hatch" => parsed.hatch = true,
            _ => parsed.rest.push(arg),
        }
    }
    parsed
}

fn load_character(path: Option<&Path>) -> Character {
    let Some(path) = path else {
        return Character::default();
    };
    let spec = awan_core::spec::load(path).unwrap_or_else(|e| {
        eprintln!("awan: {}: {e}", path.display());
        std::process::exit(2);
    });
    Character::from_spec(&spec).unwrap_or_else(|e| {
        eprintln!("awan: {}: {e}", path.display());
        std::process::exit(2);
    })
}

/// First-run marker: the buddy hatches once, then walks in ever after.
fn hatch_marker() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))?;
    Some(base.join("awan").join("hatched"))
}

fn first_run() -> bool {
    let Some(marker) = hatch_marker() else {
        return false;
    };
    if marker.exists() {
        return false;
    }
    if let Some(dir) = marker.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(&marker, "");
    true
}

/// A Ctrl+C flag, shared with the signal handler so the play loop can exit.
fn stop_flag() -> Arc<AtomicBool> {
    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst))
        .expect("failed to set the Ctrl+C handler");
    stop
}

fn run(stage: &Stage, color_wanted: bool, caption: Option<&str>) {
    let stop = stop_flag();
    let color = color_wanted && stdout().is_terminal();
    let mut out = stdout().lock();
    stage.play(&mut out, color, 0, FRAME_DELAY, caption, &|| {
        stop.load(Ordering::SeqCst)
    });
}

fn sing(karaoke: &Karaoke) {
    let stop = stop_flag();
    let color = stdout().is_terminal();
    let mut out = stdout().lock();
    karaoke.play(&mut out, color, FRAME_DELAY, &|| {
        stop.load(Ordering::SeqCst)
    });
}

/// Play a one-shot reaction once, then exit.
fn react(stage: &Stage) {
    let stop = stop_flag();
    let color = stdout().is_terminal();
    let mut out = stdout().lock();
    stage.play(&mut out, color, 1, FRAME_DELAY, None, &|| {
        stop.load(Ordering::SeqCst)
    });
}

fn main() {
    let args = parse_args();
    match args.cmd.as_str() {
        "demo" => {
            let intro = if args.hatch || first_run() {
                Intro::Hatch
            } else {
                Intro::WalkIn
            };
            let stage = Stage::show(load_character(args.character.as_deref())).with_intro(intro);
            run(&stage, true, None);
        }
        "busy" => {
            let label = if args.rest.is_empty() {
                "is making something".to_string()
            } else {
                args.rest.join(" ")
            };
            let stage = Stage::busy(load_character(args.character.as_deref()));
            run(&stage, true, Some(&label));
        }
        "sing" => {
            let lines = if args.rest.is_empty() {
                DEFAULT_LYRICS.iter().map(|s| s.to_string()).collect()
            } else {
                args.rest.clone()
            };
            sing(&Karaoke::new(
                load_character(args.character.as_deref()),
                lines,
            ));
        }
        "react" => {
            let event = args.rest.first().map(String::as_str).unwrap_or("idle");
            let ch = load_character(args.character.as_deref());
            let name = ch.name.clone();
            match Stage::react(ch, event) {
                Some(stage) => react(&stage),
                None => eprintln!("awan: {name} has no reaction to \"{event}\""),
            }
        }
        "--version" | "-V" => println!("awan {}", env!("CARGO_PKG_VERSION")),
        _ => {
            println!(
                "awan ☁️  v{} — a tiny living character for your terminal",
                env!("CARGO_PKG_VERSION")
            );
            println!();
            println!("Commands:");
            println!("  demo  [--hatch] [-c <spec.toml>]   play the show (Ctrl+C to stop)");
            println!("  busy  [label]   [-c <spec.toml>]   the working loop, with a caption");
            println!("  sing  [\"line\" \"line\" …]            karaoke: one quoted line per lyric");
            println!(
                "  react <event>   [-c <spec.toml>]   play the character's reaction to an event"
            );
            println!();
            println!("Characters are plain TOML — see the characters/ directory.");
            println!("Planned: watch | idle | statusline | event");
        }
    }
}
