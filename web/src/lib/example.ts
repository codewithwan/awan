import type { Identity } from "./config";
import type { Scene } from "./acts";

/** One click to a filled-in page, so nobody has to invent seven captions before
 *  they can tell whether they want this at all. It's the project's own username
 *  rather than a stranger's: a placeholder that looks like a real person's
 *  details is a placeholder somebody ships by accident. */
export const EXAMPLE: Identity = {
  username: "codewithwan",
  name: "codewithwan",
  role: "fullstack engineer, crafting smooth UX",
  location: "Indonesia",
  stack: "Rust, Go & TypeScript",
  song: "your favourite song",
  artist: "the artist",
  lyrics: ["humming a tune only i can hear", "la-la, off we go again", "the melody walks me home"],
};

export const EXAMPLE_STORY: Scene[] = [
  { act: "wave", say: "hi there! i'm {name}" },
  { act: "present", say: "{role}" },
  { act: "stats", say: "the numbers, if you're curious" },
  { act: "contributions", say: "i'm very happy, {contrib_year} this year", then: "and {contrib_recent} in the last 30 days" },
  { act: "{verdict}", say: "CI decides" },
  { act: "sing" },
  { act: "sleep", say: "okay... nap time, zzz" },
];
