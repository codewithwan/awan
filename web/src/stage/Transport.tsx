import { TICK_MS } from "../lib/acts";
import { Button } from "../ui/Button";

export function Transport({
  tick,
  total,
  playing,
  onPlay,
  onScrub,
}: {
  tick: number;
  total: number;
  playing: boolean;
  onPlay: () => void;
  onScrub: (t: number) => void;
}) {
  return (
    <div className="mt-3 flex items-center gap-3">
      <Button tone="lime" onClick={onPlay} aria-label={playing ? "Pause" : "Play"} className="w-12">
        {playing ? "❚❚" : "▶"}
      </Button>
      <input
        type="range"
        min={0}
        max={Math.max(total - 1, 0)}
        value={tick}
        onChange={(e) => onScrub(+e.target.value)}
        className="h-3 flex-1 appearance-none border-2 border-line bg-void accent-lime"
        aria-label="Scrub the reel"
      />
      <span className="w-24 text-right text-[10px] tabular-nums text-mute">
        {((tick * TICK_MS) / 1000).toFixed(1)}s / {((total * TICK_MS) / 1000).toFixed(0)}s
      </span>
    </div>
  );
}
