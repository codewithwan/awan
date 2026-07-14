//! Karaoke mode: the buddy sings into a handheld mic on the right while your
//! lyrics scroll on the left — the current line lights up word by word, the
//! last fades bold above, the next waits faint below. Music sync comes later.

mod panel;

use std::io::Write;
use std::time::Duration;

use panel::{Kind, styled_line};

use crate::character::{Character, MASCOT_W};
use crate::grid::{CANVAS_H, Grid, blit};
use crate::palette::Role;
use crate::pose::{EyeMode, LegsMode, Pose};
use crate::sprites::MIC;

const INTRO: i32 = 26; // walk out, reach the mic, settle on the right
const OUTRO: i32 = 16; // a final bow after the last line
const TICKS_PER_WORD: i32 = 9;
const SING_HOME: i32 = 21; // character x while singing (flush right, room to sway)
const LYRIC_PIXELS: usize = 18; // pixels 0..18 = the left text panel
const LYRIC_COLS: usize = LYRIC_PIXELS * 2; // 36 columns of text
const ROWS: [usize; 3] = [4, 6, 8]; // previous / current / next lines

/// A character singing a set of lyric lines.
pub struct Karaoke {
    character: Character,
    lines: Vec<Vec<String>>, // words per non-empty line
}

impl Karaoke {
    /// Build a karaoke from lyric lines (blank lines are dropped).
    pub fn new(character: Character, lines: Vec<String>) -> Self {
        let lines = lines
            .into_iter()
            .map(|l| l.split_whitespace().map(str::to_string).collect::<Vec<_>>())
            .filter(|w| !w.is_empty())
            .collect();
        Self { character, lines }
    }

    fn sing_ticks(&self) -> i32 {
        self.lines.iter().map(Vec::len).sum::<usize>() as i32 * TICKS_PER_WORD
    }

    /// Total run length: intro, every line, then the bow.
    pub fn total_ticks(&self) -> i32 {
        INTRO + self.sing_ticks() + OUTRO
    }

    /// At singing tick `s` (>= 0): the current line and how many of its words
    /// have been sung.
    fn cursor(&self, s: i32) -> (usize, usize) {
        let mut acc = 0;
        for (li, words) in self.lines.iter().enumerate() {
            let dur = words.len() as i32 * TICKS_PER_WORD;
            if s < acc + dur {
                return (li, ((s - acc) / TICKS_PER_WORD) as usize);
            }
            acc += dur;
        }
        let last = self.lines.len().saturating_sub(1);
        (last, self.lines.get(last).map_or(0, Vec::len))
    }

    fn compose(&self, t: i32) -> Grid {
        let mut grid = Grid::new();
        let (p, base_x) = if t < INTRO {
            let mut walk = Pose::default();
            (walk.eyes, walk.legs) = (EyeMode::Left, LegsMode::Walk);
            (walk, -MASCOT_W / 2 + (SING_HOME + MASCOT_W / 2) * t / INTRO) // half on at t=0
        } else {
            draw_notes(&mut grid, t - INTRO);
            (sing_pose(t - INTRO, self.total_ticks() - t), SING_HOME)
        };
        let x = base_x + p.dx;
        let rows = self.character.mascot_rows(p, t);
        blit(&mut grid, &rows, x, CANVAS_H - 6 + p.dy, Role::Body);
        if t >= INTRO {
            blit(&mut grid, MIC, x - 2, 8 + p.dy, Role::Tool); // mic, off his left
        }
        grid
    }

    /// Render frame `t` as terminal text; `color` toggles ANSI styling.
    pub fn frame(&self, t: i32, color: bool) -> String {
        let grid = self.compose(t);
        if t < INTRO || self.lines.is_empty() {
            return crate::play::render(&grid, &self.character, color);
        }
        let (li, wd) = self.cursor((t - INTRO).min(self.sing_ticks()));
        let panel = [
            self.line(li.checked_sub(1), Kind::Prev, 0, color),
            self.line(Some(li), Kind::Cur, wd, color),
            self.line(li.checked_add(1), Kind::Next, 0, color),
        ];
        let mut b = String::with_capacity(2048);
        for (ri, row) in grid.rows().enumerate() {
            b.push_str("  ");
            match ROWS.iter().position(|&r| r == ri) {
                Some(pi) => b.push_str(&panel[pi]),
                None => (0..LYRIC_COLS).for_each(|_| b.push(' ')),
            }
            crate::play::push_cells(&mut b, &row[LYRIC_PIXELS..], &self.character, color);
            b.push('\n');
        }
        crate::play::push_baseline(&mut b, color);
        b
    }

    /// One panel line, padded to the panel width. `li` out of range → blank.
    fn line(&self, li: Option<usize>, kind: Kind, done: usize, color: bool) -> String {
        match li.and_then(|i| self.lines.get(i)) {
            Some(words) => styled_line(words, kind, done, color),
            None => " ".repeat(LYRIC_COLS),
        }
    }

    /// Play through the song once. `stop` is polled between frames so Ctrl+C
    /// restores the cursor cleanly.
    pub fn play<W: Write>(&self, w: &mut W, color: bool, delay: Duration, stop: &dyn Fn() -> bool) {
        let _ = write!(w, "\x1b[?25l\x1b[2J");
        for t in 0..self.total_ticks() {
            if stop() {
                break;
            }
            let _ = write!(w, "\x1b[H");
            for line in self.frame(t, color).split('\n') {
                let _ = writeln!(w, "{line}\x1b[K");
            }
            let _ = w.flush();
            if !delay.is_zero() {
                std::thread::sleep(delay);
            }
        }
        let _ = writeln!(w, "\x1b[?25h");
    }
}

/// The singing pose: mouths the words, sways gently, bows at the very end.
fn sing_pose(s: i32, left: i32) -> Pose {
    let mut p = Pose {
        eyes: [EyeMode::Left, EyeMode::Happy][((s / 6) % 4 == 3) as usize],
        dx: [0, -1, 0, 1][((s / 3) % 4) as usize],
        mouth_open: (s / 3) % 2 == 0,
        dy: -((s % 12 == 0) as i32), // a small bob on the beat
        ..Pose::default()
    };
    if left <= OUTRO {
        (p.eyes, p.mouth_open, p.dy) = (EyeMode::Happy, false, 0); // a happy bow
    }
    p
}

/// Music notes drift up on the right while he sings.
fn draw_notes(grid: &mut Grid, s: i32) {
    for (i, &(nx, role)) in [(26, Role::Spark), (30, Role::Gem)].iter().enumerate() {
        let phase = (s + i as i32 * 6) % 14;
        if phase < 10 {
            let ny = 7 - phase / 2;
            grid.set(nx, ny, if phase % 4 < 2 { "♪ " } else { "♫ " }, role);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Karaoke;
    use crate::character::Character;

    fn sing(lines: &[&str]) -> Karaoke {
        let lines = lines.iter().map(|s| s.to_string()).collect();
        Karaoke::new(Character::default(), lines)
    }

    #[test]
    fn sings_word_by_word_scrolls_and_stays_bounded() {
        // Original placeholder lyrics — never real/copyrighted song text.
        let k = sing(&["tune we hum", "drift down", "home now"]);
        assert!(k.total_ticks() > 0);
        assert!(k.frame(40, false).contains("tune")); // current line on the left
        assert!(k.frame(40, false).contains("██")); // singer on the right, never hidden

        // More of the current line lights up bold-bright as it is sung.
        let lit = |t: i32| k.frame(t, true).matches("1;38;5;231").count();
        assert!(lit(45) > lit(30), "words light up as sung");

        // On line two: line one fades bold above, line three waits faint below.
        let c = k.frame(60, true);
        assert!(c.contains("1;38;5;245") && c.contains("2;38;5;236"));

        // An absurd line truncates rather than breaking the fixed-width layout.
        let long = "w ".repeat(90);
        let wide = sing(&[long.as_str()]).frame(50, false);
        assert!(wide.lines().all(|l| l.chars().count() == 66), "fixed width");

        let _ = sing(&[""]).frame(0, true); // blank input never panics
    }
}
