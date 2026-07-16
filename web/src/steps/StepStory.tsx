import type { Scene } from "../lib/acts";
import type { Tokens } from "../lib/sample";
import { CAST, castOf, type Cast } from "../lib/characters";
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
  drawn?: Cast;
  onStory: (s: Scene[]) => void;
  onBeat: (i: number) => void;
  onCast: (id: string) => void;
  onSolo: (i: number) => void;
  onDraw: () => void;
};

/** The reel, and everything that changes it. The preview leads: it's the only
 *  thing here that tells you whether any of this was a good idea. */
export function StepStory({ story, beat, cast, solo, id, drawn, onStory, onBeat, onCast, onSolo, onDraw }: Props) {
  // solo plays one beat on its own — deleting the rest to see a scene means
  // rebuilding the story afterwards, which is a rotten way to look at anything
  const shown = solo >= 0 && story[solo] ? [story[solo]] : story;

  return (
    <div className="grid min-w-0 gap-4 xl:grid-cols-[1.4fr_1fr]">
      <div className="flex min-w-0 flex-col gap-4">
        <Reel story={shown} toml={castOf(cast, drawn).toml} id={id} onBeat={(i) => onBeat(solo >= 0 ? solo : i)} />
        <Meter story={story} at={beat} solo={solo} onPick={(i) => onSolo(i === solo ? -1 : i)} />
      </div>

      <div className="flex min-w-0 flex-col gap-4">
        <Card title="Who plays him" tone="text-grape-ink">
          <div className="flex flex-wrap gap-2">
            {[...CAST, ...(drawn ? [drawn] : [])].map((c) => (
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
          <div className="mt-3 border-t-3 border-line pt-3">
            <button onClick={onDraw} className="nb-btn w-full bg-sky px-3 py-2 text-[10px] uppercase text-line">
              ✎ {drawn ? "keep drawing yours" : "draw your own"}
            </button>
            <p className="mt-2 text-[10px] leading-relaxed text-faint">
              Every scene works with every character — bake, sing, juggle, nap. A character is ten
              by six pixels and two colours; the engine derives the rest. Yours travels in the zip.
            </p>
          </div>
        </Card>

        <Card title="Running order" hint="drag to reorder" tone="text-lime-ink">
          <SceneList story={story} playing={beat} onChange={onStory} />
        </Card>

        <Card title="Add a beat" tone="text-sky-ink">
          <Shelf onAdd={(s) => onStory([...story, s])} />
        </Card>
      </div>
    </div>
  );
}
