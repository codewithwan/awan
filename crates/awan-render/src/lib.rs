//! # awan-render
//!
//! Terminal output backends for the awan engine. The core promise: **art must
//! never tear**, regardless of the user's font or line spacing.
//!
//! ## Why block glyphs tear (and what we do instead)
//!
//! Foreground glyphs like `█` are drawn at font height, but a terminal *cell*
//! is font height **plus line spacing** — extra leading (macOS Terminal.app
//! defaults, custom line-height settings) leaves visible seams between rows.
//! The fix: paint solid pixels as **background-colored cells** and use the
//! half-block `▀` with fg+bg for 2× vertical resolution. Terminals fill the
//! whole cell rect (including leading) with the background color, so the art
//! stays seam-free everywhere. Pixel-perfect graphics protocols (kitty,
//! iTerm2, sixel) are a progressive enhancement on top.
//!
//! ## Backend ladder (selected automatically)
//!
//! 1. [`Backend::Graphics`] — kitty / iTerm2 / sixel pixel sprites (planned)
//! 2. [`Backend::HalfBlock`] — bg-color + `▀`, truecolor or 256
//! 3. [`Backend::Cells`] — plain colored cells, last resort
//!
//! Backend implementations are in progress. Terminal capability detection
//! below is settled and tested.

/// How frames are drawn to the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// Pixel sprites via kitty graphics / iTerm2 inline images / sixel.
    Graphics,
    /// Background-colored cells + `▀` half-blocks (seam-free everywhere).
    HalfBlock,
    /// Plain colored cells; last-resort fallback.
    Cells,
}

/// Color capability of the attached terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorDepth {
    /// 24-bit `38;2;r;g;b`.
    TrueColor,
    /// 256-color palette (`38;5;n`) — includes macOS Terminal.app.
    X256,
    /// 16 ANSI colors.
    X16,
}

/// Decide color depth from environment hints. Pure function for testability;
/// call it with the values of `TERM_PROGRAM`, `COLORTERM`, and `TERM`.
///
/// Notable: **macOS Terminal.app does not support truecolor** even though it
/// sets a modern `TERM`, so it is pinned to 256 colors explicitly.
pub fn detect_color_depth(
    term_program: Option<&str>,
    colorterm: Option<&str>,
    term: Option<&str>,
) -> ColorDepth {
    if term_program == Some("Apple_Terminal") {
        return ColorDepth::X256;
    }
    if let Some(ct) = colorterm {
        if ct.contains("truecolor") || ct.contains("24bit") {
            return ColorDepth::TrueColor;
        }
    }
    match term {
        Some(t) if t.contains("256color") => ColorDepth::X256,
        _ => ColorDepth::X16,
    }
}

/// Detect color depth from the current process environment.
pub fn detect_color_depth_from_env() -> ColorDepth {
    detect_color_depth(
        std::env::var("TERM_PROGRAM").ok().as_deref(),
        std::env::var("COLORTERM").ok().as_deref(),
        std::env::var("TERM").ok().as_deref(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apple_terminal_is_pinned_to_256_even_with_truecolor_hint() {
        let depth = detect_color_depth(
            Some("Apple_Terminal"),
            Some("truecolor"),
            Some("xterm-256color"),
        );
        assert_eq!(depth, ColorDepth::X256);
    }

    #[test]
    fn colorterm_wins_for_modern_terminals() {
        let depth =
            detect_color_depth(Some("iTerm.app"), Some("truecolor"), Some("xterm-256color"));
        assert_eq!(depth, ColorDepth::TrueColor);
    }

    #[test]
    fn term_256_fallback() {
        assert_eq!(
            detect_color_depth(None, None, Some("screen-256color")),
            ColorDepth::X256
        );
    }

    #[test]
    fn bare_term_means_16_colors() {
        assert_eq!(
            detect_color_depth(None, None, Some("xterm")),
            ColorDepth::X16
        );
    }
}
