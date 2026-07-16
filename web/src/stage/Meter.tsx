import { TICK_MS, actInfo, type Scene } from "../lib/acts";

/** How long the loop runs, and roughly what it weighs.
 *
 *  A profile banner is not a film: nobody scrolls past a README and waits a
 *  minute to learn you like football, and every second is bytes the reader
 *  pays for. Bands and the KB slope come from real renders — 20s ≈ 444 KB,
 *  31s ≈ 532 KB, 53s ≈ 705 KB, 77s ≈ 936 KB. */
const BANDS = [
  { max: 25, label: "tight", tone: "text-lime", bar: "bg-lime", note: "people watch this one to the end" },
  { max: 40, label: "good", tone: "text-sky", bar: "bg-sky", note: "a comfortable length for a profile" },
  { max: 60, label: "long", tone: "text-gold", bar: "bg-gold", note: "the last beats rarely get seen" },
  { max: Infinity, label: "too long", tone: "text-punch", bar: "bg-punch", note: "nobody waits this long — cut a beat" },
];

export function Meter({ story, at }: { story: Scene[]; at: number }) {
  const ticks = story.reduce((n, s) => n + actInfo(s.act).ticks, 0) + 22; // walk on + off
  const secs = (ticks * TICK_MS) / 1000;
  const band = BANDS.find((b) => secs <= b.max)!;

  return (
    <div className="nb p-3">
      <div className="flex items-baseline gap-2">
        <span className={`text-3xl tabular-nums ${band.tone}`}>{secs.toFixed(0)}s</span>
        <span className={`text-[10px] uppercase ${band.tone}`}>{band.label}</span>
        <span className="ml-auto text-[10px] tabular-nums text-mute">≈ {Math.round(180 + secs * 9.8)} KB</span>
      </div>
      <div className="mt-2 flex h-3 gap-0.5 border-2 border-line" aria-hidden>
        {story.map((s, i) => (
          <div
            key={i}
            className={i === at ? band.bar : "bg-cloud"}
            style={{ flexGrow: actInfo(s.act).ticks, opacity: i === at ? 1 : 0.45 }}
            title={actInfo(s.act).label}
          />
        ))}
      </div>
      <p className="mt-2 text-[10px] text-mute">{band.note}</p>
    </div>
  );
}
