import type { Identity as Id } from "./Output";

const FIELDS: { key: keyof Id; label: string; hint: string }[] = [
  { key: "handle", label: "handle", hint: "codewithwan" },
  { key: "name", label: "name", hint: "what he calls you" },
  { key: "role", label: "role", hint: "fullstack engineer" },
  { key: "location", label: "location", hint: "Indonesia" },
  { key: "stack", label: "stack", hint: "Rust, Go & TypeScript" },
  { key: "song", label: "song", hint: "your favourite" },
  { key: "artist", label: "artist", hint: "who sings it" },
];

/** Who he's talking about. These fill the `{tokens}` a caption carries; the
 *  numbers are CI's job, not yours. */
export function Identity({ id, onChange }: { id: Id; onChange: (id: Id) => void }) {
  return (
    <div className="flex flex-col gap-2">
      {FIELDS.map((f) => (
        <label key={f.key} className="flex items-center gap-2">
          <span className="w-16 shrink-0 text-xs text-mute">{f.label}</span>
          <input
            value={id[f.key] as string}
            placeholder={f.hint}
            onChange={(e) => onChange({ ...id, [f.key]: e.target.value })}
            className="min-w-0 flex-1 border-2 border-edge bg-void px-2 py-1 text-xs
              text-ink outline-none placeholder:text-edge focus:border-sky"
          />
        </label>
      ))}
      <div className="mt-1 flex flex-col gap-1">
        <span className="text-xs text-mute">lyrics — he sings these, one line at a time</span>
        {[0, 1, 2].map((i) => (
          <input
            key={i}
            value={id.lyrics[i] ?? ""}
            placeholder={`line ${i + 1}`}
            onChange={(e) => {
              const lyrics = [...id.lyrics];
              lyrics[i] = e.target.value;
              onChange({ ...id, lyrics });
            }}
            className="border-2 border-edge bg-void px-2 py-1 text-xs text-ink
              outline-none placeholder:text-edge focus:border-sky"
          />
        ))}
      </div>
    </div>
  );
}
