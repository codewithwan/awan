import { useState } from "react";
import { type Character, type Glyph, nits, toToml } from "../lib/spec";
import { check_spec } from "../wasm/awan_wasm";
import { Reel } from "../stage/Reel";
import { Canvas, Brushes } from "./Canvas";
import { Swatch } from "./Swatch";
import { STARTERS, starter } from "./starters";
import { Button } from "../ui/Button";
import { Card } from "../ui/Card";
import { Field } from "../ui/Field";
import { Code } from "../ui/Code";

/** Four beats that show what a character has to survive: standing still,
 *  walking, sitting down, and being pleased with itself. If it reads in these,
 *  it reads anywhere — every scene works with every character. */
const AUDITION = [
  { act: "present", say: "this is me" },
  { act: "stroll", say: "walking" },
  { act: "sleep", say: "sitting, dozing" },
  { act: "dance", say: "and pleased about it" },
];

type Props = { char: Character; onChange: (c: Character) => void; onClose: () => void };

/** Draw him, and watch him go.
 *
 *  The repo has said "characters are plain TOML — zero Rust" from the start,
 *  and meant it. But zero Rust still meant typing `#` and `+` into a text file,
 *  guessing at the shape, and running a build to find out. Two characters
 *  exist. That's the tell.
 */
export function Studio({ char, onChange, onClose }: Props) {
  const [brush, setBrush] = useState<Glyph>("#");
  const [showToml, setShowToml] = useState(false);
  const toml = toToml(char);
  // the engine's own verdict, in the engine's own words
  const broken = check_spec(toml);
  const wrong = nits(char);

  return (
    <div className="grid min-w-0 gap-4 xl:grid-cols-[1fr_1fr]">
      <Card title="Draw him" hint="drag to paint · right-drag to erase" tone="text-sky-ink">
        <div className="flex flex-col gap-3">
          <Brushes brush={brush} char={char} onPick={setBrush} />
          <Canvas char={char} brush={brush} onRows={(rows) => onChange({ ...char, rows })} />

          <div className="flex flex-col gap-2">
            <Swatch label="body" value={char.body} onChange={(body) => onChange({ ...char, body })} />
            <Swatch label="eye" value={char.eye} onChange={(eye) => onChange({ ...char, eye })} />
          </div>

          <div className="flex flex-wrap items-center gap-2 border-t-3 border-line pt-3">
            <span className="text-[10px] uppercase text-mute">start from</span>
            {STARTERS.map((s) => (
              <Button key={s.id} onClick={() => onChange({ ...char, ...starter(s.id) })}>
                {s.label}
              </Button>
            ))}
          </div>
        </div>
      </Card>

      <div className="flex min-w-0 flex-col gap-4">
        {broken ? (
          <div className="nb grid min-h-56 place-items-center p-6 text-center">
            <div>
              <p className="text-sm text-punch-ink">he won't load yet</p>
              <p className="mt-2 text-[10px] leading-relaxed text-mute">{broken}</p>
              <p className="mt-3 text-[10px] text-faint">
                That's the engine talking, not us — it's the same check CI runs.
              </p>
            </div>
          </div>
        ) : (
          <Reel story={AUDITION} toml={toml} id={{}} onBeat={() => {}} />
        )}

        <Card title="Who is he" tone="text-grape-ink">
          <div className="flex flex-col gap-2">
            <Field label="name" value={char.name} placeholder="Oyen" onChange={(name) => onChange({ ...char, name })} />
            <Field label="author" value={char.author} placeholder="you" onChange={(author) => onChange({ ...char, author })} />
            <Field
              label="about"
              value={char.description}
              placeholder="a chunky orange cat"
              onChange={(description) => onChange({ ...char, description })}
            />
          </div>

          {broken || wrong.length ? (
            <ul className="mt-3 flex flex-col gap-1">
              {broken && <li className="text-[10px] text-punch-ink">✕ {broken}</li>}
              {wrong.map((w) => (
                <li key={w} className="text-[10px] text-gold-ink">
                  · {w}
                </li>
              ))}
            </ul>
          ) : (
            <p className="mt-3 text-[10px] text-lime-ink">✓ he loads — every scene works with him</p>
          )}

          <div className="mt-3 flex flex-wrap gap-2">
            <Button tone="lime" onClick={onClose} disabled={!!broken}>
              use him ›
            </Button>
            <Button onClick={() => setShowToml(!showToml)}>{showToml ? "hide" : "show"} toml</Button>
          </div>

          <p className="mt-2 text-[10px] leading-relaxed text-faint">
            He rides along in the zip as <span className="text-sky-ink">characters/{slug(char.name)}.toml</span>,
            and your <span className="text-sky-ink">awan.json</span> points at him. Nothing else to wire.
          </p>
        </Card>

        {showToml && (
          <div className="border-3 border-line bg-void">
            <Code body={toml} />
          </div>
        )}
      </div>
    </div>
  );
}

/** A filename someone can type. */
export const slug = (name: string) =>
  name.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "") || "mine";
