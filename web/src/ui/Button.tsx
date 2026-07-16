import type { ButtonHTMLAttributes } from "react";

type Tone = "lime" | "sky" | "gold" | "punch" | "slab";

const TONES: Record<Tone, string> = {
  lime: "bg-lime text-line",
  sky: "bg-sky text-line",
  gold: "bg-gold text-line",
  punch: "bg-punch text-line",
  slab: "bg-slab text-ink",
};

export function Button({
  tone = "slab",
  className = "",
  ...rest
}: { tone?: Tone } & ButtonHTMLAttributes<HTMLButtonElement>) {
  return <button {...rest} className={`nb-btn px-3 py-1.5 text-xs uppercase ${TONES[tone]} ${className}`} />;
}
