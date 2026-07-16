import { glyph, icon } from "../wasm/awan_wasm";

/** The renderer's own numbers, from profile/src/gif.rs. A preview that picks
 *  its own type size is a preview that lies about what CI will produce. */
export const SCALE = 3;
export const GLYPH = 8 * SCALE;
export const CAPTION_H = 56;
/** The karaoke line is smaller and sits up beside him, not under the ground —
 *  every number from gif.rs. */
export const LYRIC_SCALE = 2;
export const LYRIC_LIMIT = 18 * 33;
/** Ticks a lyric holds before the next one, from script.rs. */
export const LYRIC_HOLD = 30;
export const INK = "#9696a0"; // [150, 150, 160] in gif.rs — convert, don't eyeball
export const ACCENT = "#e6b464"; // [230, 180, 100]

const cache = new Map<string, Uint8Array>();
const bits = (ch: string) => {
  let b = cache.get(ch);
  if (!b) cache.set(ch, (b = glyph(ch)));
  return b;
};

/** Draw an 8-row bitmap at `scale` px a pixel — `draw_bits` in draw.rs, moved
 *  across unchanged. */
export function drawBits(
  ctx: CanvasRenderingContext2D,
  rows: Uint8Array,
  x: number,
  y: number,
  scale: number,
  colour: string,
) {
  ctx.fillStyle = colour;
  rows.forEach((byte, row) => {
    for (let col = 0; col < 8; col++) {
      if (byte & (1 << col)) ctx.fillRect(x + col * scale, y + row * scale, scale, scale);
    }
  });
}

/** Text in the engine's font, glyph for glyph. Unknown characters skip but
 *  still advance, exactly as `draw_text` does. */
export function drawText(
  ctx: CanvasRenderingContext2D,
  text: string,
  x: number,
  y: number,
  scale: number,
  colour: string,
) {
  let cx = x;
  for (const ch of text) {
    const b = bits(ch);
    if (b.length) drawBits(ctx, b, cx, y, scale, colour);
    cx += 8 * scale;
  }
}

/** The narration line under the ground — icon, then text, centred. */
export function drawCaption(
  ctx: CanvasRenderingContext2D,
  name: string,
  text: string,
  w: number,
  ground: number,
) {
  const gap = SCALE * 3;
  const iconW = 8 * SCALE + gap;
  const textW = [...text].length * GLYPH;
  // Rust divides integers; JS doesn't. A half-pixel x makes the canvas
  // antialias every glyph edge, which is a third of the ink gone and a preview
  // that quietly stops matching the file CI commits.
  const x = Math.floor(Math.max(w - (iconW + textW), 0) / 2);
  const y = ground + 20;
  drawBits(ctx, icon(name), x, y, SCALE, ACCENT);
  drawText(ctx, text, x + iconW, y, SCALE, INK);
}

/** The pinned streak badge, top-right. Hidden at zero, same as the renderer. */
export function drawStreak(ctx: CanvasRenderingContext2D, streak: number, w: number) {
  if (!streak) return;
  const num = String(streak);
  const x = w - (8 * SCALE + SCALE * 2 + num.length * GLYPH + 14);
  drawBits(ctx, icon("fire"), x, 12, SCALE, ACCENT);
  drawText(ctx, num, x + 8 * SCALE + SCALE * 2, 12, SCALE, ACCENT);
}

/** One karaoke line down the left while he sings on the right.
 *
 *  Not a caption: `rasterize` picks *either* the strip under the ground *or*
 *  this, never both, and this one is half the size and up at his shoulder. The
 *  preview drew the intro line, in the caption strip, at caption size, forever
 *  — which meant the lyrics somebody had just typed never appeared at all.
 */
export function drawKaraoke(
  ctx: CanvasRenderingContext2D,
  k: number,
  ground: number,
  song: string,
  artist: string,
  lyrics: string[],
) {
  const step = Math.floor(k / LYRIC_HOLD);
  const [iconName, text] =
    step === 0
      ? ["star", `my fav song "${song || "an old favourite"}" - ${artist || "someone great"}`]
      : lyrics.length
        ? ["globe", lyrics[(step - 1) % lyrics.length]]
        : ["globe", "la la la ~"];

  const fit = Math.floor(Math.max(LYRIC_LIMIT - 24, 0) / (8 * LYRIC_SCALE));
  const y = Math.floor(ground / 2) - 4 * LYRIC_SCALE;
  drawBits(ctx, icon(iconName), 24, y, LYRIC_SCALE, ACCENT);
  drawText(ctx, [...text].slice(0, fit).join(""), 24 + 8 * LYRIC_SCALE + 6, y, LYRIC_SCALE, INK);
}
