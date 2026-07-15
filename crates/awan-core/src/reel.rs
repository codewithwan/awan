//! A finite, seam-free reel: the buddy walks in from the left, plays a
//! sequence of self-contained scenes, and walks out to the right — sized so
//! the last frame flows back into the first for a perfect GIF loop.
//!
//! Unlike [`crate::Stage`], the background here is a function of loop
//! *progress*, so every drifting layer completes a whole number of cycles and
//! `frame(0)` matches `frame(ticks())`. Kept separate from Stage so the Go
//! parity of the live show is untouched.

use crate::character::{Character, MASCOT_W};
use crate::grid::{CANVAS_H, CANVAS_W, GROUND_Y, Grid, blit};
use crate::palette::Role;
use crate::pose::{LegsMode, Pose};
use crate::scene::{FULL_SHOW, Scene, locate, show_ticks};
use crate::sprites::{CLOUD_BIG, CLOUD_SMALL};
use crate::stage::{MASCOT_HOME, Size, WALK_IN_TICKS};

/// The walk-out ends one tick past the edge so the *last* played frame is
/// already fully off-screen — no leftover sliver at the loop point.
const WALK_OUT_TICKS: i32 = WALK_IN_TICKS + 1;

/// A finite, looping reel of scenes.
pub struct Reel {
    character: Character,
    scenes: &'static [Scene],
    size: Size,
}

impl Reel {
    /// A reel over the full show, rendered seam-free.
    pub fn new(character: Character) -> Self {
        Self {
            character,
            scenes: FULL_SHOW,
            size: Size::Seamless,
        }
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Total ticks in one loop: walk-in, the scenes, then walk-out.
    pub fn ticks(&self) -> i32 {
        WALK_IN_TICKS + show_ticks(self.scenes) + WALK_OUT_TICKS
    }

    /// Render frame `t` (`0..ticks()`) as terminal text.
    pub fn frame(&self, t: i32, color: bool) -> String {
        let grid = self.compose(t);
        match self.size {
            Size::Compact if color => crate::halfblock::render(&grid, &self.character),
            Size::Seamless if color => crate::seamless::render(&grid, &self.character),
            _ => crate::play::render(&grid, &self.character, color),
        }
    }

    /// A dialogue caption under the reel at tick `t`, if any.
    pub fn caption(&self, t: i32) -> Option<&'static str> {
        let show = show_ticks(self.scenes);
        if t < WALK_IN_TICKS {
            return Some("here i come~");
        }
        if t >= WALK_IN_TICKS + show {
            return Some("see ya~");
        }
        let u = t - WALK_IN_TICKS;
        Some(self.scenes[locate(self.scenes, u).0].cap)
    }

    /// This reel's character name (for the dialogue prefix).
    pub fn name(&self) -> &str {
        &self.character.name
    }

    fn compose(&self, t: i32) -> Grid {
        let mut grid = Grid::new();
        clouds(&mut grid, t, self.ticks());
        dust(&mut grid);

        let show = show_ticks(self.scenes);
        if t < WALK_IN_TICKS {
            // striding in from off the left edge
            draw_mascot(&mut grid, &self.character, walking(), t, -MASCOT_W + t);
        } else if t < WALK_IN_TICKS + show {
            let u = t - WALK_IN_TICKS;
            let (idx, k, _) = locate(self.scenes, u);
            let p = (self.scenes[idx].run)(k, t, &mut grid);
            draw_mascot(&mut grid, &self.character, p, t, MASCOT_HOME + p.dx);
        } else {
            // striding out toward the right edge, and off it
            let f = t - WALK_IN_TICKS - show;
            draw_mascot(&mut grid, &self.character, walking(), t, MASCOT_HOME + f);
        }
        grid
    }
}

fn walking() -> Pose {
    Pose {
        legs: LegsMode::Walk,
        ..Pose::default()
    }
}

fn draw_mascot(grid: &mut Grid, ch: &Character, p: Pose, t: i32, mx: i32) {
    let body = if p.charred { Role::Charred } else { Role::Body };
    let rows = ch.mascot_rows(p, t);
    blit(grid, &rows, mx, CANVAS_H - 6 + p.dy, body);
}

/// Parallax clouds as a function of loop progress: each completes exactly one
/// drift over `len`, so their positions at `t = 0` and `t = len` coincide.
fn clouds(grid: &mut Grid, t: i32, len: i32) {
    let big = CANVAS_W + 10;
    let small = CANVAS_W + 7;
    blit(
        grid,
        CLOUD_BIG,
        wrap_x(30 - big * t / len, 10),
        0,
        Role::Sky,
    );
    blit(
        grid,
        CLOUD_SMALL,
        wrap_x(8 - small * t / len, 7),
        2,
        Role::Sky,
    );
}

fn wrap_x(x: i32, w: i32) -> i32 {
    let span = CANVAS_W + w;
    x.rem_euclid(span) - w
}

/// A fixed scatter of ground specks — static so it never breaks the loop.
fn dust(grid: &mut Grid) {
    for x in 0..CANVAS_W {
        if x % 11 == 0 {
            grid.set(x, GROUND_Y, "· ", Role::Dust);
        } else if x % 17 == 5 {
            grid.set(x, GROUND_Y, " ·", Role::Dust);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Reel;
    use crate::character::Character;
    use crate::grid::{CANVAS_H, CANVAS_W};

    #[test]
    fn frame_zero_matches_the_seam() {
        // The character is off-screen at both ends, so the whole composed grid
        // at t=0 must equal the grid at t=ticks() — a perfectly seamless loop.
        let r = Reel::new(Character::default());
        let (a, b) = (r.compose(0), r.compose(r.ticks()));
        for y in 0..CANVAS_H {
            for x in 0..CANVAS_W {
                assert_eq!(a.at(x, y), b.at(x, y), "seam differs at ({x},{y})");
            }
        }
    }
}
