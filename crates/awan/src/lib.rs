//! # awan ☁️
//!
//! A tiny living character for your terminal — and a **personality layer**
//! any CLI can embed.
//!
//! ## The three embed primitives (designed, not yet implemented)
//!
//! - **`wait`** — replaces a spinner: the character works/animates while your
//!   closure runs, celebrates on `Ok`, reacts on `Err`.
//! - **`ask`** — replaces a bare `[y/N]`: the character delivers the question
//!   and reacts to the answer.
//! - **`react`** — one-shot reactions (`celebrate`, `fail`, `notice`).
//!
//! Design contract: rendering is inline in the host CLI's stdout (no daemon
//! required when embedded). If the ambient daemon is running, events are also
//! mirrored to it over a cross-language event protocol, so non-Rust tools
//! can integrate without an SDK.
//!
//! Non-TTY / CI environments are detected and all output is suppressed; the
//! primitives degrade to plain passthrough so embedding is always safe.
//!
//! Until the embed API lands, this crate re-exports the engine for early
//! adopters.

pub use awan_core as core;
pub use awan_render as render;
