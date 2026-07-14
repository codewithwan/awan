use awan_core::{Character, Size, Stage};
use std::io::Write;
fn main() {
    let stage = Stage::show(Character::default()).with_size(Size::Seamless);
    let mut out = std::io::stdout().lock();
    let _ = write!(out, "\x1b[2J");
    for _ in 0..3 {
        for k in 0..66 {
            let _ = write!(out, "\x1b[H");
            for line in stage.frame(569 + k, true).split('\n') {
                let _ = writeln!(out, "{line}\x1b[K");
            }
            let _ = writeln!(out, "  Awan: juggle juggle~\x1b[K");
            let _ = out.flush();
            std::thread::sleep(std::time::Duration::from_millis(90));
        }
    }
}
