import { DndContext, PointerSensor, closestCenter, useSensor, useSensors } from "@dnd-kit/core";
import type { DragEndEvent } from "@dnd-kit/core";
import { SortableContext, arrayMove, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { CAPTION_LIMIT, type Scene, actInfo } from "./acts";
import { fill } from "./sample";

type Props = {
  story: Scene[];
  playing: number;
  onChange: (story: Scene[]) => void;
};

/** The running order, draggable. Everything a beat carries is edited in place:
 *  reordering and rewording are the same task, so they shouldn't live in two
 *  places. */
export function SceneList({ story, playing, onChange }: Props) {
  const sensors = useSensors(useSensor(PointerSensor, { activationConstraint: { distance: 4 } }));
  const ids = story.map((_, i) => String(i));

  const onDragEnd = ({ active, over }: DragEndEvent) => {
    if (!over || active.id === over.id) return;
    onChange(arrayMove(story, +active.id, +over.id));
  };

  return (
    <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={onDragEnd}>
      <SortableContext items={ids} strategy={verticalListSortingStrategy}>
        <ol className="flex flex-col gap-2">
          {story.map((scene, i) => (
            <Row
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

type RowProps = {
  id: string;
  scene: Scene;
  live: boolean;
  onEdit: (s: Scene) => void;
  onDrop: () => void;
};

function Row({ id, scene, live, onEdit, onDrop }: RowProps) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });
  const info = actInfo(scene.act);

  return (
    <li
      ref={setNodeRef}
      style={{ transform: CSS.Transform.toString(transform), transition }}
      className={`px-box p-2 ${isDragging ? "opacity-60" : ""} ${live ? "border-lime" : ""}`}
    >
      <div className="flex items-center gap-2">
        <button
          {...attributes}
          {...listeners}
          className="cursor-grab px-1 text-mute active:cursor-grabbing"
          aria-label={`Reorder ${info.label}`}
        >
          ⠿
        </button>
        <span aria-hidden>{info.icon}</span>
        <span className="text-sm text-ink">{info.label}</span>
        {info.live && <Tag className="bg-lime text-void">live</Tag>}
        <span className="ml-auto text-xs text-mute tabular-nums">
          {(info.ticks * 0.09).toFixed(1)}s
        </span>
        <button
          onClick={onDrop}
          className="px-btn bg-panel px-2 text-xs text-punch"
          aria-label={`Remove ${info.label}`}
        >
          ✕
        </button>
      </div>

      {info.mute ? (
        <p className="mt-2 pl-7 text-xs text-mute">plays your lyrics — no caption of its own</p>
      ) : (
        <div className="mt-2 flex flex-col gap-1 pl-7">
          <Line
            value={scene.say ?? ""}
            placeholder={scene.act === "{verdict}" ? "CI writes this one" : "what he says"}
            onChange={(say) => onEdit({ ...scene, say })}
          />
          {info.splits && (
            <Line
              value={scene.then ?? ""}
              placeholder="…and this lands when the month lights up"
              onChange={(then) => onEdit({ ...scene, then })}
            />
          )}
        </div>
      )}
    </li>
  );
}

/** One caption line, with the only rule the renderer actually enforces: past
 *  ~42 characters it runs off the canvas. Better to know while typing. */
function Line({
  value,
  placeholder,
  onChange,
}: {
  value: string;
  placeholder: string;
  onChange: (v: string) => void;
}) {
  const len = fill(value).length;
  const over = len > CAPTION_LIMIT;
  return (
    <label className="flex items-center gap-2">
      <input
        value={value}
        placeholder={placeholder}
        onChange={(e) => onChange(e.target.value)}
        className={`min-w-0 flex-1 border-2 bg-void px-2 py-1 text-xs text-ink outline-none
          placeholder:text-edge focus:border-sky ${over ? "border-punch" : "border-edge"}`}
      />
      <span className={`w-12 text-right text-xs tabular-nums ${over ? "text-punch" : "text-edge"}`}>
        {len}/{CAPTION_LIMIT}
      </span>
    </label>
  );
}

function Tag({ children, className }: { children: React.ReactNode; className: string }) {
  return <span className={`px-1 text-[10px] uppercase ${className}`}>{children}</span>;
}
