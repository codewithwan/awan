import { useEffect, useState } from "react";
import { loadEngine, type Preview } from "../lib/engine";
import { Preview as Engine } from "../wasm/awan_wasm";
import { TICK_MS, type Scene } from "../lib/acts";
import type { Tokens } from "../lib/sample";
import { Stage } from "./Stage";
import { Transport } from "./Transport";

/** The reel: build it, run its clock, hand each tick to the stage. Split from
 *  Stage so that one only ever has to paint. */
export function Reel({
  story,
  toml,
  id,
  onBeat,
}: {
  story: Scene[];
  toml: string;
  id: Tokens;
  onBeat: (i: number) => void;
}) {
  const [reel, setReel] = useState<Preview | null>(null);
  const [playing, setPlaying] = useState(true);
  const [tick, setTick] = useState(0);
  const order = story.map((s) => s.act).join(",");

  useEffect(() => {
    let dead = false;
    loadEngine().then(() => {
      if (dead || !story.length) return;
      setReel(new Engine(story.map((s) => s.act), toml || undefined));
      setTick(0);
    });
    return () => void (dead = true);
    // rebuilt on running order only: editing a line must not restart the reel
    // under someone's cursor
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [order, toml]);

  useEffect(() => {
    if (!reel || !playing) return;
    const total = reel.ticks();
    const id = setInterval(() => setTick((t) => (t + 1) % total), TICK_MS);
    return () => clearInterval(id);
  }, [reel, playing]);

  useEffect(() => {
    if (reel) onBeat(reel.is_leaving(tick) ? -1 : reel.beat_at(tick));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [reel, tick]);

  if (!reel || !story.length) {
    return (
      <div className="nb grid h-64 place-items-center p-4 text-xs text-mute">
        {story.length ? "waking him up..." : "add a beat to see him"}
      </div>
    );
  }

  return (
    <div className="nb min-w-0 p-3">
      <Stage reel={reel} story={story} tick={tick} id={id} />
      <Transport
        tick={tick}
        total={reel.ticks()}
        playing={playing}
        onPlay={() => setPlaying((p) => !p)}
        onScrub={(t) => {
          setPlaying(false);
          setTick(t);
        }}
      />
    </div>
  );
}
