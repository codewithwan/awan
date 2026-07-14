//! The stage: composes a character, a show, and an intro into frames.

use crate::character::{Character, MASCOT_W};
use crate::grid::{CANVAS_H, CANVAS_W, GROUND_Y, Grid, blit};
use crate::palette::Role;
use crate::pose::{LegsMode, Pose};
use crate::scene::hatch::{HATCH_TICKS, hatch_frame};
use crate::scene::{BUSY_SHOW, FULL_SHOW, Scene, locate, show_ticks, show_walk_ticks};
use crate::sprites::{CLOUD_BIG, CLOUD_SMALL};

/// Where the buddy settles (roughly centered).
pub(crate) const MASCOT_HOME: i32 = 11;
/// Walk-in from the left, played once.
pub(crate) const WALK_IN_TICKS: i32 = 21;

/// How a run begins, before the show loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intro {
    /// Strolls in from the left (the default).
    WalkIn,
    /// Hatches out of an egg — the first-run arrival.
    Hatch,
    /// Straight into the show (busy indicators).
    None,
}

impl Intro {
    fn ticks(self) -> i32 {
        match self {
            Intro::WalkIn => WALK_IN_TICKS,
            Intro::Hatch => HATCH_TICKS,
            Intro::None => 0,
        }
    }
}

/// How big the character is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    /// Full-height block glyphs (the default look).
    #[default]
    Big,
    /// Half-height, seam-free half-blocks — smaller and consistent everywhere.
    Compact,
    /// Full-height but seam-free — normal proportions, no gaps between rows.
    Seamless,
}

/// A character on a show, ready to render frames.
pub struct Stage {
    pub character: Character,
    show: &'static [Scene],
    intro: Intro,
    size: Size,
}

impl Stage {
    /// The full looping show, walk-in intro.
    pub fn show(character: Character) -> Self {
        Self {
            character,
            show: FULL_SHOW,
            intro: Intro::WalkIn,
            size: Size::Big,
        }
    }

    /// The "working…" loop — just the making-things skits, no intro.
    pub fn busy(character: Character) -> Self {
        Self {
            character,
            show: BUSY_SHOW,
            intro: Intro::None,
            size: Size::Big,
        }
    }

    /// A one-shot reaction to `event`, or `None` if the character maps no
    /// (known) reaction scene to it. Play the result with `cycles = 1`.
    pub fn react(character: Character, event: &str) -> Option<Self> {
        let show = character
            .reaction(event)
            .and_then(crate::scene::react::show_by_name)?;
        Some(Self {
            character,
            show,
            intro: Intro::None,
            size: Size::Big,
        })
    }

    pub fn with_intro(mut self, intro: Intro) -> Self {
        self.intro = intro;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub(crate) fn cycle_ticks(&self) -> i32 {
        show_ticks(self.show)
    }

    pub(crate) fn intro_ticks(&self) -> i32 {
        self.intro.ticks()
    }

    /// The cute one-liner shown under the canvas at tick `t`.
    pub(crate) fn caption(&self, t: i32) -> Option<&'static str> {
        let intro = self.intro.ticks();
        if t < intro {
            return match self.intro {
                Intro::Hatch => Some("hatching!"),
                Intro::WalkIn => Some("here i come~"),
                Intro::None => None,
            };
        }
        let tt = (t - intro) % show_ticks(self.show);
        Some(self.show[locate(self.show, tt).0].cap)
    }

    /// Compose the frame grid at tick `t`. Pure function of `t`, so frames
    /// are deterministic and snapshot-testable.
    pub(crate) fn compose(&self, t: i32) -> Grid {
        let mut grid = Grid::new();

        // Parallax clouds drift on their own clocks.
        blit(&mut grid, CLOUD_BIG, wrap_x(30 - t / 6, 10), 0, Role::Sky);
        blit(&mut grid, CLOUD_SMALL, wrap_x(8 - t / 9, 7), 2, Role::Sky);

        let intro_ticks = self.intro.ticks();
        if t < intro_ticks {
            draw_dust(&mut grid, 0);
            match self.intro {
                Intro::Hatch => hatch_frame(t, &mut grid, &self.character),
                _ => {
                    // stroll in from off-screen
                    let p = Pose {
                        legs: LegsMode::Walk,
                        ..Pose::default()
                    };
                    self.draw_mascot(&mut grid, p, t, -MASCOT_W + t);
                }
            }
            return grid;
        }

        let u = t - intro_ticks;
        let cycle = show_ticks(self.show);
        let (loops, tt) = (u / cycle, u % cycle);
        let (idx, k, walk_before) = locate(self.show, tt);
        let sc = &self.show[idx];

        let mut walked = loops * show_walk_ticks(self.show) + walk_before;
        if sc.walking {
            walked += k;
        }
        draw_dust(&mut grid, walked / 2);

        let p = (sc.run)(k, t, &mut grid);
        self.draw_mascot(&mut grid, p, t, MASCOT_HOME + p.dx);
        grid
    }

    fn draw_mascot(&self, grid: &mut Grid, p: Pose, t: i32, mx: i32) {
        let body = if p.charred { Role::Charred } else { Role::Body };
        let rows = self.character.mascot_rows(p, t);
        blit(grid, &rows, mx, CANVAS_H - 6 + p.dy, body);
    }

    /// Render the frame at tick `t`; `color` toggles ANSI codes. Compact and
    /// seamless are colour-only; mono keeps the full block render (Go parity).
    pub fn frame(&self, t: i32, color: bool) -> String {
        let grid = self.compose(t);
        match self.size {
            Size::Compact if color => crate::halfblock::render(&grid, &self.character),
            Size::Seamless if color => crate::seamless::render(&grid, &self.character),
            _ => crate::play::render(&grid, &self.character, color),
        }
    }
}

/// Keeps drifting background clouds cycling around the scene.
fn wrap_x(x: i32, w: i32) -> i32 {
    let span = CANVAS_W + w;
    x.rem_euclid(span) - w
}

fn draw_dust(grid: &mut Grid, off: i32) {
    for x in 0..CANVAS_W {
        let wx = x + off;
        if wx % 11 == 0 {
            grid.set(x, GROUND_Y, "· ", Role::Dust);
        } else if wx % 17 == 5 {
            grid.set(x, GROUND_Y, " ·", Role::Dust);
        }
    }
}
