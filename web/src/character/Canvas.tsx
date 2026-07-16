import { useRef, useState } from "react";
import { GLYPHS, H, W, type Character, type Glyph } from "../lib/spec";

/** Each glyph, painted the way the engine paints it: the body colour, thinned.
 *  `-` and `+` aren't separate colours, they're less of him — so the swatch
 *  can't be a fixed grey without lying about what you're drawing. */
const ALPHA: Record<Glyph, number> = { " ": 0, "-": 0.38, "+": 0.68, "#": 1, "@": 1 };

export const paintOf = (g: Glyph, body: string, eye: string) =>
  g === "@" ? eye : g === " " ? "transparent" : body;

type Props = { char: Character; brush: Glyph; onRows: (rows: string[]) => void };

/** The 10×6 he's made of. Drag to paint; right-drag to erase.
 *
 *  Ten by six is not a limitation to work around — it's the whole grammar. The
 *  engine blows each pixel up to a 33×30 block, so a stray dot is a brick. */
export function Canvas({ char, brush, onRows }: Props) {
  const down = useRef<Glyph | null>(null);
  const [hover, setHover] = useState<string | null>(null);

  const put = (x: number, y: number, g: Glyph) => {
    if (char.rows[y][x] === g) return;
    onRows(char.rows.map((row, i) => (i === y ? row.slice(0, x) + g + row.slice(x + 1) : row)));
  };

  return (
    <div
      className="nb-tight inline-block bg-void p-2 select-none"
      onPointerUp={() => (down.current = null)}
      onPointerLeave={() => {
        down.current = null;
        setHover(null);
      }}
      onContextMenu={(e) => e.preventDefault()}
    >
      <div className="grid gap-px" style={{ gridTemplateColumns: `repeat(${W}, minmax(0, 1fr))` }}>
        {Array.from({ length: H }).flatMap((_, y) =>
          Array.from({ length: W }).map((_, x) => {
            const g = char.rows[y][x] as Glyph;
            return (
              <button
                key={`${x}-${y}`}
                // a touch that starts on a pixel means paint, never scroll
                style={{
                  touchAction: "none",
                  aspectRatio: "1",
                  background: paintOf(g, char.body, char.eye),
                  opacity: ALPHA[g] || undefined,
                }}
                className={`min-w-5 border ${
                  hover === `${x}-${y}` ? "border-sky-ink" : "border-line/40"
                } ${g === " " ? "bg-slab/40" : ""}`}
                onPointerDown={(e) => {
                  e.preventDefault();
                  down.current = e.button === 2 ? " " : brush;
                  put(x, y, down.current);
                }}
                onPointerEnter={() => {
                  setHover(`${x}-${y}`);
                  if (down.current) put(x, y, down.current);
                }}
                aria-label={`pixel ${x + 1}, ${y + 1}`}
              />
            );
          }),
        )}
      </div>
    </div>
  );
}

/** The brush. Ordered the way you'd reach for them: nothing, then more of him. */
export function Brushes({
  brush,
  char,
  onPick,
}: {
  brush: Glyph;
  char: Character;
  onPick: (g: Glyph) => void;
}) {
  return (
    <div className="flex flex-wrap gap-2">
      {GLYPHS.map((g) => (
        <button
          key={g}
          onClick={() => onPick(g)}
          className={`nb-btn flex items-center gap-2 px-2 py-1 text-[10px] uppercase ${
            brush === g ? "bg-sky text-line" : "bg-slab text-ink"
          }`}
        >
          <span
            className="inline-block h-3 w-3 border border-line"
            style={{
              background: paintOf(g, char.body, char.eye),
              opacity: ALPHA[g] || undefined,
            }}
          />
          {g === " " ? "erase" : g}
        </button>
      ))}
    </div>
  );
}
