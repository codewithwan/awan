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
  --hobby "football & coffee" \
  --gif awan.gif
```

Without `--gif` it previews one loop in the terminal and exits (no Ctrl+C).

| Flag | Shows as |
|---|---|
| `--name` | `♦ i'm <name>` |
| `--role` | `💼 <role>` |
| `--location` | `📍 <location>` |
| `--stack` | `>_ <stack>` |
| `--hobby` | `✦ off the clock: <hobby>` |
| `--gif <path>` | write a looping GIF instead of previewing |

Empty fields are skipped. The greeting and sign-off are always there.

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
               --hobby "football & coffee" \
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
