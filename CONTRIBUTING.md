# Contributing to awan ☁️

Thanks for your interest! awan is in early development. Three contribution
lanes, pick your comfort level:

| Lane | Skills needed | Where |
|---|---|---|
| **Characters** | Pixel art + TOML — **zero Rust** | `characters/*.toml` (see `characters/awan.toml` for the spec format; blinks, glances, and the startled mouth are derived automatically from your eye/mouth rows) |
| **Scenes / skits** | Light Rust (mostly choreography data) | `crates/awan-core/src/scene/` |
| **Engine** | Rust | `awan-core`, `awan-render` (the ambient daemon is planned — see the roadmap) |

## Development setup

```sh
rustup toolchain install stable   # includes rustfmt + clippy
cargo test --workspace            # must stay green
cargo fmt --all && cargo clippy --all-targets -- -D warnings
```

## House rules

- Everything in the repo (code, comments, docs, commits) is written in English.
- Keep source files at 200 lines or fewer — split modules instead of growing them.
- Scenes and frames are deterministic pure functions of tick time — no
  wall-clock or RNG inside the engine (this keeps golden-frame tests stable).
- No telemetry, no network calls. PRs adding either will be declined.

## Character PRs

- Add a `characters/<name>.toml` spec (see `characters/awan.toml` for the
  full format) and paste a few frames of `awan demo -c characters/<name>.toml`
  into the PR so the art can be reviewed by eye.
- Original work only; no trademarked/branded characters.

## License

Dual-licensed under MIT OR Apache-2.0. By contributing, you agree your
contribution is licensed the same way.
