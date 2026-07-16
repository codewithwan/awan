import type { Scene } from "./acts";

export type Identity = {
  /** Your GitHub username. It names the repo this goes in, it's who CI reads
   *  the numbers for, and it's what he calls you when `name` is blank. */
  username: string;
  name: string;
  role: string;
  location: string;
  stack: string;
  song: string;
  artist: string;
  lyrics: string[];
};

export const BLANK: Identity = {
  username: "", name: "", role: "", location: "", stack: "", song: "", artist: "", lyrics: [],
};

/** Everything CI writes. They ship as zeroes rather than invented numbers on
 *  purpose: a made-up streak is decoration, and drawing the real one is the
 *  entire point of the thing. */
const CI_FILLED = { streak: 0, stats: [], contributions: "", contrib_year: 0, contrib_recent: 0 };

export const buildConfig = (id: Identity, story: Scene[], character = "") =>
  JSON.stringify(
    {
      ...id,
      // omitted for the built-in buddy: an empty key is a question the reader
      // has to answer before they know it isn't one
      ...(character ? { character } : {}),
      lyrics: id.lyrics.filter(Boolean),
      ...CI_FILLED,
      output: "assets/awan.gif",
      scenes: story.map((s) => ({
        act: s.act,
        ...(s.say ? { say: s.say } : {}),
        ...(s.then ? { then: s.then } : {}),
      })),
    },
    null,
    2,
  ) + "\n";

export const WORKFLOW = `name: awan profile
on:
  push:
    branches: [main, master]
    paths: ["awan.json"]
  schedule:
    - cron: "0 3 * * *"
  workflow_dispatch:

jobs:
  awan:
    uses: codewithwan/awan/.github/workflows/profile.yml@v0
    permissions:
      contents: write
    with:
      brag_over: 100
      brag_say: "i'm so excited!"
      cope_say: "...i'll fix that, promise"
`;

export const README_LINE = "![awan](assets/awan.gif)";

/** The three files, at the paths they belong at. Handing over a zip beats
 *  handing over three clipboards: the paths *are* the instructions, and
 *  ".github/workflows/" is exactly the bit someone gets wrong at midnight. */
export const files = (id: Identity, story: Scene[], character = "", drawn?: { path: string; toml: string }) => ({
  "awan.json": buildConfig(id, story, drawn?.path ?? character),
  ".github/workflows/awan.yml": WORKFLOW,
  "README.md": `${README_LINE}\n\n# hi, i'm ${id.name || id.username || "you"}\n`,
  // a character you drew travels with the config that names it: the zip is the
  // setup, and a config pointing at a file you don't have isn't a setup
  ...(drawn ? { [drawn.path]: drawn.toml } : {}),
});
