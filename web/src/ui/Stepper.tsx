export const STEPS = ["who you are", "the story", "take it home"] as const;

/** Three steps, because one page of everything is how the last version buried
 *  the preview under a form. Steps are clickable both ways: nothing here is
 *  destructive, so there's no reason to trap anyone going back. */
export function Stepper({ at, onGo }: { at: number; onGo: (i: number) => void }) {
  return (
    <ol className="flex flex-wrap items-stretch gap-2">
      {STEPS.map((label, i) => {
        const state = i === at ? "bg-lime text-line" : i < at ? "bg-slab text-lime-ink" : "bg-slab text-mute";
        return (
          <li key={label} className="flex items-center gap-2">
            <button onClick={() => onGo(i)} className={`nb-btn px-3 py-2 text-xs uppercase ${state}`}>
              <span className="tabular-nums">{i + 1}</span>
              <span className="ml-2 hidden sm:inline">{label}</span>
            </button>
            {i < STEPS.length - 1 && <span className="text-faint">›</span>}
          </li>
        );
      })}
    </ol>
  );
}
