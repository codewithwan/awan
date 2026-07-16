import { TICK_MS, actInfo, type Scene } from "./acts";

/** How long the loop runs, and roughly what it weighs.
 *
 *  A profile banner is not a film. Nobody scrolls past your README and waits
 *  fifty seconds to find out you like football — and every second is bytes the
 *  reader pays for. The bands are drawn from real renders: 20s ≈ 444 KB, 31s ≈
 *  532 KB, 53s ≈ 705 KB, 77s ≈ 936 KB. */
const BANDS = [
  { max: 25, label: "tight", tone: "text-lime", note: "people watch this one to the end" },
  { max: 40, label: "comfortable", tone: "text-sky", note: "a good length for a profile" },
  { max: 60, label: "long", tone: "text-gold", note: "the last beats rarely get seen" },
  { max: Infinity, label: "too long", tone: "text-punch", note: "nobody waits this long — cut a beat or two" },
];

export function Meter({ story }: { story: Scene[] }) {
  const ticks = story.reduce((n, s) => n + actInfo(s.act).ticks, 0) + 22; // walk on + off
  const secs = (ticks * TICK_MS) / 1000;
  const band = BANDS.find((b) => secs <= b.max)!;
  const kb = Math.round(180 + secs * 9.8); // measured: GIF grows ~9.8 KB a second

  return (
    <div className="px-box p-3">
      <div className="flex items-baseline gap-2">
        <span className={`text-2xl ${band.tone} tabular-nums`}>{secs.toFixed(0)}s</span>
        <span className={`text-xs uppercase ${band.tone}`}>{band.label}</span>
        <span className="ml-auto text-xs text-mute tabular-nums">≈ {kb} KB</span>
      </div>
      <div className="mt-2 flex h-2 gap-px" aria-hidden>
        {story.map((s, i) => (
          <div
            key={i}
            className="bg-cloud"
            style={{ flexGrow: actInfo(s.act).ticks, opacity: 0.4 + (i % 3) * 0.3 }}
            title={actInfo(s.act).label}
          />
        ))}
      </div>
      <p className="mt-2 text-xs text-mute">{band.note}</p>
    </div>
  );
}
