//! The canvas: a fixed grid of "pixels", each two terminal columns wide.

use crate::palette::Role;

/// Scene width in "pixels" (each pixel = 2 terminal columns).
pub(crate) const CANVAS_W: i32 = 32;
pub(crate) const CANVAS_H: i32 = 12;
pub(crate) const GROUND_Y: i32 = CANVAS_H - 1;

/// One canvas pixel: a two-column glyph plus a palette role resolved to a
/// concrete color at paint time.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Cell {
    pub glyph: &'static str,
    pub color: Role,
}

/// A flat, stack-allocated frame buffer — no per-row indirection.
pub(crate) struct Grid {
    cells: [Cell; (CANVAS_W * CANVAS_H) as usize],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [Cell::default(); (CANVAS_W * CANVAS_H) as usize],
        }
    }

    /// Write one cell; out-of-bounds coordinates are silently clipped.
    pub fn set(&mut self, x: i32, y: i32, glyph: &'static str, color: Role) {
        if (0..CANVAS_W).contains(&x) && (0..CANVAS_H).contains(&y) {
            self.cells[(y * CANVAS_W + x) as usize] = Cell { glyph, color };
        }
    }

    #[cfg(test)]
    pub fn at(&self, x: i32, y: i32) -> Cell {
        self.cells[(y * CANVAS_W + x) as usize]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Cell]> {
        self.cells.chunks(CANVAS_W as usize)
    }
}

/// Draw a sprite (rows in the glyph language) onto the grid at `(ox, oy)`,
/// clipping at the edges. `'@'`/`'%'`/`'^'` map to the eye palette roles.
pub(crate) fn blit<S: AsRef<str>>(grid: &mut Grid, sprite: &[S], ox: i32, oy: i32, color: Role) {
    for (y, row) in sprite.iter().enumerate() {
        for (x, ch) in row.as_ref().chars().enumerate() {
            let (glyph, c) = match ch {
                '#' => ("██", color),
                '+' => ("▓▓", color),
                '-' => ("░░", color),
                '@' => ("██", Role::Eye),
                '%' => ("██", Role::EyeWhite),
                '^' => ("▀▀", Role::Eye),
                '_' => ("▄▄", color),
                _ => continue,
            };
            grid.set(ox + x as i32, oy + y as i32, glyph, c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_clips_out_of_bounds_writes() {
        let mut g = Grid::new();
        g.set(-1, 0, "██", Role::Body);
        g.set(CANVAS_W, 0, "██", Role::Body);
        g.set(0, CANVAS_H, "██", Role::Body);
        g.set(3, 2, "██", Role::Body);
        assert_eq!(g.at(3, 2).glyph, "██");
        assert_eq!(g.at(0, 0).glyph, "");
    }

    #[test]
    fn blit_clips_at_edges() {
        let mut g = Grid::new();
        blit(
            &mut g,
            &["##", "##"],
            CANVAS_W - 1,
            CANVAS_H - 1,
            Role::Body,
        );
        assert_eq!(g.at(CANVAS_W - 1, CANVAS_H - 1).glyph, "██");
    }
}
