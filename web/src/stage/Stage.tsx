import { useEffect, useRef } from "react";
import type { Preview } from "../lib/engine";
import type { Scene } from "../lib/acts";
import type { Tokens } from "../lib/sample";
import { actIcon } from "../lib/acts";
import { fill } from "../lib/sample";
import { drawCaption, drawKaraoke, drawStreak, CAPTION_H } from "./text";
import { drawStats, drawWall } from "./overlays";
import { SAMPLE } from "../lib/sample";

/** Pixels per canvas cell, and the tick the wall's second line lands on —
 *  every number here is the GIF renderer's, because the whole point is that
 *  this canvas *is* the GIF, not a picture of one. */
const CELL_W = 33;
const CELL_H = 30;
const GLOW_AT = 40;
const GROUND = "#505460"; // [80, 84, 96] in gif.rs

type Props = { reel: Preview; story: Scene[]; tick: number; id: Tokens };

/** One frame of the reel, painted the way the encoder paints it: cells, wall,
 *  ground line, readout, badge, caption. Same order, same font, same 1056×416
 *  as the file CI commits. */
export function Stage({ reel, story, tick, id }: Props) {
  const ref = useRef<HTMLCanvasElement>(null);
  const cols = reel.cols();
  const rows = reel.rows();

  useEffect(() => {
    const ctx = ref.current?.getContext("2d");
    if (!ctx) return;
    const w = cols * CELL_W;
    const ground = rows * CELL_H;

    ctx.fillStyle = "#0d1117";
    ctx.fillRect(0, 0, w, ground + CAPTION_H);

    const frame = reel.frame(tick);
    for (let i = 0; i < cols * rows; i++) {
      if (!frame[i * 4 + 3]) continue;
      ctx.fillStyle = `rgb(${frame[i * 4]},${frame[i * 4 + 1]},${frame[i * 4 + 2]})`;
      ctx.fillRect((i % cols) * CELL_W, Math.floor(i / cols) * CELL_H, CELL_W, CELL_H);
    }

    const leaving = reel.is_leaving(tick);
    const beat = leaving ? -1 : reel.beat_at(tick);
    const k = reel.beat_tick(tick);
    const scene = beat >= 0 ? story[beat] : undefined;

    if (scene?.act === "contributions") drawWall(ctx, k);
    ctx.fillStyle = GROUND;
    ctx.fillRect(0, ground - 2, w, 2);
    if (scene?.act === "stats") drawStats(ctx, k);
    else drawStreak(ctx, SAMPLE.streak, w);

    // the renderer picks one or the other, never both
    if (scene?.act === "sing" && !leaving) {
      drawKaraoke(ctx, k, ground, id.song ?? "", id.artist ?? "", id.lyrics ?? []);
    } else {
      drawCaption(ctx, actIcon(scene?.act), captionOf(scene, k, leaving, id), w, ground);
    }
  }, [reel, story, tick, cols, rows, id]);

  return (
    <canvas
      ref={ref}
      width={cols * CELL_W}
      height={rows * CELL_H + CAPTION_H}
      className="stage block w-full max-w-full [image-rendering:pixelated]"
      aria-label="Preview of your banner"
    />
  );
}

/** Which of a beat's lines is speaking. The wall's `then` takes over the tick
 *  the spotlight lands, so the preview tells the joke on the same beat CI does. */
function captionOf(scene: Scene | undefined, k: number, leaving: boolean, id: Tokens): string {
  if (leaving) return "thanks for stopping by ~";
  if (!scene) return "";
  const line = scene.then && k >= GLOW_AT ? scene.then : (scene.say ?? "");
  return fill(line, id);
}
