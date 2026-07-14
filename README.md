# awan ☁️

[![CI](https://github.com/codewithwan/awan/actions/workflows/ci.yml/badge.svg)](https://github.com/codewithwan/awan/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

> A tiny living character for your terminal — and a **personality layer** any
> CLI can embed: `wait`, `ask`, `react`.

```text
  ░░▓▓▓▓▓▓▓▓▓▓▓▓░░
  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░
  ▓▓▓▓▓▓░░
  ▓▓▓▓▓▓▓▓░░
                                              ██  ██
                                              ██████
                                                ██
                          ░░▓▓████████▓▓░░
                        ▓▓████████████████▓▓          ▓▓████████▓▓
                        ████▀▀▀▀████▀▀▀▀████          ██░░████░░██
                        ████████████████████          ██░░░░░░░░██
      ·                 ▓▓████████████████▓▓      · ░░░░████████▓▓
  ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
```

*He just baked a cake, ate the whole thing, and is very pleased about it.*

He strolls, bonks into crates, dozes off mid-sit, chases butterflies, freezes
at falling gems, fetches his little oven to bake (and devour) a cake, dances
to a silent beat, juggles a ball until it bonks him dizzy, builds a rocket,
and watches it explode — then shakes off the soot and strolls on. On the very
first run, he hatches out of an egg. 🥚

## Quick start

```sh
git clone https://github.com/codewithwan/awan && cd awan
cargo run -p awan-cli -- demo
```

| Command | What it does |
|---|---|
| `awan demo` | Play the full show on a loop (Ctrl+C to stop) |
| `awan demo --hatch` | Replay the first-run hatching intro |
| `awan demo -c characters/oyen.toml` | Same show, different character |
| `awan busy "compiling"` | The making-things loop with an animated caption — a living progress indicator |
| `awan sing "line one" "line two" …` | Karaoke: he steps to a mic and sings your lyrics, lighting them up word by word |
| `awan react cmd.failed` | Play the character's one-shot reaction to an event, then exit |
| `awan watch` | Ambient companion that reacts to events read from stdin (or `--pipe`) |

`awan watch` turns him into a companion that reacts to your shell in real
time — source [shell/awan.zsh](shell/awan.zsh) and run `awan watch --pipe`
in a spare pane; he goes busy while a command runs, celebrates when it passes,
and chars when it fails. Which event maps to which scene is per-character data
(`[reactions]` in the TOML).

Two seam-free looks are available for terminals that show gaps between rows
(macOS Terminal.app among them), both painting each pixel as a coloured cell
background so the font's line spacing is filled in:

- `--size seamless` — normal proportions, just gap-free.
- `--size compact` — packs two pixel rows per cell, so it's also half as tall.

**Status: early development (v0.0.x).** The engine is ported 1:1 from a
battle-tested Go implementation and verified frame-by-frame. Expect breaking
changes until v0.1.

## Characters

Characters are plain TOML — pixel rows plus a palette, **zero Rust**:

| Spec | Who |
|---|---|
| [characters/awan.toml](characters/awan.toml) | Awan ☁️ — the reference cloud buddy |
| [characters/oyen.toml](characters/oyen.toml) | Oyen 🐈 — a chunky orange cat |

The heart of a spec is the pixel art — here's the sprite block, abridged
(`eye_row`/`mouth_row`/`legs_row` point the engine at the rows to animate):

```toml
[sprite]
rows = [
    " #+    +# ",   # '#' solid · '+' dense · '-' light · '@' eye
    "+########+",
    "##@@##@@##",   # the engine derives blinks, glances & happy eyes from here
    "###----###",   # …and opens the mouth here when startled
    "+########+",
    " # #  # # ",
]
eye_row = 2
mouth_row = 3
legs_row = 5
# …plus sit_rows, leg_frames, a palette, and metadata
```

Copy [characters/awan.toml](characters/awan.toml) as your starting point,
edit the art, and run `awan demo -c my-character.toml`. The loader validates
the spec with friendly errors and drops your character into the full scene
library — no Rust, no rebuild of the engine.

## How it works

| Crate | Purpose |
|---|---|
| `awan-core` | Scene engine: deterministic frames from `(tick, character)` — no wall-clock, no RNG, snapshot-testable |
| `awan-render` | Terminal backends: color-depth detection now; seam-free half-block rendering next |
| `awan` | Public embed API for CLI authors: `wait` / `ask` / `react` (planned) |
| `awan-cli` | The `awan` binary |

## Roadmap

- **v0.1** — seam-free half-block rendering, polished `awan` binary
- **v0.2** — embed API for CLI authors, ambient daemon, cross-language event
  protocol (any language, zero SDK)
- **Later** — more characters and skits, graphics-protocol backends,
  community roster

## Promises

No telemetry · no network calls · single static binary · characters are data.

## Contributing

Three lanes: character art (TOML only), scenes/skits (light Rust), or engine
work. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT OR Apache-2.0, at your option.

---

Heritage: the engine began as `idl pet` (Go) inside the
[IDCloud](https://idcloud.app) CLI; awan is its standalone, embeddable second life.
