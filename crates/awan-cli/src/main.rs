//! The `awan` binary. Subcommand scope:
//!
//! - `awan demo`       — play the show on a loop (hatches from an egg on first run)
//! - `awan busy`       — the "working…" loop with an animated caption
//! - `awan sing`       — karaoke: he steps to a mic and sings the lines you give
//! - `awan react`      — play the character's one-shot reaction to an event
//! - `awan watch`      — ambient companion that reacts to events read from stdin
//! - `awan statusline` — one-line badge for Claude Code / starship / tmux

use std::io::{BufRead, IsTerminal, Write, stdout};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use awan_core::{Character, Companion, Intro, Karaoke, Stage};

mod args;
use args::{first_run, load_character, parse_args};

/// Pause between frames.
const FRAME_DELAY: Duration = Duration::from_millis(90);

/// Original placeholder song used when `awan sing` is given no lyrics.
const DEFAULT_LYRICS: &[&str] = &[
    "up where the little clouds play",
    "i drift along on the breeze",
    "nothing to carry today",
    "just me and the wide open sky",
];

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

/// Ambient companion: render the show while reacting to events read one per
/// line, from stdin or a named pipe (`--pipe`). Ctrl+C restores the cursor.
fn watch(character: Character, pipe: Option<std::path::PathBuf>, size: awan_core::Size) {
    let stop = stop_flag();
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || match pipe {
        // a fifo: reopen each time a writer (a shell hook) closes it
        Some(path) => {
            while let Ok(file) = std::fs::File::open(&path) {
                for line in std::io::BufReader::new(file).lines().map_while(Result::ok) {
                    if tx.send(line.trim().to_string()).is_err() {
                        return;
                    }
                }
            }
        }
        None => {
            for line in std::io::stdin().lock().lines().map_while(Result::ok) {
                if tx.send(line.trim().to_string()).is_err() {
                    break;
                }
            }
        }
    });

    let mut companion = Companion::new(character, size);
    let color = stdout().is_terminal();
    let mut out = stdout().lock();
    let _ = write!(out, "\x1b[?25l\x1b[2J");
    let mut t = 0;
    while !stop.load(Ordering::SeqCst) {
        while let Ok(ev) = rx.try_recv() {
            if !ev.is_empty() {
                companion.feed(&ev, t);
            }
        }
        let _ = write!(out, "\x1b[H");
        for line in companion.frame(t, color).split('\n') {
            let _ = writeln!(out, "{line}\x1b[K");
        }
        if let Some(cap) = companion.caption(t) {
            let _ = writeln!(out, "  {}: {cap}\x1b[K", companion.name());
        }
        let _ = out.flush();
        std::thread::sleep(FRAME_DELAY);
        t += 1;
    }
    let _ = writeln!(out, "\x1b[?25h");
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
            let stage = Stage::show(load_character(args.character.as_deref()))
                .with_intro(intro)
                .with_size(args.size);
            run(&stage, true, None);
        }
        "busy" => {
            let label = if args.rest.is_empty() {
                "is making something".to_string()
            } else {
                args.rest.join(" ")
            };
            let stage = Stage::busy(load_character(args.character.as_deref())).with_size(args.size);
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
                Some(stage) => react(&stage.with_size(args.size)),
                None => eprintln!("awan: {name} has no reaction to \"{event}\""),
            }
        }
        "watch" => watch(
            load_character(args.character.as_deref()),
            args.pipe,
            args.size,
        ),
        "statusline" => {
            let label = (!args.rest.is_empty()).then(|| args.rest.join(" "));
            let ch = load_character(args.character.as_deref());
            let color = std::env::var_os("NO_COLOR").is_none();
            println!("{}", awan_core::statusline(&ch, label.as_deref(), color));
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
            println!("  react <event>   [-c <spec.toml>]   play the character's reaction once");
            println!("  watch           [-c <spec.toml>]   companion that reacts to stdin events");
            println!("  statusline [text] [-c <spec.toml>]   one-line badge for prompts / tmux");
            println!();
            println!(
                "Renders seam-free by default. --size big = classic textured, compact = half height."
            );
            println!("Characters are plain TOML — see the characters/ directory.");
            println!("Feed watch: (echo cmd.start; sleep 2; echo cmd.failed) | awan watch");
        }
    }
}
