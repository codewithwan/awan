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
  "streak": 0,                          // 🔥 badge, top-right. Leave it at 0:
                                        // CI counts it off your calendar, and
                                        // the badge hides itself when it is 0
  "song": "your favourite song",        // shown as: my fav song "…" - artist
  "artist": "the artist",
  "lyrics": ["your", "favourite", "song lines"],
  "stats": [],                          // "label:value" pairs, printed into the
                                        // terminal window by the `stats` act. CI
                                        // fills these: repos, stars earned,
                                        // followers, following
  "contributions": "",                  // one char per day, 0-4 = GitHub's own
                                        // quartiles, "." = no such day. 371 of
                                        // them = a year. CI writes it; you don't
                                        // — until it has, the wall stays empty,
                                        // so run the workflow once and look
  "contrib_year": 2060,                 // ↳ both filled in by the workflow too
  "contrib_recent": 183,
  "output": "assets/awan.gif",          // where the GIF is written
  "scenes": [                            // ← reorder / add / remove freely
    { "act": "wave",     "say": "hi there! i'm {name}" },
    { "act": "present",  "say": "{role}" },
    { "act": "stroll",   "say": "based in {location}" },
    { "act": "rocket",   "say": "i build with {stack}" },
    { "act": "launch",   "say": "...then watch 'em take off!" },
    { "act": "bake",     "say": "and i love to eat" },
    { "act": "campfire", "say": "{streak}-day streak" },
    { "act": "contributions",
      "say":  "i'm very happy, {contrib_year} this year",   // as the wall rises
      "then": "and {contrib_recent} in the last 30 days" }, // as the month lights
    { "act": "{verdict}", "say": "CI decides: excited, or not" },
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
| `contributions` | your GitHub year rises behind him; he walks over to the newest end and stands there while the last 30 days keep their colour and the year steps back |
| `sleep` | yawns, dozes (`zzz`), wakes up |
| `dance` | a little dance |

- **`say`** is the caption; `{name} {role} {location} {stack} {streak} {handle}`
  are filled in, plus `{contrib_year}` and `{contrib_recent}`. The `sing` beat
  needs no `say` — it plays your `lyrics`.
- Omit `scenes` entirely for a sensible default story.

### He reacts to how the month went

It says three things, in three moments. `say` lands as the wall rises — *"i'm
very happy, 2060 this year"*. **`then` takes over the instant the spotlight
hits**, so the month is named on the tick it lights up rather than a beat later
— *"and 183 in the last 30 days"*. Any act can carry a `then`; only the wall
has a moment worth splitting on.

`streak` comes free with this act: the same calendar already says which days
you showed up, so the workflow counts back from today and fills it in. A blank
*today* doesn't break the run — you've still got the rest of the day.

The mood comes last, on its own beat. `{ "act": "{verdict}" }` is filled in by
the workflow from your own numbers: clear `BRAG_OVER` and he `dance`s, fall
short and he `sleep`s.

```yaml
BRAG_OVER: 100 # your bar for "a good month"
BRAG_SAY: "i'm so excited!"
COPE_SAY: "...i'll fix that, promise"
```

Set the bar to your own idea of a good month, and write both lines yourself —
they should sound like you, not like us. Keep every caption under ~42
characters or it runs off the edge.

It needs **no PAT and no secret** — the stock `GITHUB_TOKEN` reads a public
contribution calendar fine. As ever, CI does the fetching and the binary only
draws, so the renderer still never touches the network.

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
