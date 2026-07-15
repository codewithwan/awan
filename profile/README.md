# awan profile generator

Turn awan into a **seam-free looping banner for your GitHub profile** — he walks
in, tells your story, sings, warms up by a campfire, and loops (~60s). A
separate, opt-in tool: it never ships with the core `awan`.

<p align="center">
  <img src="../assets/profile-sample.gif" alt="awan profile banner sample" width="640">
</p>

Everything is driven by one file — [`awan.json`](awan.json). You control the
words *and* the order of scenes; no long command lines.

## Try it locally

```sh
cp profile/awan.json awan.json     # then edit it
cargo run -p awan-profile -- whoami --config awan.json
```

Add nothing else to preview one loop in the terminal (it exits on its own). The
`output` path in the JSON is where the GIF is written when run in CI.

## The `awan.json` format

```jsonc
{
  "handle": "codewithwan",
  "name": "Muhammad Ridwan",
  "role": "fullstack engineer",
  "location": "Indonesia",
  "stack": "Rust, Go & TypeScript",
  "streak": 1975,                       // 🔥 badge, top-right
  "lyrics": ["your", "favourite", "song lines"],
  "output": "assets/awan.gif",
  "scenes": [                            // ← reorder / add / remove freely
    { "act": "wave",     "say": "hi there! i'm {name}" },
    { "act": "present",  "say": "{role}" },
    { "act": "stroll",   "say": "based in {location}" },
    { "act": "rocket",   "say": "i build with {stack}" },
    { "act": "launch",   "say": "...then watch 'em take off!" },
    { "act": "bake",     "say": "and i love to eat" },
    { "act": "sing",     "say": "" },
    { "act": "campfire", "say": "{streak}-day streak" },
    { "act": "dance",    "say": "@{handle}" }
  ]
}
```

- **Acts** you can order: `wave`, `present`, `stroll`, `rocket`, `launch`,
  `bake`, `sing`, `campfire`, `dance`, `soccer`. Put `stroll` between beats so
  he walks (the ground only scrolls while he does).
- **`say`** is the caption; `{name} {role} {location} {stack} {streak} {handle}`
  are filled in. The `sing` beat shows your **`lyrics`** karaoke-style instead.
- Omit `scenes` entirely and a sensible default story is used.

## Put it on your GitHub profile (auto-updating)

In your profile repo (`<you>/<you>`), add **one file** `awan.json` (as above),
then this workflow — **no personal data in the workflow itself**:

```yaml
# .github/workflows/awan.yml
name: awan profile
on:
  push: { branches: [main], paths: ["awan.json"] }
  workflow_dispatch:
permissions:
  contents: write
jobs:
  awan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install --git https://github.com/codewithwan/awan awan-profile
      - run: awan-profile whoami --config awan.json
      - run: |
          git config user.name  "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add "$(python3 -c "import json;print(json.load(open('awan.json'))['output'])")"
          git commit -m "chore: refresh awan profile" || echo "no changes"
          git push
```

Then reference it in your profile `README.md`: `![awan](assets/awan.gif)`.

## Notes

- **Lyrics are yours** — put a couple of lines of your own favourite song in
  `lyrics`. The sample uses original placeholder lines.
- The GIF is a few MB raw; shrink it with
  `gifsicle -O3 --lossy=80 --colors 64 assets/awan.gif -o assets/awan.gif`.
