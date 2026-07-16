import type { Scene } from "./acts";
import { fill } from "./sample";

/** The line under the ground. The wall's `then` takes over the moment the
 *  spotlight lands — tick 40 of that beat, where `glow_pct` first rises — so
 *  the preview tells the same joke at the same time CI does. */
const GLOW_AT = 40;

export function Caption({ scene, leaving, k }: { scene?: Scene; leaving: boolean; k: number }) {
  const text = leaving
    ? "thanks for stopping by ~"
    : !scene
      ? ""
      : scene.act === "sing"
        ? 'my fav song "…" — sung karaoke-style'
        : scene.then && k >= GLOW_AT
          ? scene.then
          : (scene.say ?? "");

  return (
    <div className="mt-2 flex h-7 items-center justify-center gap-2 text-sm">
      <span className="text-gold">◆</span>
      <span className="text-ink">{fill(text)}</span>
      {text && <span className="px-caret text-lime">▌</span>}
    </div>
  );
}
