import type { Scene } from "../lib/acts";
import type { Tokens } from "../lib/sample";
import { CAST, castOf } from "../lib/characters";
import { Reel } from "../stage/Reel";
import { Meter } from "../stage/Meter";
import { SceneList } from "../story/SceneList";
import { Shelf } from "../story/Shelf";
import { Card } from "../ui/Card";

type Props = {
  story: Scene[];
  beat: number;
  cast: string;
  solo: number;
  id: Tokens;
  onStory: (s: Scene[]) => void;
  onBeat: (i: number) => void;
  onCast: (id: string) => void;
  onSolo: (i: number) => void;
};

/** The reel, and everything that changes it. The preview leads: it's the only
 *  thing here that tells you whether any of this was a good idea. */
export function StepStory({ story, beat, cast, solo, id, onStory, onBeat, onCast, onSolo }: Props) {
  // solo plays one beat on its own — deleting the rest to see a scene means
  // rebuilding the story afterwards, which is a rotten way to look at anything
  const shown = solo >= 0 && story[solo] ? [story[solo]] : story;

  return (
    <div className="grid gap-4 xl:grid-cols-[1.4fr_1fr]">
      <div className="flex flex-col gap-4">
        <Reel story={shown} toml={castOf(cast).toml} id={id} onBeat={(i) => onBeat(solo >= 0 ? solo : i)} />
        <Meter story={story} at={beat} solo={solo} onPick={(i) => onSolo(i === solo ? -1 : i)} />
      </div>

      <div className="flex flex-col gap-4">
        <Card title="Who plays him" tone="text-grape">
          <div className="flex flex-wrap gap-2">
            {CAST.map((c) => (
              <button
                key={c.id}
                onClick={() => onCast(c.id)}
                className={`nb-btn px-3 py-1.5 text-left ${cast === c.id ? "bg-grape text-line" : "bg-void text-ink"}`}
              >
                <div className="text-[10px] uppercase">{c.label}</div>
                <div className="text-[9px] opacity-70">{c.blurb}</div>
              </button>
            ))}
          </div>
          <p className="mt-2 text-[10px] text-mute">
            Every scene works with every character. Pick one and the whole reel restyles — adding to
            the cast is TOML only.
          </p>
        </Card>

        <Card title="Running order" hint="drag to reorder" tone="text-lime">
          <SceneList story={story} playing={beat} onChange={onStory} />
        </Card>

        <Card title="Add a beat" tone="text-sky">
          <Shelf onAdd={(s) => onStory([...story, s])} />
        </Card>
      </div>
    </div>
  );
}
