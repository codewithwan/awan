//! A finite, seam-free reel: the buddy walks in, plays a scripted sequence of
//! acts, and walks out. Clouds drift on loop *progress* (always moving); the
//! ground only scrolls while he is actually walking, and by exactly a whole
//! number of pebble-spacings, so `frame(0)` still matches `frame(ticks())`.
//! Separate from [`crate::Stage`] so the live show's Go parity is intact.

use crate::character::{Character, MASCOT_W};
use crate::grid::{CANVAS_H, CANVAS_W, GROUND_Y, Grid, blit};
use crate::palette::Role;
use crate::pose::{LegsMode, Pose};
use crate::scene::{Scene, locate, scene_for, show_ticks, show_walk_ticks};
use crate::sprites::{CLOUD_BIG, CLOUD_SMALL};
use crate::stage::{MASCOT_HOME, Size, WALK_IN_TICKS};

const WALK_OUT_TICKS: i32 = WALK_IN_TICKS + 1;
const PEBBLE_SPACING: i32 = 9;

/// One beat of the reel's story; each maps to a scene with a tuned duration.
#[derive(Clone, Copy)]
pub enum Act {
    Wave,
    Present,
    Stroll,
    RocketBuild,
    RocketLaunch,
    Bake,
    Sing,
    Campfire,
    Stats,
    Contributions,
    Sleep,
    Dance,
    Soccer,
}

/// A finite, looping reel of scenes.
pub struct Reel {
    character: Character,
    scenes: Vec<Scene>,
    size: Size,
}

impl Reel {
    /// A reel over a default story (the profile generator supplies its own).
    pub fn new(character: Character) -> Self {
        use Act::*;
        Self::story(character, &[Wave, Present, Sing, Soccer, Sleep, Dance])
    }

    pub fn story(character: Character, acts: &[Act]) -> Self {
        Self {
            character,
            scenes: acts.iter().map(|a| scene_for(*a)).collect(),
            size: Size::Seamless,
        }
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn ticks(&self) -> i32 {
        WALK_IN_TICKS + show_ticks(&self.scenes) + WALK_OUT_TICKS
    }

    /// The scene index and tick-within-scene at `t` (None while walking in/out).
    pub fn act_at(&self, t: i32) -> Option<(usize, i32)> {
        let show = show_ticks(&self.scenes);
        if t < WALK_IN_TICKS || t >= WALK_IN_TICKS + show {
            return None;
        }
        let (idx, k, _) = locate(&self.scenes, t - WALK_IN_TICKS);
        Some((idx, k))
    }

    pub fn is_leaving(&self, t: i32) -> bool {
        t >= WALK_IN_TICKS + show_ticks(&self.scenes)
    }

    pub fn frame(&self, t: i32, color: bool) -> String {
        let grid = self.compose(t);
        match self.size {
            Size::Compact if color => crate::halfblock::render(&grid, &self.character),
            Size::Seamless if color => crate::seamless::render(&grid, &self.character),
            _ => crate::play::render(&grid, &self.character, color),
        }
    }

    pub fn name(&self) -> &str {
        &self.character.name
    }

    /// The frame at tick `t` as canvas pixel colours (row-major, `None` = empty).
    pub fn pixel_grid(&self, t: i32) -> (usize, usize, Vec<Option<[u8; 3]>>) {
        let grid = self.compose(t);
        let cells = grid
            .rows()
            .flatten()
            .map(|c| crate::color::cell_rgb(c, &self.character).map(|(r, g, b)| [r, g, b]))
            .collect();
        (CANVAS_W as usize, CANVAS_H as usize, cells)
    }

    /// How far he has walked by tick `t` — only walk-in, strolls, and walk-out
    /// count, so idle scenes leave the ground still.
    fn walked(&self, t: i32) -> i32 {
        let show = show_ticks(&self.scenes);
        let scene_walk = show_walk_ticks(&self.scenes);
        if t < WALK_IN_TICKS {
            t
        } else if t >= WALK_IN_TICKS + show {
            WALK_IN_TICKS + scene_walk + (t - WALK_IN_TICKS - show)
        } else {
            let (idx, k, before) = locate(&self.scenes, t - WALK_IN_TICKS);
            WALK_IN_TICKS + before + if self.scenes[idx].walking { k } else { 0 }
        }
    }

    fn total_walk(&self) -> i32 {
        WALK_IN_TICKS + show_walk_ticks(&self.scenes) + WALK_OUT_TICKS
    }

    fn compose(&self, t: i32) -> Grid {
        let mut grid = Grid::new();
        clouds(&mut grid, t, self.ticks());
        ground(&mut grid, self.walked(t), self.total_walk());

        let show = show_ticks(&self.scenes);
        if t < WALK_IN_TICKS {
            draw_mascot(&mut grid, &self.character, walking(), t, -MASCOT_W + t);
        } else if t < WALK_IN_TICKS + show {
            let (idx, k, _) = locate(&self.scenes, t - WALK_IN_TICKS);
            let p = (self.scenes[idx].run)(k, t, &mut grid);
            draw_mascot(&mut grid, &self.character, p, t, MASCOT_HOME + p.dx);
        } else {
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

/// Parallax clouds as a function of loop progress — always drifting.
fn clouds(grid: &mut Grid, t: i32, len: i32) {
    let bx = wrap_x(30 - (CANVAS_W + 10) * t / len, 10);
    let sx = wrap_x(8 - (CANVAS_W + 7) * t / len, 7);
    blit(grid, CLOUD_BIG, bx, 0, Role::Sky);
    blit(grid, CLOUD_SMALL, sx, 2, Role::Sky);
}

fn wrap_x(x: i32, w: i32) -> i32 {
    let span = CANVAS_W + w;
    x.rem_euclid(span) - w
}

/// Sparse ground pebbles, scrolled by how far he has walked. The total shift
/// over the loop is a whole number of spacings, so they line up at the seam.
fn ground(grid: &mut Grid, walked: i32, total: i32) {
    let target = (total / PEBBLE_SPACING).max(1) * PEBBLE_SPACING;
    let shift = walked * target / total.max(1);
    for x in 0..CANVAS_W {
        if (x + shift) % PEBBLE_SPACING == 0 {
            grid.set(x, GROUND_Y, "· ", Role::Dust);
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
        let r = Reel::new(Character::default());
        let (a, b) = (r.compose(0), r.compose(r.ticks()));
        for y in 0..CANVAS_H {
            for x in 0..CANVAS_W {
                assert_eq!(a.at(x, y), b.at(x, y), "seam differs at ({x},{y})");
            }
        }
    }
}
