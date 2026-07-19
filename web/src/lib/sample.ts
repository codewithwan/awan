/** Stand-in numbers for the preview.
 *
 *  The stats a profile shows are one unauthenticated REST call away, but the
 *  contribution calendar lives only in GraphQL, and GraphQL wants a token. We
 *  are not going to ask anyone to paste a token into a web page to look at a
 *  cartoon. So the preview shows a plausible year and says so, and CI fills in
 *  the real one — which it can, because it already has a token of its own.
 */
export const SAMPLE = {
  name: "your name",
  role: "your role",
  location: "where you are",
  stack: "what you build with",
  username: "your username",
  streak: 4,
  contrib_year: 2060,
  contrib_recent: 183,
};

/** Whatever the reader has typed so far — every field is optional because a
 *  half-filled form still deserves a preview. */
export type Tokens = Partial<Record<string, unknown>> & {
  song?: string;
  artist?: string;
  lyrics?: string[];
};

/** Fill the `{tokens}` a caption carries.
 *
 *  Yours first, ours only where you've left a blank. A preview that ignores
 *  what you just typed is a preview of somebody else's banner — and the numbers
 *  stay ours regardless, because those are CI's to fetch.
 */
export function fill(text: string, id?: Tokens): string {
  return text.replace(/\{(\w+)\}/g, (whole, key: string) => {
    const mine = id?.[key];
    if (typeof mine === "string" && mine.trim()) return mine;
    return key in SAMPLE ? String(SAMPLE[key as keyof typeof SAMPLE]) : whole;
  });
}

/** The readout's stand-in lines — the exact four CI writes (`repos`, `stars
 *  earned`, `followers`, `following`), in that order, so the preview's readout
 *  matches the real one. The streak is *not* one of them: it's the 🔥 badge, not
 *  a readout line, and listing it here drew a fifth row the real banner never
 *  has. Real values arrive as "label:value" from CI. */
export const SAMPLE_STATS = [
  "repos:71",
  "stars earned:82",
  "followers:42",
  "following:36",
];

/** A stand-in year: 53 weeks x 7 days, a GitHub quartile per day, -1 where the
 *  calendar has no day. Generated from a fixed seed rather than random, so the
 *  preview looks the same on every reload — a banner that changes shape while
 *  you're deciding on it is worse than a fake one. */
export const SAMPLE_WALL: number[] = (() => {
  const days: number[] = [];
  let seed = 20260716;
  const next = () => (seed = (seed * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff;
  for (let i = 0; i < 53 * 7; i++) {
    const r = next();
    // busier lately, and quieter at weekends — enough shape to read as a life
    const weekend = i % 7 === 0 || i % 7 === 6;
    const recent = i > 53 * 7 - 40;
    const lift = (recent ? 0.28 : 0) - (weekend ? 0.2 : 0);
    days.push(r + lift < 0.34 ? 0 : Math.min(4, 1 + Math.floor((r + lift) * 3.6)));
  }
  for (let i = 53 * 7 - 2; i < 53 * 7; i++) days[i] = -1; // the week isn't over
  return days;
})();
