# awan profile generator — design notes

> **Using it?** You want [`profile/`](../profile) for the format and
> [the editor](https://codewithwan.github.io/awan/) for the fast way. This file
> is why it's built the way it is, for anyone changing it.

A **seamless looping animation** generated from one JSON file, for a GitHub
profile README. The reader edits `awan.json`, calls one reusable workflow, and
an Action redraws the GIF nightly. It's a **separate, opt-in** layer — the core
engine and CLI are untouched by it.

## Separation

- **`awan-core`** owns all *animation*: the seam-free reel, scenes, sprites,
  the corner badge. (Additive — Go parity of the live show is preserved.)
- **`profile/`** (a `publish = false` crate) owns the *generator*: JSON config,
  the GIF encoder, dynamic data (streak), and the `whoami` / `render` commands.
  It never ships with the published `awan` binary/packages.

## The seamless loop

- A reel is a finite `N` ticks: **walk in from the left → scenes → walk out to
  the right**. The character is off-screen at both ends, so the loop point
  shows only background.
- The background is a function of loop **progress** (`t/N`); every drifting
  layer completes a whole number of cycles, so `frame(0) == frame(N)`.
- **Invariant:** every scene starts and ends on an empty stage. This makes the
  loop seamless *and* lets scenes be reordered freely.

## Config (`awan.json`)

Identity fields, a streak, a song + lyrics, an `output` path, and a `scenes`
array of `{ act, say }` beats. `say` supports `{name} {role} {location} {stack}
{streak} {username}`; the `sing` beat plays `lyrics` instead. See the full,
copy-ready file in [`profile/sample/awan.json`](../profile/sample/awan.json):

```jsonc
{
  "username": "codewithwan",
  "name": "Muhammad Ridwan",
  "role": "fullstack engineer",
  "stack": "Rust, Go & TypeScript",
  "streak": 1975,
  "song": "your favourite song",
  "artist": "the artist",
  "lyrics": ["a line", "another line"],
  "output": "assets/awan.gif",
  "scenes": [
    { "act": "wave",     "say": "hi there! i'm {name}" },
    { "act": "rocket",   "say": "i build with {stack}" },
    { "act": "campfire", "say": "{streak}-day streak" },
    { "act": "sing" },
    { "act": "sleep",    "say": "okay... nap time, zzz" }
  ]
}
```

`scenes` is an ordered array — that is how the order is customised. Omit it for
a default story.

## Scenes (acts)

`wave`, `present`, `stroll`, `rocket`, `launch`, `bake`, `campfire`, `stats`
(a terminal window that prints the numbers), `sing`, `soccer`, `sleep`, `dance`. Each `say` is
drawn with a small **pixel icon**
chosen by the act (pin, briefcase, code, fire, star, heart). Karaoke (`sing`)
has no mic — he steps to the right and the lyrics play on the left, one line at
a time. The campfire is built (haul wood → toss a spark → it catches → pops).
The ground scrolls only during walking beats; only the clouds always drift.

## Dynamic data & the badge

A `🔥N` streak badge is pinned top-right every frame. Numbers — the badge and
the `stats` readout — live in `awan.json`; the CI job fetches the real values
(`gh api` → `jq`) and writes them in before rendering, so the binary stays
deterministic and offline.

## The stats banner

A separate output (`awan-profile stats`), and the only one with no character:
three metrics — all-time contributions, current streak, longest streak — in a row,
split by vertical rules, each with the dates it covers. It answers a different
question ("the numbers, at a glance") and wants the full width, so it isn't a
scene in the reel.

The split mirrors the rest: the engine knows nothing about it. The renderer lays
out `stat_boxes` (three `{value, label, note}` strings) in the 8×8 pixel font, so
it matches the character banner beside it; CI formats the strings. A `.gif` drifts
soft clouds behind the numbers and loops (two parallax clouds, each crossing a
whole number of spans per loop, so `frame(0) == frame(N)`); a `.png` is the still.

The numbers it wants reach past the one-year calendar the walking banner uses —
an all-time count from the first commit, the longest streak *ever* — so the
workflow walks the contribution calendar year by year from the year the account
was created, and only when the `stats_banner` input is set, so the extra API
calls are opt-in.

## Rendering to GIF

The generator lives in the `publish = false` `profile/` crate, so the `image` /
`font8x8` deps never touch the core packages. In seamless mode the character is
solid coloured cells → rasterise as rectangles; only the captions need the
bitmap font. Encodes with the `gif` crate, `loop = 0`.

`awan-profile whoami --config awan.json` — previews one loop and exits (no
Ctrl+C); with an `output` (or `--gif`) it writes the file.

## CI

The reader copies [`profile/sample/`](../profile/sample) into their profile
repo — an `awan.json`, a starter README, and a workflow that carries **no
personal data**, just `awan-profile whoami --config awan.json`, then commits the
GIF back.

## Build phases

1. **Loop core** — walk-out, progress background, walking ground, auto-exit. ✅
2. **Config** — `awan.json` → scene order + narration. ✅
3. **GIF encoder** — rasteriser, bitmap font, streak badge. ✅
4. **Scenes** — wave, rocket, bake, campfire, sing (karaoke), soccer, sleep. ✅
5. **Sample + docs** — `profile/sample/` template + workflow. ✅
6. **Next** — a packaged `awan-action`, and live streak fetching in CI.
