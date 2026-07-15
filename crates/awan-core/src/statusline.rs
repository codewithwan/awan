//! A single-line status badge for prompts, tmux, or a Claude Code statusline.
//! Unlike the animated shows this prints one static, colored line and exits —
//! ideal to embed in a shell prompt where it is re-rendered on every command.

use crate::character::Character;
use crate::palette::Role;

/// Render a one-line badge: a tiny face in the character's colour, its name,
/// and an optional status label. `color` wraps it in ANSI (leave on for
/// prompts — set it from `NO_COLOR`, not from a TTY check).
pub fn statusline(ch: &Character, label: Option<&str>, color: bool) -> String {
    let face = "(•‿•)";
    let name = &ch.name;
    let tail = match label {
        Some(l) if !l.is_empty() => format!("  {l}"),
        _ => String::new(),
    };
    if color {
        let body = Role::Body.sgr(ch);
        format!("\x1b[{body}m{face}\x1b[0m {name}{tail}")
    } else {
        format!("{face} {name}{tail}")
    }
}

#[cfg(test)]
mod tests {
    use super::statusline;
    use crate::character::Character;

    #[test]
    fn one_line_badge_carries_face_name_and_label() {
        let ch = Character::default();
        let line = statusline(&ch, Some("building"), false);
        assert!(!line.contains('\n'), "must be a single line");
        assert!(line.contains("(•‿•)"), "has the face");
        assert!(line.contains(&ch.name), "has the name");
        assert!(line.ends_with("building"), "has the label");
        // No label → no trailing gap.
        assert!(statusline(&ch, None, false).trim_end().ends_with(&ch.name));
        // Colour mode wraps in ANSI.
        assert!(statusline(&ch, None, true).contains("\x1b["));
    }
}
