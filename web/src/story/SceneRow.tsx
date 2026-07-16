import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { CAPTION_LIMIT, actInfo, type Scene } from "../lib/acts";
import { fill } from "../lib/sample";
import { PixelIcon } from "../ui/PixelIcon";
import { Field } from "../ui/Field";

type Props = {
  id: string;
  scene: Scene;
  live: boolean;
  solo: boolean;
  onEdit: (s: Scene) => void;
  onDrop: () => void;
  onSolo: () => void;
};

/** One beat: what it is, what it costs, and what it says. Reordering and
 *  rewording are the same task, so they belong in the same place. */
export function SceneRow({ id, scene, live, solo, onEdit, onDrop, onSolo }: Props) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });
  const info = actInfo(scene.act);

  return (
    <li
      ref={setNodeRef}
      style={{ transform: CSS.Transform.toString(transform), transition }}
      className={`nb-tight bg-void p-2 ${isDragging ? "opacity-50" : ""} ${live ? "border-lime" : ""}`}
    >
      <div className="flex items-center gap-2">
        <button
          {...attributes}
          {...listeners}
          className="cursor-grab px-1 text-mute/60 active:cursor-grabbing"
          aria-label={`Reorder ${info.label}`}
        >
          ⣿
        </button>
        <PixelIcon id={scene.act} />
        <span className="text-xs uppercase text-ink">{info.label}</span>
        {info.live && <span className="bg-lime px-1 text-[9px] uppercase text-line">live</span>}
        <span className="ml-auto text-[10px] tabular-nums text-mute">{(info.ticks * 0.09).toFixed(1)}s</span>
        <button
          onClick={onSolo}
          className={`nb-btn px-1.5 py-0.5 text-[10px] ${solo ? "bg-sky text-line" : "bg-slab text-sky"}`}
          aria-label={solo ? `Play the whole story` : `Play only ${info.label}`}
          title={solo ? "back to the whole story" : "play this beat on its own"}
        >
          {solo ? "◼" : "▶"}
        </button>
        <button onClick={onDrop} className="nb-btn bg-slab px-1.5 py-0.5 text-[10px] text-punch" aria-label={`Remove ${info.label}`}>
          ✕
        </button>
      </div>

      {info.mute ? (
        <p className="mt-2 pl-8 text-[10px] text-mute">plays your lyrics — no caption of its own</p>
      ) : (
        <div className="mt-2 flex flex-col gap-1.5 pl-8">
          <Field
            value={scene.say ?? ""}
            placeholder={scene.act === "{verdict}" ? "CI writes this one" : "what he says"}
            limit={CAPTION_LIMIT}
            onChange={(say) => onEdit({ ...scene, say })}
          />
          {info.splits && (
            <Field
              value={scene.then ?? ""}
              placeholder="...and this lands when the month lights up"
              limit={CAPTION_LIMIT}
              onChange={(then) => onEdit({ ...scene, then })}
            />
          )}
          {(scene.say ?? "").includes("{") && (
            <p className="text-[10px] text-mute/70">→ {fill(scene.say ?? "")}</p>
          )}
        </div>
      )}
    </li>
  );
}
