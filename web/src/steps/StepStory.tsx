import type { Scene } from "../lib/acts";
import { Reel } from "../stage/Reel";
import { Meter } from "../stage/Meter";
import { SceneList } from "../story/SceneList";
import { Shelf } from "../story/Shelf";
import { Card } from "../ui/Card";

/** The reel, and the two things that change it. The preview leads: it's the
 *  only thing here that tells you whether any of this was a good idea. */
export function StepStory({
  story,
  beat,
  onStory,
  onBeat,
}: {
  story: Scene[];
  beat: number;
  onStory: (s: Scene[]) => void;
  onBeat: (i: number) => void;
}) {
  return (
    <div className="grid gap-4 xl:grid-cols-[1.4fr_1fr]">
      <div className="flex flex-col gap-4">
        <Reel story={story} onBeat={onBeat} />
        <Meter story={story} at={beat} />
      </div>
      <div className="flex flex-col gap-4">
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
