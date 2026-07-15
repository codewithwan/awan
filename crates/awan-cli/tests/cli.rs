//! End-to-end tests that exercise the real `awan` binary — the same surface
//! every language client drives over the process boundary.

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

const BIN: &str = env!("CARGO_BIN_EXE_awan");

fn run(args: &[&str]) -> std::process::Output {
    Command::new(BIN).args(args).output().expect("spawn awan")
}

#[test]
fn reports_its_version() {
    let out = run(&["--version"]);
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.starts_with("awan "), "got: {s:?}");
}

#[test]
fn help_lists_the_commands() {
    let s = String::from_utf8_lossy(&run(&[]).stdout).to_string();
    for cmd in ["demo", "busy", "sing", "react", "watch"] {
        assert!(s.contains(cmd), "help missing `{cmd}`");
    }
}

#[test]
fn react_plays_a_known_event() {
    let out = run(&["react", "task.done"]);
    assert!(out.status.success());
    assert!(!out.stdout.is_empty(), "reaction produced no frames");
}

#[test]
fn react_to_an_unknown_event_is_reported() {
    let out = run(&["react", "no.such.event"]);
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("no reaction"), "stderr was: {err:?}");
}

#[test]
fn a_missing_character_spec_fails_cleanly() {
    let out = run(&["demo", "-c", "does-not-exist.toml"]);
    assert!(!out.status.success(), "should exit non-zero");
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn watch_consumes_piped_events_without_crashing() {
    // The ambient companion reads events on stdin; feed a few, then close the
    // pipe and stop it. It should still be running (not have crashed) when we
    // kill it.
    let mut child = Command::new(BIN)
        .arg("watch")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("spawn watch");
    {
        let mut stdin = child.stdin.take().unwrap();
        for ev in ["cmd.start", "cmd.ok", "task.done"] {
            writeln!(stdin, "{ev}").unwrap();
        }
    }
    std::thread::sleep(Duration::from_millis(300));
    assert!(
        matches!(child.try_wait(), Ok(None)),
        "watch exited early — it should keep running"
    );
    let _ = child.kill();
    let _ = child.wait();
}
