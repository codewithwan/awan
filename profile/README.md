# awan profile generator

Turn awan into a **seam-free looping banner for your GitHub profile** — he walks
in, waves, tells your story (name, role, location, stack, hobby), then walks out
and loops. A separate, opt-in tool: it never ships with the core `awan`.

<p align="center">
  <img src="../assets/profile-sample.gif" alt="awan profile banner sample" width="640">
</p>

## Try it locally

```sh
cargo run -p awan-profile -- whoami <your-handle> \
  --name "Your Name" \
  --role "Fullstack Engineer" \
  --location "Indonesia" \
  --stack "Rust / Go / TypeScript" \
  --streak 365 \
  --lyrics "a line from|your favourite song|goes here" \
  --gif awan.gif
```

Without `--gif` it previews one loop (~60s) in the terminal and exits (no
Ctrl+C). It's a little story: waves hello, introduces himself, builds & launches
a rocket, bakes, **sings** (mic-free — mouth and notes only), then dances out.

| Flag | Does |
|---|---|
| `--name` / `--role` / `--location` / `--stack` | your intro lines, each with an icon |
| `--streak N` | pins a `🔥 N` badge in the top-right corner |
| `--lyrics "a\|b\|c"` | lines he sings during the music beat (`\|`-separated) |
| `--gif <path>` | write a looping GIF instead of previewing |

Empty fields fall back to friendly defaults. **Lyrics are yours to provide** —
put in a couple of lines from your own favourite song. The GIF is ~4 MB raw;
optimise it with `gifsicle -O3 --lossy=80 --colors 64 awan.gif -o awan.gif`.

## Put it on your GitHub profile (auto-updating)

1. In your profile repo (`<you>/<you>`), copy this workflow to
   `.github/workflows/awan.yml` and fill in your details:

   ```yaml
   name: awan profile
   on:
     push: { branches: [main] }
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
         - run: |
             awan-profile whoami ${{ github.repository_owner }} \
               --name "Your Name" \
               --role "Fullstack Engineer" \
               --location "Indonesia" \
               --stack "Rust / Go / TypeScript" \
               --streak 365 \
               --lyrics "a line from|your favourite song" \
               --gif assets/awan.gif
         - run: |
             git config user.name  "github-actions[bot]"
             git config user.email "github-actions[bot]@users.noreply.github.com"
             git add assets/awan.gif
             git commit -m "chore: refresh awan profile" || echo "no changes"
             git push
   ```

2. Add it to your profile `README.md`:

   ```md
   ![awan](assets/awan.gif)
   ```

Push, and the Action renders `assets/awan.gif` and commits it back — your banner
regenerates itself whenever you tweak the details.

## Coming next

A `awan.json` config (so the details live in one file instead of flags), a
coding-streak campfire with a corner badge, and a packaged `awan-action` so the
workflow is a single `uses:` line. See [../docs/PROFILE.md](../docs/PROFILE.md).
