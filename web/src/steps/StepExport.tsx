import { useState } from "react";
import { files, type Identity } from "../lib/config";
import { downloadZip } from "../lib/zip";
import { castOf, type Cast } from "../lib/characters";
import type { Scene } from "../lib/acts";
import { Card } from "../ui/Card";
import { Button } from "../ui/Button";
import { Code } from "../ui/Code";

/** The whole setup, as a folder — or a file at a time, if that's your way of
 *  working. The zip is the fast path, not the only one. */
export function StepExport({ id, story, cast, drawn }: { id: Identity; story: Scene[]; cast: string; drawn?: Cast }) {
  const you = id.username.trim();
  const useMine = cast === "mine" && drawn;
  const bundle = files(id, story, castOf(cast, drawn).path, useMine ? { path: drawn.path, toml: drawn.toml } : undefined);

  return (
    <Card title="Take it home" hint="one zip, or three files — your call">
      <div className="flex flex-col gap-4">
        <div className="flex flex-wrap items-center gap-3">
          <Button
            tone="lime"
            onClick={() => downloadZip(`awan-${you || "profile"}.zip`, bundle)}
            className="!px-5 !py-3 !text-sm"
          >
            ↓ download the folder
          </Button>
          <p className="text-[10px] leading-relaxed text-faint">
            Unzip into{" "}
            <span className="text-sky-ink">
              {you || "you"}/{you || "you"}
            </span>{" "}
            — the repo named after you — and push.
            <br />
            The paths come with it: {".github/workflows/"} is the bit that's easy to get wrong.
          </p>
        </div>

        <ul className="flex flex-col gap-2">
          {Object.entries(bundle).map(([path, body]) => (
            <FileRow key={path} path={path} body={body} />
          ))}
        </ul>

        <p className="text-[10px] leading-relaxed text-faint">
          Push and it draws itself, then again every night with your real numbers. No secrets to set
          up: the token Actions already gives you reads everything this needs. Want it frozen
          instead? Point at <span className="text-sky-ink">@v0.0.5</span> rather than{" "}
          <span className="text-sky-ink">@v0</span> — that tag pins the renderer too, so nothing can
          change under you.
        </p>
      </div>
    </Card>
  );
}

/** One file: copy it, save it, or open it up and read it first. */
function FileRow({ path, body }: { path: string; body: string }) {
  const [open, setOpen] = useState(false);
  const [copied, setCopied] = useState(false);

  const copy = async () => {
    await navigator.clipboard.writeText(body);
    setCopied(true);
    setTimeout(() => setCopied(false), 1200);
  };

  const save = () => {
    const url = URL.createObjectURL(new Blob([body], { type: "text/plain" }));
    const a = document.createElement("a");
    a.href = url;
    a.download = path.split("/").pop()!;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <li className="border-3 border-line bg-void">
      <div className="flex flex-wrap items-center gap-2 bg-slab px-2 py-1.5">
        <button
          onClick={() => setOpen(!open)}
          className="text-[10px] text-faint"
          aria-expanded={open}
          aria-label={`${open ? "Hide" : "Show"} ${path}`}
        >
          {open ? "▾" : "▸"}
        </button>
        <span className="text-[10px] text-sky-ink">{path}</span>
        <span className="text-[9px] text-faint">{body.trimEnd().split("\n").length} lines</span>
        <Button tone="lime" onClick={copy} className="ml-auto !px-2 !py-0.5 !text-[9px]">
          {copied ? "copied" : "copy"}
        </Button>
        <Button onClick={save} className="!px-2 !py-0.5 !text-[9px]">
          save
        </Button>
      </div>
      {open && <Code body={body} />}
    </li>
  );
}
