/** One colour of him. Two is the whole palette: body and eye. The engine
 *  derives the rest — the thinner shades are the body with less of it, and the
 *  charred/blink/happy faces come out of the eye row. */
export function Swatch({
  label,
  value,
  onChange,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
}) {
  return (
    <label className="flex items-center gap-2">
      <span className="w-10 text-[10px] uppercase text-mute">{label}</span>
      <input
        type="color"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="nb-tight h-7 w-10 cursor-pointer bg-void p-0.5"
        aria-label={`${label} colour`}
      />
      <input
        value={value.toUpperCase()}
        onChange={(e) => onChange(e.target.value)}
        className="nb-input w-24 px-2 py-1 text-[10px] text-ink"
        aria-label={`${label} hex`}
      />
    </label>
  );
}
