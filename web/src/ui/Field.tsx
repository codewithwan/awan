/** One labelled input. The counter is the only rule the renderer enforces:
 *  past ~42 glyphs a caption runs off the 1056px canvas, and finding that out
 *  from CI a day later is worse than seeing it while you type. */
export function Field({
  label,
  value,
  placeholder,
  limit,
  onChange,
}: {
  label?: string;
  value: string;
  placeholder?: string;
  limit?: number;
  onChange: (v: string) => void;
}) {
  const over = limit !== undefined && value.length > limit;
  return (
    <label className="flex items-center gap-2">
      {label && <span className="w-20 shrink-0 text-[10px] uppercase text-mute">{label}</span>}
      <input
        value={value}
        placeholder={placeholder}
        onChange={(e) => onChange(e.target.value)}
        className={`nb-input min-w-0 flex-1 px-2 py-1.5 text-xs text-ink placeholder:text-faint
          ${over ? "border-punch" : ""}`}
      />
      {limit !== undefined && (
        <span className={`w-10 text-right text-[10px] tabular-nums ${over ? "text-punch-ink" : "text-faint"}`}>
          {value.length}/{limit}
        </span>
      )}
    </label>
  );
}
