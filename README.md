# awan ☁️

> A tiny living character for your terminal — and a **personality layer** any
> CLI can embed: `wait`, `ask`, `react`.

**Status: early development (v0.0.x).** The core engine — scenes, the face
system, and the frame composer — is ported from an earlier battle-tested Go
implementation and verified frame-by-frame. Characters are fully data-driven
TOML specs. Expect breaking changes until v0.1.

On the very first `awan demo`, the buddy hatches out of an egg. 🥚

## Workspace layout

| Crate | Purpose |
|---|---|
| `awan-core` | Scene engine, face system, character spec (TOML) loader |
| `awan-render` | Seam-free terminal backends: half-block + bg-color, 256-color ladder, graphics protocols |
| `awan` | Public embed API for CLI authors: `wait` / `ask` / `react` |
| `awan-cli` | The `awan` binary: `demo`, `watch`, `idle`, `statusline`, `event` |

```sh
cargo run -p awan-cli -- demo                            # play the show (Ctrl+C to stop)
cargo run -p awan-cli -- demo -c characters/oyen.toml    # same show, different character
cargo run -p awan-cli -- busy "compiling"                # the working loop, with a caption
cargo test --workspace                                   # engine + spec + detection tests
```

## Characters

Characters are plain TOML — pixel rows plus a palette, zero Rust. The engine
derives all the face animation (blinks, glances, happy eyes, the startled
open mouth) from your art:

| Spec | Who |
|---|---|
| [characters/awan.toml](characters/awan.toml) | Awan ☁️ — the reference cloud buddy |
| [characters/oyen.toml](characters/oyen.toml) | Oyen 🐈 — a chunky orange cat |

Contributions welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

## Roadmap

- **v0.1** — seam-free half-block rendering, polished `awan` binary
- **v0.2** — embed API for CLI authors, ambient daemon, cross-language event
  protocol (any language, zero SDK)
- **Later** — more characters, graphics-protocol backends, community roster

## Promises

No telemetry · no network calls · single static binary · characters are data
(contribute one without writing Rust).

## License

MIT OR Apache-2.0, at your option.

---

Heritage: the engine began as `idl pet` (Go) inside the
[IDCloud](https://idcloud.app) CLI; awan is its standalone, embeddable second life.
