//! Use case: embed the character *inside* a Rust program — no binary, no
//! subprocess. The engine is a pure function of `(tick, character)`, so you
//! own the loop and the output.

use awan_core::{statusline, Character, Stage};

fn main() {
    let stage = Stage::show(Character::default());

    // Render deterministic frames of the show (same tick → same frame).
    for t in 0..40 {
        print!("\x1b[2J\x1b[H{}", stage.frame(t, true));
        std::thread::sleep(std::time::Duration::from_millis(90));
    }

    // …or just a one-line badge for your own status output.
    println!("{}", statusline(&Character::default(), Some("build ok"), true));
}
