import { ACTS, type Scene } from "../lib/acts";
import { PixelIcon } from "../ui/PixelIcon";

/** The acts you can add. Every card wears its duration, because a reel gets too
 *  long one innocent-looking beat at a time and nobody feels that cost unless
 *  the price is on the label. */
export function Shelf({ onAdd }: { onAdd: (s: Scene) => void }) {
  return (
    <div className="grid grid-cols-2 gap-2 sm:grid-cols-3">
      {ACTS.map((a) => (
        <button
          key={a.id}
          onClick={() => onAdd({ act: a.id, say: "" })}
          className="nb-btn bg-void p-2 text-left"
        >
          <div className="flex items-center gap-2">
            <PixelIcon id={a.id} />
            <span className="text-[10px] uppercase text-ink">{a.label}</span>
            <span className="ml-auto text-[9px] tabular-nums text-faint">{(a.ticks * 0.09).toFixed(1)}s</span>
          </div>
          <p className="mt-1 text-[9px] leading-tight text-mute">{a.blurb}</p>
        </button>
      ))}
    </div>
  );
}
