import { useState } from "react";
import { buildConfig, README_LINE, WORKFLOW, type Identity } from "../lib/config";
import type { Scene } from "../lib/acts";
import { Card } from "../ui/Card";
import { Button } from "../ui/Button";

/** Three files, and the honest version of what happens next. */
export function StepExport({ id, story }: { id: Identity; story: Scene[] }) {
  const who = id.handle || "you";
  return (
    <Card title="Take it home" hint={`three files in ${who}/${who}`}>
      <div className="flex flex-col gap-3">
        <File name="awan.json" body={buildConfig(id, story)} />
        <File name=".github/workflows/awan.yml" body={WORKFLOW} />
        <File name="README.md" body={README_LINE} note="add this line anywhere" />
        <p className="text-[10px] leading-relaxed text-mute">
          Push, and it draws itself — then again every night with your real numbers. No secrets to
          set up: the token Actions already gives you reads everything this needs. Want it frozen
          instead? Point at <span className="text-sky">@v0.0.5</span> rather than{" "}
          <span className="text-sky">@v0</span> — that tag pins the renderer too, so nothing can
          change under you.
        </p>
      </div>
    </Card>
  );
}

function File({ name, body, note }: { name: string; body: string; note?: string }) {
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
    a.download = name.split("/").pop()!;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="border-3 border-line bg-void">
      <div className="flex items-center gap-2 border-b-3 border-line bg-slab px-2 py-1.5">
        <span className="text-[10px] text-sky">{name}</span>
        {note && <span className="text-[9px] text-mute/60">{note}</span>}
        <Button tone="lime" onClick={copy} className="ml-auto !px-2 !py-0.5 !text-[9px]">
          {copied ? "copied" : "copy"}
        </Button>
        <Button onClick={save} className="!px-2 !py-0.5 !text-[9px]">
          save
        </Button>
      </div>
      <pre className="max-h-64 overflow-auto p-2 text-[10px] leading-relaxed text-mute">{body}</pre>
    </div>
  );
}
