import { pixelArt } from "../lib/pixels";

/** An 8×8 icon, drawn as 8×8 real squares. SVG rather than a font so it stays
 *  crisp at any size and never falls back to something the OS chose for us. */
export function PixelIcon({ id, size = 16 }: { id: string; size?: number }) {
  const { rows, colour } = pixelArt(id);
  return (
    <svg width={size} height={size} viewBox="0 0 8 8" shapeRendering="crispEdges" aria-hidden>
      {rows.flatMap((row, y) =>
        [...row].map((px, x) =>
          px === "#" ? <rect key={`${x}-${y}`} x={x} y={y} width="1" height="1" fill={colour} /> : null,
        ),
      )}
    </svg>
  );
}
