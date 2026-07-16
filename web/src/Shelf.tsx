import { ACTS, type Scene } from "./acts";

/** The shelf of acts you can add. Duration sits on every card on purpose: a
 *  reel gets too long one innocent-looking beat at a time, and nobody feels
 *  that cost unless the price is on the label. */
export function Shelf({ onAdd }: { onAdd: (s: Scene) => void }) {
  return (
    <div className="grid grid-cols-2 gap-2 sm:grid-cols-3">
      {ACTS.map((a) => (
        <button
          key={a.id}
          onClick={() => onAdd({ act: a.id, say: "" })}
          className="px-btn group bg-panel p-2 text-left"
        >
          <div className="flex items-center gap-2">
            <span aria-hidden>{a.icon}</span>
            <span className="text-xs text-ink">{a.label}</span>
            <span className="ml-auto text-[10px] text-edge tabular-nums group-hover:text-mute">
              {(a.ticks * 0.09).toFixed(1)}s
            </span>
          </div>
          <p className="mt-1 text-[10px] leading-tight text-mute">{a.blurb}</p>
        </button>
      ))}
    </div>
  );
}
