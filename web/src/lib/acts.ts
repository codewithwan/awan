/** One beat of the story, exactly as it lands in `awan.json`. */
export type Scene = { act: string; say?: string; then?: string };

/** What an act is, for people picking one off a shelf. */
export type ActInfo = {
  id: string;
  label: string;
  blurb: string;
  /** Which of the engine's 8×8 caption icons this beat carries. */
  caption: string;
  /** Its colour on the timeline, matching its icon — the bar should read as
   *  the story, not as a progress meter. */
  hue: string;
  /** Ticks, straight from the engine's `scene_for`. 11 ticks ≈ 1 second. */
  ticks: number;
  /** Whether the act reads numbers CI fetches, rather than words you write. */
  live?: boolean;
  /** A `then` line takes the caption over mid-beat. Only the wall has a moment
   *  worth splitting on: the year as it rises, the month as it lights. */
  splits?: boolean;
  /** Beats with nothing to say — `sing` plays your lyrics instead. */
  mute?: boolean;
};

export const TICK_MS = 90;

/** The shelf. Durations mirror `scene_for` in awan-core; if that changes, this
 *  lies, so the preview clock is the thing that would catch it. */
export const ACTS: ActInfo[] = [
  { id: "wave", hue: "gold", caption: "heart", label: "Wave", blurb: "bounces in an excited hello", ticks: 30 },
  { id: "present", hue: "punch", caption: "briefcase", label: "Present", blurb: "stands and introduces himself", ticks: 60 },
  { id: "stroll", hue: "sky", caption: "pin", label: "Stroll", blurb: "walks along, ground scrolling past", ticks: 30 },
  { id: "stats", hue: "lime", caption: "diamond", label: "Stats", blurb: "types your numbers into a terminal", ticks: 150, live: true },
  { id: "contributions", hue: "lime", caption: "code", label: "Year wall", blurb: "walks his contribution year", ticks: 150, live: true, splits: true },
  { id: "rocket", hue: "mute", caption: "code", label: "Rocket", blurb: "builds a rocket", ticks: 40 },
  { id: "launch", hue: "punch", caption: "star", label: "Launch", blurb: "...and watches it explode", ticks: 50 },
  { id: "bake", hue: "gold", caption: "heart", label: "Bake", blurb: "fetches an oven, bakes, devours", ticks: 118 },
  { id: "campfire", hue: "punch", caption: "fire", label: "Campfire", blurb: "drags in wood, the fire catches", ticks: 90 },
  { id: "sing", hue: "grape", caption: "globe", label: "Sing", blurb: "karaoke — plays your lyrics", ticks: 150, mute: true },
  { id: "soccer", hue: "ink", caption: "star", label: "Soccer", blurb: "juggles until it bonks him", ticks: 66 },
  { id: "dance", hue: "grape", caption: "star", label: "Dance", blurb: "a little dance", ticks: 48 },
  { id: "sleep", hue: "cloud", caption: "heart", label: "Sleep", blurb: "yawns, dozes, wakes up", ticks: 80 },
  { id: "{verdict}", hue: "gold", caption: "star", label: "Verdict", blurb: "CI picks: dance if the month was good, sleep if not", ticks: 48 },
];

export const actInfo = (id: string): ActInfo =>
  ACTS.find((a) => a.id === id) ?? { id, hue: "mute", caption: "diamond", label: id, blurb: "", ticks: 60 };

/** The caption icon a beat carries — the engine's own bitmap, so the strip
 *  under the ground matches the GIF glyph for glyph. */
export const actIcon = (id?: string): string => (id ? actInfo(id).caption : "heart");

/** Placeholders CI fills in. Shown so nobody wonders where the numbers come
 *  from — and so a preview can stand in for them. */
export const TOKENS: Record<string, string> = {
  "{name}": "your name",
  "{role}": "your role",
  "{location}": "where you are",
  "{stack}": "what you build with",
  "{username}": "your GitHub username",
  "{streak}": "days in a row, counted from your calendar",
  "{contrib_year}": "contributions this year",
  "{contrib_recent}": "contributions in the last 30 days",
};

/** The story we open with — the one from the sample, which is the one we'd
 *  defend. Reordering it is the whole point of the page. */
export const DEFAULT_STORY: Scene[] = [
  { act: "wave", say: "hi there! i'm {name}" },
  { act: "present", say: "{role}" },
  { act: "stats", say: "the numbers, if you're curious" },
  { act: "contributions", say: "i'm very happy, {contrib_year} this year", then: "and {contrib_recent} in the last 30 days" },
  { act: "{verdict}", say: "CI decides" },
  { act: "sing" },
  { act: "sleep", say: "okay... nap time, zzz" },
];

/** The caption is drawn at 8×3 px a glyph across a 1056px canvas, so it runs
 *  off the edge past this. Worth saying out loud while you type, not after CI
 *  has already cropped it. */
export const CAPTION_LIMIT = 42;
