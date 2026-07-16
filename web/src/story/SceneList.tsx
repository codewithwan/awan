import { DndContext, PointerSensor, closestCenter, useSensor, useSensors } from "@dnd-kit/core";
import type { DragEndEvent } from "@dnd-kit/core";
import { SortableContext, arrayMove, verticalListSortingStrategy } from "@dnd-kit/sortable";
import type { Scene } from "../lib/acts";
import { SceneRow } from "./SceneRow";

export function SceneList({
  story,
  playing,
  onChange,
}: {
  story: Scene[];
  playing: number;
  onChange: (s: Scene[]) => void;
}) {
  const sensors = useSensors(useSensor(PointerSensor, { activationConstraint: { distance: 4 } }));
  const onDragEnd = ({ active, over }: DragEndEvent) => {
    if (over && active.id !== over.id) onChange(arrayMove(story, +active.id, +over.id));
  };

  return (
    <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={onDragEnd}>
      <SortableContext items={story.map((_, i) => String(i))} strategy={verticalListSortingStrategy}>
        <ol className="flex flex-col gap-2">
          {story.map((scene, i) => (
            <SceneRow
              key={i}
              id={String(i)}
              scene={scene}
              live={i === playing}
              onEdit={(next) => onChange(story.map((s, j) => (i === j ? next : s)))}
              onDrop={() => onChange(story.filter((_, j) => j !== i))}
            />
          ))}
        </ol>
      </SortableContext>
    </DndContext>
  );
}
