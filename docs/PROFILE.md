# awan profile generator (design)

A **seamless looping animation** generated from one JSON file, for a GitHub
profile README. Users edit `awan.json`, add one workflow, and a GitHub Action
regenerates the GIF on every push. This is a **separate, opt-in** layer — the
core personality-layer CLI is untouched.

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

```json
{
  "handle": "codewithwan",
  "character": "awan",
  "size": "seamless",
  "output": { "width": 1050, "height": 300, "fps": 12, "loop": true },
  "badge": { "streak": true, "corner": "top-right" },
  "scenes": [
    { "type": "intro" },
    { "type": "wave" },
    { "type": "profile", "fields": [
      { "icon": "briefcase", "text": "Fullstack Engineer" },
      { "icon": "pin", "text": "Indonesia" },
      { "icon": "code", "text": "Rust · Go · TS" }
    ] },
    { "type": "typing", "text": "console.log('hi 👋')" },
    { "type": "streak" },
    { "type": "outro" }
  ]
}
```

- `scenes` is an ordered array — that is how the order is customised.
- `output.width` defaults to **1050** (safe under VHS too).
- `intro`/`outro` are auto-added when omitted, to keep the loop seamless.

## Scenes

`intro`, `outro`, `wave`, `typing`, `card`, `profile` (icon + text rows),
`streak` (campfire → number → embers), plus the existing skits (dance, soccer,
bake…). Karaoke (`sing`) drops the mic — the character just sings/talks.

Profile rows carry a small **pixel icon** (pin, briefcase, code, fire, star,
link, coffee), never plain text.

## Dynamic data & the badge

- A `🔥N` streak badge is pinned top-right on every frame (trivial in the
  rendered pipeline).
- The streak number is fetched by the **Action** (GitHub GraphQL contributions)
  and passed in as `--streak N`, so the binary stays deterministic.

## Rendering to GIF

Built-in encoder behind a `gif` cargo feature: in seamless mode the character
is solid coloured cells → rasterise as rectangles (no font); only captions need
a small embedded bitmap font. Encodes with the `gif` crate, `loop = 0`. VHS
remains a fallback.

`awan whoami "<handle>" [--config awan.json] [--gif out.gif]` — terminal preview
plays one loop and exits (no Ctrl+C); `--gif` writes the file.

## CI

`profile/action/` is a composite Action. A user's repo needs two files:

```yaml
on:
  push: { branches: [main], paths: ["awan.json"] }
  schedule: [{ cron: "0 0 * * *" }]   # refresh the streak daily
jobs:
  awan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: codewithwan/awan-action@v1
        with: { config: awan.json, output: assets/awan.gif }
```

The Action installs the gif-enabled binary, computes the streak, renders, and
commits the GIF. A template repo ships alongside.

## Build phases

1. **Loop core** — `outro`, progress background, finite auto-exit reel. ✅
2. **Config** — `awan.json` → scene sequence (in `profile/`).
3. **GIF encoder** — `gif` feature, rasteriser, bitmap font.
4. **Scenes** — wave, typing, card, profile+icons, campfire; drop the mic.
5. **Badge + streak input.**
6. **Action + template + docs.**
