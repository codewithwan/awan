import { useState } from "react";
import type { Scene } from "./acts";

/** Everything CI writes for you. Shipping them as zeroes rather than invented
 *  numbers is the point: a made-up streak is decoration, and this whole thing
 *  is supposed to draw the real one. */
const CI_FILLED = {
  streak: 0,
  stats: [],
  contributions: "",
  contrib_year: 0,
  contrib_recent: 0,
};

export type Identity = {
  handle: string;
  name: string;
  role: string;
  location: string;
  stack: string;
  song: string;
  artist: string;
  lyrics: string[];
};

export const buildConfig = (id: Identity, story: Scene[]) =>
  JSON.stringify(
    {
      ...id,
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

export function Output({ id, story }: { id: Identity; story: Scene[] }) {
  const config = buildConfig(id, story);
  return (
    <div className="flex flex-col gap-3">
      <p className="text-xs text-mute">
        Three files in the repo named after you — <span className="text-ink">{id.handle || "you"}/
        {id.handle || "you"}</span>. No secrets to set up: the token Actions already gives you
        reads everything this needs.
      </p>
      <File name="awan.json" body={config} />
      <File name=".github/workflows/awan.yml" body={WORKFLOW} />
      <File name="README.md" body={"![awan](assets/awan.gif)"} note="add this line anywhere" />
      <p className="text-xs text-mute">
        Push, and it draws itself — then again every night, with your real numbers. Want it frozen
        instead? Point at <code className="text-sky">@v0.0.5</code> rather than{" "}
        <code className="text-sky">@v0</code>: that tag pins the renderer too, so nothing changes
        under you.
      </p>
    </div>
  );
}

function File({ name, body, note }: { name: string; body: string; note?: string }) {
  const [copied, setCopied] = useState(false);
  const copy = async () => {
    await navigator.clipboard.writeText(body);
    setCopied(true);
    setTimeout(() => setCopied(false), 1200);
  };
  const download = () => {
    const url = URL.createObjectURL(new Blob([body], { type: "text/plain" }));
    const a = document.createElement("a");
    a.href = url;
    a.download = name.split("/").pop()!;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="px-box">
      <div className="flex items-center gap-2 border-b-2 border-edge px-2 py-1">
        <span className="text-xs text-sky">{name}</span>
        {note && <span className="text-[10px] text-edge">{note}</span>}
        <button onClick={copy} className="px-btn ml-auto bg-lime px-2 py-0.5 text-[10px] text-void">
          {copied ? "copied!" : "copy"}
        </button>
        <button onClick={download} className="px-btn bg-panel px-2 py-0.5 text-[10px] text-mute">
          save
        </button>
      </div>
      <pre className="max-h-56 overflow-auto p-2 text-[10px] leading-relaxed text-mute">{body}</pre>
    </div>
  );
}
