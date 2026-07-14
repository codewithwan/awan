//! The `awan` binary. Subcommand scope:
//!
//! - `awan demo`       — play the show on a loop (hatches from an egg on first run)
//! - `awan busy`       — the "working…" loop with an animated caption
//! - `awan watch`      — ambient companion for a tmux/zellij pane (planned)
//! - `awan statusline` — one-line output for Claude Code / starship / tmux (planned)

use std::io::{IsTerminal, stdout};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use awan_core::{Character, Intro, Stage};

/// Pause between frames.
const FRAME_DELAY: Duration = Duration::from_millis(90);

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

fn run(stage: &Stage, color_wanted: bool, caption: Option<&str>) {
    let stop = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stop);
    ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst))
        .expect("failed to set the Ctrl+C handler");

    let color = color_wanted && stdout().is_terminal();
    let mut out = stdout().lock();
    stage.play(&mut out, color, 0, FRAME_DELAY, caption, &|| {
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
            println!();
            println!("Characters are plain TOML — see the characters/ directory.");
            println!("Planned: watch | idle | statusline | event");
        }
    }
}
