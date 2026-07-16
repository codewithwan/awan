import type { Identity } from "../lib/config";
import { BLANK } from "../lib/config";
import { EXAMPLE } from "../lib/example";
import { Card } from "../ui/Card";
import { Field } from "../ui/Field";
import { Button } from "../ui/Button";

const FIELDS: { key: keyof Identity; label: string; hint: string }[] = [
  { key: "username", label: "username", hint: "codewithwan" },
  { key: "name", label: "name", hint: "what he calls you" },
  { key: "role", label: "role", hint: "fullstack engineer" },
  { key: "location", label: "location", hint: "Indonesia" },
  { key: "stack", label: "stack", hint: "Rust, Go & TypeScript" },
  { key: "song", label: "song", hint: "your favourite" },
  { key: "artist", label: "artist", hint: "who sings it" },
];

/** Who he's talking about. Numbers are deliberately absent: those are CI's job,
 *  and a field for them would only invite someone to invent one. */
export function StepIdentity({ id, onChange }: { id: Identity; onChange: (id: Identity) => void }) {
  const filled = Object.values(id).some((v) => (Array.isArray(v) ? v.length : v));
  return (
    <div className="grid min-w-0 gap-4 lg:grid-cols-2">
      <Card title="About you" hint="these fill the {tokens} in his lines">
        <div className="mb-3 flex gap-2">
          <Button tone="gold" onClick={() => onChange(EXAMPLE)}>
            fill an example
          </Button>
          <Button onClick={() => onChange(BLANK)} disabled={!filled}>
            clear
          </Button>
        </div>
        <div className="flex flex-col gap-2">
          {FIELDS.map((f) => (
            <Field
              key={f.key}
              label={f.label}
              value={id[f.key] as string}
              placeholder={f.hint}
              onChange={(v) => onChange({ ...id, [f.key]: v })}
            />
          ))}
        </div>
      </Card>

      <Card title="Lyrics" hint="he sings these, one line at a time" tone="text-grape-ink">
        <div className="flex flex-col gap-2">
          {[0, 1, 2].map((i) => (
            <Field
              key={i}
              label={`line ${i + 1}`}
              value={id.lyrics[i] ?? ""}
              placeholder={i === 0 ? "the first line he sings" : ""}
              onChange={(v) => {
                const lyrics = [...id.lyrics];
                lyrics[i] = v;
                onChange({ ...id, lyrics });
              }}
            />
          ))}
        </div>
        <p className="mt-3 text-[10px] leading-relaxed text-mute">
          They light up word by word while he holds a mic. Skip the sing beat and you can leave
          these empty.
        </p>
      </Card>
    </div>
  );
}
