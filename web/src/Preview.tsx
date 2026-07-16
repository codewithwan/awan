import { useEffect, useRef, useState } from "react";
import init, { Preview as Reel } from "./wasm/awan_wasm";
import { TICK_MS, type Scene, actInfo } from "./acts";
import { Caption } from "./Caption";
import { drawStats, drawWall } from "./overlays";

/** Pixels per canvas cell — the same 33×30 the GIF renderer fills, so what you
 *  see here is what CI draws, not an impression of it. */
const CELL_W = 33;
const CELL_H = 30;

let ready: Promise<unknown> | null = null;
const loadEngine = () => (ready ??= init());

/** The reel, playing. Everything drawn on the canvas comes straight out of the
 *  engine compiled to wasm — same ticks, same cells, same loop. The caption
 *  rides underneath, because that's a renderer's job in the GIF too. */
export function Preview({ story, onBeat }: { story: Scene[]; onBeat: (i: number) => void }) {
  const canvas = useRef<HTMLCanvasElement>(null);
  const [reel, setReel] = useState<Reel | null>(null);
  const [playing, setPlaying] = useState(true);
  const [tick, setTick] = useState(0);
  const acts = story.map((s) => s.act).join(",");

  useEffect(() => {
    let dead = false;
    loadEngine().then(() => {
      if (dead || !story.length) return;
      setReel(new Reel(story.map((s) => s.act)));
      setTick(0);
    });
    return () => {
      dead = true;
    };
    // rebuilt only when the running order changes; editing a line must not
    // restart the reel under someone's cursor
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [acts]);

  useEffect(() => {
    if (!reel || !canvas.current) return;
    const ctx = canvas.current.getContext("2d");
    if (!ctx) return;
    const [cols, rows, total] = [reel.cols(), reel.rows(), reel.ticks()];
    let raf = 0;
    let last = performance.now();
    let t = tick;

    const paint = (now: number) => {
      raf = requestAnimationFrame(paint);
      if (playing && now - last >= TICK_MS) {
        last = now;
        t = (t + 1) % total;
        setTick(t);
      }
      const frame = reel.frame(playing ? t : tick);
      ctx.clearRect(0, 0, cols * CELL_W, rows * CELL_H);
      for (let i = 0; i < cols * rows; i++) {
        if (!frame[i * 4 + 3]) continue;
        ctx.fillStyle = `rgb(${frame[i * 4]},${frame[i * 4 + 1]},${frame[i * 4 + 2]})`;
        ctx.fillRect((i % cols) * CELL_W, Math.floor(i / cols) * CELL_H, CELL_W, CELL_H);
      }
      // Overlays, exactly as the GIF renderer layers them: cells, then the
      // wall, then the ground line, then the readout.
      const shown = playing ? t : tick;
      const k = reel.beat_tick(shown);
      const act = k >= 0 ? story[reel.beat_at(shown)]?.act : undefined;
      if (act === "contributions") drawWall(ctx, k);
      ctx.fillStyle = "#50545f";
      ctx.fillRect(0, rows * CELL_H - 2, cols * CELL_W, 2);
      if (act === "stats") drawStats(ctx, k);
    };
    raf = requestAnimationFrame(paint);
    return () => cancelAnimationFrame(raf);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [reel, playing, story]);

  // one source of truth for "which beat is on screen", so scrubbing highlights
  // the same row that playing does
  useEffect(() => {
    if (reel) onBeat(reel.is_leaving(tick) ? -1 : reel.beat_at(tick));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [reel, tick]);

  const total = reel?.ticks() ?? 0;
  const beat = reel && !reel.is_leaving(tick) ? reel.beat_at(tick) : -1;
  const scene = beat >= 0 ? story[beat] : undefined;

  return (
    <div className="px-box p-3">
      <canvas
        ref={canvas}
        width={32 * CELL_W}
        height={12 * CELL_H}
        className="w-full [image-rendering:pixelated] bg-void"
      />
      <Caption
        scene={scene}
        leaving={!!reel?.is_leaving(tick)}
        k={reel?.beat_tick(tick) ?? -1}
      />
      <div className="mt-3 flex items-center gap-3">
        <button
          onClick={() => setPlaying((p) => !p)}
          className="px-btn bg-lime px-3 py-1 text-void"
          aria-label={playing ? "Pause" : "Play"}
        >
          {playing ? "❚❚" : "▶"}
        </button>
        <input
          type="range"
          min={0}
          max={Math.max(total - 1, 0)}
          value={tick}
          onChange={(e) => {
            setPlaying(false);
            setTick(+e.target.value);
          }}
          className="h-2 flex-1 appearance-none rounded-none bg-edge accent-lime"
          aria-label="Scrub"
        />
        <span className="w-28 text-right text-xs text-mute tabular-nums">
          {((tick * TICK_MS) / 1000).toFixed(1)}s / {((total * TICK_MS) / 1000).toFixed(0)}s
        </span>
      </div>
      <p className="mt-2 text-xs text-mute">
        {scene ? (
          <>
            now playing <span className="text-ink">{actInfo(scene.act).label}</span>
          </>
        ) : (
          "walking on"
        )}
        {" · "}the engine itself, compiled to wasm — these are the frames CI draws
      </p>
    </div>
  );
}
