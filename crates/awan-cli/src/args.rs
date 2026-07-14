//! Command-line parsing, character loading, and the first-run hatch marker.

use std::path::{Path, PathBuf};

use awan_core::{Character, Size};

/// Parsed command line: subcommand, `-c/--character` path, flags, free args.
pub struct Args {
    pub cmd: String,
    pub character: Option<PathBuf>,
    pub pipe: Option<PathBuf>,
    pub size: Size,
    pub hatch: bool,
    pub rest: Vec<String>,
}

pub fn parse_args() -> Args {
    let mut args = std::env::args().skip(1);
    let mut parsed = Args {
        cmd: args.next().unwrap_or_default(),
        character: None,
        pipe: None,
        size: Size::Seamless,
        hatch: false,
        rest: Vec::new(),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-c" | "--character" => parsed.character = args.next().map(PathBuf::from),
            "--pipe" => parsed.pipe = args.next().map(PathBuf::from),
            "--size" => match args.next().as_deref() {
                Some("big") => parsed.size = Size::Big,
                Some("compact") => parsed.size = Size::Compact,
                Some("seamless") => parsed.size = Size::Seamless,
                _ => {}
            },
            "--hatch" => parsed.hatch = true,
            _ => parsed.rest.push(arg),
        }
    }
    parsed
}

/// Load a character spec, or the built-in default when no path is given.
pub fn load_character(path: Option<&Path>) -> Character {
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

/// Where the first-run marker lives (`$XDG_STATE_HOME` or `~/.local/state`).
fn hatch_marker() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))?;
    Some(base.join("awan").join("hatched"))
}

/// True exactly once: the buddy hatches on first run, then walks in ever after.
pub fn first_run() -> bool {
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
