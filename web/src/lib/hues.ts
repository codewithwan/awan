/** Tailwind can't build a class from a runtime string, so the map is explicit.
 *  Both columns come from the same act data, which is what keeps a beat's
 *  timeline block the same colour as its icon. */
export const BAR: Record<string, string> = {
  gold: "bg-gold",
  punch: "bg-punch",
  sky: "bg-sky",
  lime: "bg-lime",
  grape: "bg-grape",
  cloud: "bg-cloud",
  mute: "bg-mute",
  ink: "bg-ink",
};

export const barOf = (hue: string) => BAR[hue] ?? "bg-mute";
