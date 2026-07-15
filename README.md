# awan ☁️

[![CI](https://github.com/codewithwan/awan/actions/workflows/ci.yml/badge.svg)](https://github.com/codewithwan/awan/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

> A tiny living character for your terminal — and a **personality layer** any
> CLI can embed: `wait`, `ask`, `react`.

<p align="center">
  <img src="assets/demo.gif" alt="awan strolling through his show in the terminal" width="700">
</p>

*He strolls in, bumps into a box, and dozes off — one loop of the show.*

He strolls, bonks into crates, dozes off mid-sit, chases butterflies, freezes
at falling gems, fetches his little oven to bake (and devour) a cake, dances
to a silent beat, juggles a ball until it bonks him dizzy, builds a rocket,
and watches it explode — then shakes off the soot and strolls on. On the very
first run, he hatches out of an egg. 🥚

## Quick start

```sh
npx @codewithwan/awan demo         # try it, no install (needs Node)
npm i -g @codewithwan/awan         # npm      → prebuilt binary
pip install awan-cli               # PyPI     → prebuilt binary
brew install codewithwan/awan/awan # Homebrew → prebuilt binary
cargo install awan-cli             # Cargo    → from source
# …or grab a binary straight from the Releases page
```

Every route installs the same `awan` command. The npm/PyPI/Homebrew packages
just fetch the prebuilt binary — no Rust toolchain needed.

Then:

| Command | What it does |
|---|---|
| `awan demo` | Play the full show on a loop (Ctrl+C to stop) |
| `awan demo --hatch` | Replay the first-run hatching intro |
| `awan demo -c characters/oyen.toml` | Same show, different character |
| `awan busy "compiling"` | The making-things loop with an animated caption — a living progress indicator |
| `awan sing "line one" "line two" …` | Karaoke: he steps to a mic and sings your lyrics, lighting them up word by word |
| `awan react cmd.failed` | Play the character's one-shot reaction to an event, then exit |
| `awan watch` | Ambient companion that reacts to events read from stdin (or `--pipe`) |
| `awan statusline "deploying"` | One static line — a tiny face, name and status — for prompts, tmux, or a Claude Code statusline |

`awan watch` turns him into a companion that reacts to your shell in real
time — source [shell/awan.zsh](shell/awan.zsh) and run `awan watch --pipe`
in a spare pane; he goes busy while a command runs, celebrates when it passes,
and chars when it fails. Which event maps to which scene is per-character data
(`[reactions]` in the TOML).

He renders **seam-free by default**: every pixel is painted as a coloured
cell background, so the font's line spacing is filled in and there are no
gaps between rows on any terminal (macOS Terminal.app included). Two other
looks are a flag away:

- `--size big` — the classic block-textured look (`░`/`▓` dither).
- `--size compact` — seam-free *and* half as tall (two pixel rows per cell).

**Status: early development (v0.0.x).** The engine is ported 1:1 from a
battle-tested Go implementation and verified frame-by-frame. Expect breaking
changes until v0.1.

## Works with any language

awan is a **binary plus a text protocol**, not a library you link. Anything
that can spawn a process and write a line of text can embed it — no SDK.

```js
// Node — npm i @codewithwan/awan
const awan = require("@codewithwan/awan");
const job = awan.busy("deploying");
await deploy();
job.stop();
```

```python
# Python — pip install awan-cli
import awan
awan.react("task.done")
```

```sh
# Any shell — feed events to an ambient companion
printf 'cmd.start\n' >> events   # he goes busy
printf 'cmd.ok\n'    >> events   # he celebrates
```

Events are plain lines (`cmd.start`, `cmd.ok`, `cmd.failed`, `task.done`,
`idle`) and each character's `[reactions]` decides what they do. Ready-made
wrappers for Node, Python, Go and shell live in [`clients/`](clients); the
full guide is [**docs/INTEGRATE.md**](docs/INTEGRATE.md).

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
| `awan-render` | Terminal backends and color-depth detection |
| `awan` | Public embed API for Rust CLI authors: `wait` / `ask` / `react` (planned) |
| `awan-cli` | The `awan` binary |

## Roadmap

- **Shipping** — seam-free rendering, ambient `watch` companion, cross-language
  event protocol with Node/Python/Go/shell clients ([docs](docs/INTEGRATE.md))
- **v0.1** — prebuilt binaries on every release, polished `awan` binary
- **v0.2** — in-process embed API for Rust CLI authors (`wait` / `ask` / `react`)
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
