import * as w from "../wasm/awan_wasm";
import { SAMPLE_STATS, SAMPLE_WALL } from "../lib/sample";
import { drawText, GLYPH, INK, SCALE } from "./text";

/** Pixels per canvas cell — the GIF renderer's own numbers. */
const CELL_W = 33;
const CELL_H = 30;
const BG: [number, number, number] = [13, 17, 23];

/** GitHub's five contribution shades, quietest first. */
const SHADES = ["#161b22", "#0e4429", "#006d32", "#26a641", "#39d353"];
const PITCH = 18;
const SQUARE = 14;
const SPOT = [32, 40, 52] as const;
const YEAR_FADE = 45;

const mix = (a: readonly number[], b: readonly number[], pct: number) =>
  `rgb(${a.map((v, i) => Math.round((v * (100 - pct) + b[i] * pct) / 100)).join(",")})`;

/** The readout, typing itself into the window the engine opened — in the
 *  engine's own font, at the renderer's own scale. The engine says how much has
 *  printed; the words are the reader's, so they're ours to draw. */
export function drawStats(ctx: CanvasRenderingContext2D, k: number) {
  const [px, py, pw, ph] = w.stats_panel();
  const innerW = (pw - 2) * CELL_W;
  const innerH = (ph - 2) * CELL_H;
  const room = Math.max(Math.floor(innerW / GLYPH) - 2, 8);
  const x = px * CELL_W + CELL_W + Math.floor((innerW - room * GLYPH) / 2);
  const step = GLYPH + 12;
  const slots = w.stats_slots();
  const y0 = py * CELL_H + CELL_H + Math.floor(Math.max(innerH - ((slots - 1) * step + GLYPH), 0) / 2);

  SAMPLE_STATS.slice(0, slots).forEach((entry, i) => {
    const shown = w.stats_chars_at(k, i);
    if (!shown) return;
    const [label, value] = entry.split(":");
    const gap = Math.max(room - label.length - value.length - 1, 0);
    const line = `${label}${".".repeat(gap)} ${value}`.slice(0, shown);
    const y = y0 + i * step;
    drawText(ctx, line, x, y, SCALE, INK);
    if (w.stats_typing(k, i)) {
      ctx.fillStyle = INK;
      ctx.fillRect(x + line.length * GLYPH, y, GLYPH / 2, GLYPH);
    }
  });
}

/** The contribution year, rising behind him. Every square is real geometry —
 *  18px pitch, not one flat colour per cell — because that difference is the
 *  whole reason the wall reads as a calendar rather than a smear. */
export function drawWall(ctx: CanvasRenderingContext2D, k: number) {
  const up = w.wall_fade(k);
  if (!up) return;
  const [bx, by, bw, bh] = w.wall_band();

  // Sink the band toward the page before drawing a single square. The engine
  // stopped clearing this patch of sky when the wall started fading in — the
  // fade *is* the clearing — so without this the clouds drift straight through
  // the gaps between days, and a calendar you can see weather behind reads as
  // broken rather than atmospheric.
  veil(ctx, bx * CELL_W, by * CELL_H, bw * CELL_W, bh * CELL_H, up);
  const [weeks, rows, recent] = w.wall_shape();
  const glow = w.wall_glow(k);

  const x0 = Math.floor((32 * CELL_W - weeks * PITCH) / 2);
  const y0 = by * CELL_H + Math.floor((bh * CELL_H - rows * PITCH) / 2);
  const first = Math.floor((SAMPLE_WALL.length - recent) / rows);

  if (glow) {
    ctx.fillStyle = mix(BG, SPOT, (glow * up) / 100);
    ctx.fillRect(x0 + first * PITCH - 5, y0 - 5, (weeks - first) * PITCH + 5, rows * PITCH + 5);
  }
  for (let c = 0; c < weeks; c++) {
    for (let d = 0; d < rows; d++) {
      const level = SAMPLE_WALL[c * rows + d];
      if (level < 0) continue; // a day the calendar doesn't cover
      const old = c * rows + d < SAMPLE_WALL.length - recent;
      const base = hexToRgb(SHADES[level]);
      const stepped = old ? rgb(mix(base, BG, (glow * YEAR_FADE) / 100)) : base;
      ctx.fillStyle = mix(BG, stepped, up);
      ctx.fillRect(x0 + c * PITCH, y0 + d * PITCH, SQUARE, SQUARE);
    }
  }
}

/** Mix a whole region `pct` of the way to the page background — `veil` in
 *  wall.rs, moved across unchanged. */
function veil(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, pct: number) {
  const img = ctx.getImageData(x, y, w, h);
  const d = img.data;
  for (let i = 0; i < d.length; i += 4) {
    d[i] = (d[i] * (100 - pct) + BG[0] * pct) / 100;
    d[i + 1] = (d[i + 1] * (100 - pct) + BG[1] * pct) / 100;
    d[i + 2] = (d[i + 2] * (100 - pct) + BG[2] * pct) / 100;
  }
  ctx.putImageData(img, x, y);
}

const hexToRgb = (h: string): [number, number, number] => [
  parseInt(h.slice(1, 3), 16),
  parseInt(h.slice(3, 5), 16),
  parseInt(h.slice(5, 7), 16),
];

const rgb = (s: string): [number, number, number] =>
  s.match(/\d+/g)!.slice(0, 3).map(Number) as [number, number, number];
