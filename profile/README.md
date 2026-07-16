# awan profile generator

Turn awan into a **seam-free looping banner for your GitHub profile** — he walks
in and tells your story, then loops (~60s). A separate, opt-in tool: it never
ships with the core `awan`.

<p align="center">
  <img src="../assets/profile-sample.gif" alt="awan profile banner sample" width="700">
</p>

Everything is driven by one file, [`awan.json`](sample/awan.json) — you control
the words **and** the order of the scenes.

## Two flavours, one engine

Same acts, same config shape — only the content differs. Pick a sample and edit:

| Banner | Sample | For |
|---|---|---|
| **Profile** | [`sample/awan.json`](sample/awan.json) | your `<you>/<you>` repo — bio, streak, song |
| **Project** | [`sample/project.json`](sample/project.json) | any repo README — what it does, install, stats |

<p align="center">
  <img src="../assets/project-sample.gif" alt="awan project banner sample" width="700">
</p>

*The project flavour: he welcomes you, then opens a terminal and prints the
repo's numbers.* Drop `{ "act": "stats" }` into **either** config — it reads the
same `stats` array.

## Get started

The [`sample/`](sample) folder is a ready-to-copy setup:

```
sample/
├── awan.json                     # ← profile flavour: bio, streak, song, scenes
├── project.json                  # ← project flavour: tagline, install, stats
├── README.md                     # a starter profile README (shows the GIF)
└── .github/workflows/awan.yml    # regenerates the GIF on every push
```

Copy it into your profile repo (`<you>/<you>`), edit `awan.json`, and push:

```sh
cp -r profile/sample/. my-profile/
cargo run -p awan-profile -- whoami --config my-profile/awan.json   # preview locally
```

Without `--gif` (or `output`) it previews one loop in the terminal and exits on
its own — no Ctrl+C.

## The `awan.json` format

```jsonc
{
  "handle": "codewithwan",
  "character": "",                      // path to a character TOML; empty = the buddy
  "name": "Muhammad Ridwan",
  "role": "fullstack engineer",
  "location": "Indonesia",
  "stack": "Rust, Go & TypeScript",
  "streak": 1975,                       // 🔥 badge, top-right
  "song": "your favourite song",        // shown as: my fav song "…" - artist
  "artist": "the artist",
  "lyrics": ["your", "favourite", "song lines"],
  "stats": ["stars:1.2k", "downloads:50k", "contributors:12",
            "version:v0.0.4", "license:MIT"],   // "label:value" — printed into
                                        // the terminal window by the `stats` act
  "output": "assets/awan.gif",          // where the GIF is written
  "scenes": [                            // ← reorder / add / remove freely
    { "act": "wave",     "say": "hi there! i'm {name}" },
    { "act": "present",  "say": "{role}" },
    { "act": "stroll",   "say": "based in {location}" },
    { "act": "rocket",   "say": "i build with {stack}" },
    { "act": "launch",   "say": "...then watch 'em take off!" },
    { "act": "bake",     "say": "and i love to eat" },
    { "act": "campfire", "say": "{streak}-day streak" },
    { "act": "sing" },
    { "act": "soccer",   "say": "then a bit of football" },
    { "act": "sleep",    "say": "okay... nap time, zzz" }
  ]
}
```

| Act | What he does |
|---|---|
| `wave` | bounces in an excited hello |
| `present` | stands and introduces himself |
| `stroll` | walks along (the ground scrolls only here) |
| `rocket` / `launch` | builds a rocket, then launches it |
| `bake` | fetches an oven and bakes |
| `campfire` | drags in wood, throws a spark, the fire catches, then pops |
| `sing` | steps aside; lyrics play karaoke-style on the left |
| `soccer` | juggles a ball |
| `stats` | opens a little terminal window and *prints* your numbers into it, line by line |
| `sleep` | yawns, dozes (`zzz`), wakes up |
| `dance` | a little dance |

- **`say`** is the caption; `{name} {role} {location} {stack} {streak} {handle}`
  are filled in. The `sing` beat needs no `say` — it plays your `lyrics`.
- Omit `scenes` entirely for a sensible default story.

## Auto-update on GitHub

The workflow in [`sample/.github/`](sample/.github) carries **no personal data**
— it just reads your `awan.json`, so you only ever edit that one file:

```yaml
- run: cargo install --git https://github.com/codewithwan/awan awan-profile
- run: awan-profile whoami --config awan.json
# …then commit the generated GIF back
```

Reference the GIF in your profile `README.md`: `![awan](assets/awan.gif)`.

## Live numbers

The renderer never touches the network — that stays a promise of the binary.
Instead **CI fetches and the binary draws**: the sample workflow pulls the real
stars / forks / contributors / version / license with `gh api`, writes them into
`awan.json` with `jq`, then renders. It also runs nightly, so the numbers stay
fresh without you touching anything.

## Notes

- **Lyrics are yours** — put a couple of lines of your own favourite song in
  `lyrics`. The sample ships original placeholder lines.
- The GIF is a few MB raw; shrink it with
  `gifsicle -O3 --lossy=80 --colors 64 assets/awan.gif -o assets/awan.gif`.
