//! The lyric panel: render one karaoke line, styled by its role.

use super::LYRIC_COLS;

/// Which of the three panel lines a rendered line is.
#[derive(Clone, Copy)]
pub(super) enum Kind {
    Prev,
    Cur,
    Next,
}

/// Render a line's words to the panel width, styled by role; truncates at a
/// word boundary so the layout never breaks.
pub(super) fn styled_line(words: &[String], kind: Kind, done: usize, color: bool) -> String {
    let mut b = String::new();
    let mut col = 0usize;
    for (i, word) in words.iter().enumerate() {
        let wl = word.chars().count();
        let need = if col == 0 { wl } else { wl + 1 };
        if col + need > LYRIC_COLS {
            break;
        }
        if col > 0 {
            b.push(' ');
            col += 1;
        }
        if color {
            let code = match kind {
                Kind::Prev => "1;38;5;245",            // sung line: bold, faded
                Kind::Next => "2;38;5;236",            // upcoming: faint and dark
                Kind::Cur if i < done => "1;38;5;231", // sung word: bold, bright
                Kind::Cur => "38;5;240",               // unsung word: dim
            };
            b.push_str("\x1b[");
            b.push_str(code);
            b.push('m');
        }
        b.push_str(word);
        if color {
            b.push_str("\x1b[0m");
        }
        col += wl;
    }
    (col..LYRIC_COLS).for_each(|_| b.push(' '));
    b
}
