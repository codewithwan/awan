//! The narration script: a person's profile turned into a timed sequence of
//! captioned lines (icon + text) that plays under the reel from "hi" to a
//! sign-off, spread evenly across the loop.

use crate::icons::{self, Icon};

/// A person's profile. Empty fields are skipped.
#[derive(Default)]
pub struct Profile {
    pub handle: String,
    pub name: String,
    pub role: String,
    pub location: String,
    pub stack: String,
    pub site: String,
}

/// One narration line: an optional icon and its text.
pub struct Line {
    pub icon: Option<&'static Icon>,
    pub text: String,
}

impl Profile {
    fn lines(&self) -> Vec<Line> {
        let mut v = vec![line(&icons::HEART, "hi there!")];
        if !self.name.is_empty() {
            v.push(line(&icons::DIAMOND, &format!("i'm {}", self.name)));
        }
        if !self.role.is_empty() {
            v.push(line(&icons::BRIEFCASE, &self.role));
        }
        if !self.location.is_empty() {
            v.push(line(&icons::PIN, &self.location));
        }
        if !self.stack.is_empty() {
            v.push(line(&icons::CODE, &self.stack));
        }
        let contact = if self.site.is_empty() {
            format!("@{}", self.handle)
        } else {
            self.site.clone()
        };
        v.push(line(&icons::GLOBE, &contact));
        v.push(line(&icons::HEART, "thanks for stopping by ~"));
        v
    }

    /// The line to show at tick `t` of a `ticks`-long loop.
    pub fn line(&self, t: i32, ticks: i32) -> Line {
        let lines = self.lines();
        let n = lines.len();
        let idx = (t.max(0) as i64 * n as i64 / ticks.max(1) as i64) as usize;
        lines.into_iter().nth(idx.min(n - 1)).unwrap()
    }
}

fn line(icon: &'static Icon, text: &str) -> Line {
    Line {
        icon: Some(icon),
        text: text.to_string(),
    }
}
