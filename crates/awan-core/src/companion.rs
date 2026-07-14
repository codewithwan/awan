//! The reactive companion: an ambient show that reacts to events.
//!
//! It runs the full show by default, switches to the "busy" loop while a
//! command is running, and overlays a one-shot reaction (from the character's
//! `[reactions]` table) when an event maps to one. The engine stays pure —
//! this only tracks *which* deterministic show to render at each tick.

use crate::character::Character;
use crate::stage::{Size, Stage};

/// A character that reacts to a stream of events over time.
pub struct Companion {
    proto: Character,
    size: Size,
    ambient: Stage,
    reaction: Option<(Stage, i32)>, // (one-shot reaction, tick it started)
}

impl Companion {
    pub fn new(character: Character, size: Size) -> Self {
        Self {
            proto: character.clone(),
            size,
            ambient: Stage::show(character).with_size(size),
            reaction: None,
        }
    }

    /// Feed an event at tick `t`. Command lifecycle switches the ambient loop;
    /// any event the character maps a reaction to overlays that reaction once.
    pub fn feed(&mut self, event: &str, t: i32) {
        let sized = |s: Stage| s.with_size(self.size);
        match event {
            "cmd.start" => self.ambient = sized(Stage::busy(self.proto.clone())),
            "cmd.ok" | "cmd.done" | "idle" => self.ambient = sized(Stage::show(self.proto.clone())),
            _ => {}
        }
        if let Some(react) = Stage::react(self.proto.clone(), event) {
            self.reaction = Some((react.with_size(self.size), t));
        }
    }

    /// The active source at tick `t`: an ongoing reaction, or the ambient show.
    fn active(&self, t: i32) -> (&Stage, i32) {
        if let Some((react, start)) = &self.reaction {
            let k = t - start;
            if k < react.cycle_ticks() {
                return (react, k);
            }
        }
        (&self.ambient, t)
    }

    /// Render the frame at tick `t`.
    pub fn frame(&self, t: i32, color: bool) -> String {
        let (stage, k) = self.active(t);
        stage.frame(k, color)
    }

    /// The caption under the canvas at tick `t`.
    pub fn caption(&self, t: i32) -> Option<&'static str> {
        let (stage, k) = self.active(t);
        stage.caption(k)
    }

    /// This character's name, for the dialogue prefix.
    pub fn name(&self) -> &str {
        &self.proto.name
    }
}

#[cfg(test)]
mod tests {
    use super::Companion;
    use crate::character::Character;
    use crate::stage::Stage;

    #[test]
    fn a_reaction_overlays_then_the_ambient_resumes() {
        let mut c = Companion::new(Character::default(), crate::stage::Size::Big);
        c.feed("task.done", 100); // celebrate for ~20 ticks from t=100
        assert_eq!(c.caption(102), Some("yay!"), "the reaction plays");

        // past the reaction window it is the plain ambient show again
        let ambient = Stage::show(Character::default());
        assert_eq!(
            c.frame(200, false),
            ambient.frame(200, false),
            "ambient resumes"
        );
        assert_ne!(
            c.caption(102),
            c.caption(200),
            "caption returns to the show"
        );
    }

    #[test]
    fn cmd_start_switches_to_the_busy_loop() {
        let mut c = Companion::new(Character::default(), crate::stage::Size::Big);
        c.feed("cmd.start", 50);
        let busy = Stage::busy(Character::default());
        assert_eq!(
            c.frame(200, false),
            busy.frame(200, false),
            "busy while running"
        );
    }
}
