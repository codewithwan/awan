import type { ReactNode } from "react";

/** One slab. Deliberately not nestable: a card inside a card is two borders and
 *  two shadows saying the same thing, and it reads as a mistake. If something
 *  inside needs separating, separate it with space or a rule. */
export function Card({
  title,
  hint,
  tone = "text-gold-ink",
  children,
}: {
  title?: string;
  hint?: string;
  tone?: string;
  children: ReactNode;
}) {
  return (
    <section className="nb min-w-0 p-4">
      {title && (
        <div className="mb-4 flex items-baseline gap-2">
          <h2 className={`text-sm uppercase ${tone}`}>{title}</h2>
          {hint && <span className="text-[10px] text-mute normal-case">{hint}</span>}
        </div>
      )}
      {children}
    </section>
  );
}
