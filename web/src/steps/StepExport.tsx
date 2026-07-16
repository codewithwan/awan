import { files, type Identity } from "../lib/config";
import { downloadZip } from "../lib/zip";
import { castOf } from "../lib/characters";
import type { Scene } from "../lib/acts";
import { Card } from "../ui/Card";
import { Button } from "../ui/Button";
import { Code } from "../ui/Code";

/** The whole setup, as a folder. */
export function StepExport({ id, story, cast }: { id: Identity; story: Scene[]; cast: string }) {
  const you = id.username.trim();
  const bundle = files(id, story, castOf(cast).path);

  return (
    <Card title="Take it home" hint="one zip, three files, no secrets">
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
            Unzip it into{" "}
            <span className="text-sky-ink">
              {you || "you"}/{you || "you"}
            </span>{" "}
            — the repo named after you — and push.
            <br />
            The paths are the instructions: {".github/workflows/"} is the bit that's easy to get
            wrong.
          </p>
        </div>

        <ul className="flex flex-col gap-2">
          {Object.entries(bundle).map(([path, body]) => (
            <Preview key={path} path={path} body={body} />
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

/** What's in the box, foldable — there when you want to check it, out of the
 *  way when the button was the whole answer. */
function Preview({ path, body }: { path: string; body: string }) {
  return (
    <li className="border-3 border-line bg-void">
      <details>
        <summary className="cursor-pointer bg-slab px-2 py-1.5 text-[10px] text-sky-ink marker:text-faint">
          {path}
          <span className="ml-2 text-faint">{body.split("\n").length} lines</span>
        </summary>
        <Code body={body} />
      </details>
    </li>
  );
}
