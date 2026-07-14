//! Palette roles — cells store a semantic role, not a color. World roles
//! resolve to fixed scene colors; character roles resolve through the loaded
//! character's palette, so the same scenes recolor per character.

use crate::character::Character;

/// What a cell *is*, colored at paint time.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum Role {
    // Character-owned roles (resolved via the character palette).
    #[default]
    Body,
    Eye,
    EyeWhite,
    Charred,
    // World roles (fixed scene colors).
    Sky,
    Dust,
    Spark,
    Bang,
    Gem,
    Butterfly,
    Rocket,
    Flame,
    Tool,
    Crate,
}

impl Role {
    /// SGR foreground code (without the `\x1b[`/`m` wrapper).
    pub fn sgr(self, ch: &Character) -> &str {
        match self {
            Role::Body => &ch.colors.body,
            Role::Eye => &ch.colors.eye,
            Role::EyeWhite => &ch.colors.eye_white,
            Role::Charred => &ch.colors.charred,
            Role::Sky => "38;2;110;118;138",
            Role::Dust => "38;5;242",
            Role::Spark => "38;2;250;196;92",
            Role::Bang => "38;2;240;110;100",
            Role::Gem => "38;2;110;210;160",
            Role::Butterfly => "38;2;235;140;190",
            Role::Rocket => "38;2;185;195;215", // steel blue
            Role::Flame => "38;2;250;160;70",
            Role::Tool => "38;2;170;175;185",
            Role::Crate => "38;2;196;154;98", // wood
        }
    }
}

/// Convert a `#RRGGBB` hex color to an SGR truecolor foreground code.
pub(crate) fn hex_to_sgr(hex: &str) -> Option<String> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let n = u32::from_str_radix(hex, 16).ok()?;
    Some(format!("38;2;{};{};{}", n >> 16, (n >> 8) & 0xFF, n & 0xFF))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_converts_to_truecolor_sgr() {
        assert_eq!(hex_to_sgr("#EBEEF5").as_deref(), Some("38;2;235;238;245"));
        assert_eq!(hex_to_sgr("EBEEF5"), None);
        assert_eq!(hex_to_sgr("#XYZ"), None);
    }
}
