import {
  DndContext,
  KeyboardSensor,
  PointerSensor,
  TouchSensor,
  closestCenter,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import type { DragEndEvent } from "@dnd-kit/core";
import {
  SortableContext,
  arrayMove,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
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
  // A finger that starts on the handle has to mean "drag", not "scroll" — and
  // a phone can't tell until you've already moved, by which point the page has
  // gone. The delay is the tell: hold, then drag. Move first and it scrolls,
  // which is what a finger on a long page usually wants.
  //
  // Keyboard too. This is a list you reorder; a list you can only reorder by
  // dragging is a list some people can't reorder.
  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 4 } }),
    useSensor(TouchSensor, { activationConstraint: { delay: 180, tolerance: 8 } }),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates }),
  );
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
