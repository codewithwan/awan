import { TICK_MS, actInfo, type Scene } from "../lib/acts";
import { barOf } from "../lib/hues";

/** How long the loop runs, what it weighs, and which beat you're in.
 *
 *  A profile banner is not a film: nobody scrolls past a README and waits a
 *  minute to learn you like football, and every second is bytes the reader
 *  pays for. Bands and the KB slope come from real renders — 20s ≈ 444 KB,
 *  31s ≈ 532 KB, 53s ≈ 705 KB, 77s ≈ 936 KB. */
const BANDS = [
  { max: 25, label: "tight", tone: "text-lime-ink", note: "people watch this one to the end" },
  { max: 40, label: "good", tone: "text-sky-ink", note: "a comfortable length for a profile" },
  { max: 60, label: "long", tone: "text-gold-ink", note: "the last beats rarely get seen" },
  { max: Infinity, label: "too long", tone: "text-punch-ink", note: "nobody waits this long — cut a beat" },
];

export function Meter({ story, at, solo, onPick }: { story: Scene[]; at: number; solo: number; onPick?: (i: number) => void }) {
  const ticks = story.reduce((n, s) => n + actInfo(s.act).ticks, 0) + 22; // walk on + off
  const secs = (ticks * TICK_MS) / 1000;
  const band = BANDS.find((b) => secs <= b.max)!;

  return (
    <div className="nb min-w-0 p-3">
      <div className="flex items-baseline gap-2">
        <span className={`text-3xl tabular-nums ${band.tone}`}>{secs.toFixed(0)}s</span>
        <span className={`text-[10px] uppercase ${band.tone}`}>{band.label}</span>
        <span className="ml-auto text-[10px] tabular-nums text-mute">≈ {Math.round(180 + secs * 9.8)} KB</span>
      </div>

      {/* every beat wears its own colour, so the bar reads as the story */}
      <div className="mt-2 flex h-4 gap-0.5 border-2 border-line">
        {story.map((s, i) => {
          const info = actInfo(s.act);
          return (
            <button
              key={i}
              onClick={() => onPick?.(i)}
              title={`${info.label} — ${(info.ticks * 0.09).toFixed(1)}s · click to play it alone`}
              aria-label={i === solo ? "Play the whole story" : `Play only ${info.label}`}
              className={`${barOf(info.hue)} ${i === at || i === solo ? "" : "opacity-40"}
                ${i === solo ? "outline-2 outline-ink" : ""}`}
              style={{ flexGrow: info.ticks }}
            />
          );
        })}
      </div>

      <p className="mt-2 text-[10px] text-mute">
        {band.note} ·{" "}
        <span className="text-faint">
          {solo >= 0 ? "playing one beat — click it again for the whole story" : "click a block to play that beat alone"}
        </span>
      </p>
    </div>
  );
}
