/** A character, as a picture and two colours — and the TOML that falls out.
 *
 *  The spec asks for six fields that look like work: `sit_rows`, `leg_frames`,
 *  `eye_row`, `mouth_row`, `legs_row`, plus the standing art. They aren't work.
 *  Both characters in the repo derive from `rows` alone, exactly, and that was
 *  checked against them rather than assumed:
 *
 *    sit_rows   = [blank] + rows[:5]        · he drops a row and loses his legs
 *    leg_frames = [legs, ←1, legs, →1]      · the whole walk cycle
 *    eye_row    = the first row with an '@'
 *    legs_row   = the last row
 *
 *  So the editor asks for a drawing, and hands back a character.
 */

export const W = 10;
export const H = 6;

/** The glyph language, in the order a brush should offer them: nothing, then
 *  progressively more of him, then the eyes. */
export const GLYPHS = [" ", "-", "+", "#", "@"] as const;
export type Glyph = (typeof GLYPHS)[number];

export const GLYPH_NAME: Record<Glyph, string> = {
  " ": "empty",
  "-": "light",
  "+": "dense",
  "#": "solid",
  "@": "eye",
};

export type Character = {
  name: string;
  author: string;
  description: string;
  body: string;
  eye: string;
  /** H rows of W glyphs. The only thing anyone actually draws. */
  rows: string[];
};

export const blankRows = () => Array.from({ length: H }, () => " ".repeat(W));

/** Shift a row one pixel and pad, so the far foot lands where it should. */
const left = (r: string) => r.slice(1) + " ";
const right = (r: string) => " " + r.slice(0, -1);

export const eyeRow = (rows: string[]) => rows.findIndex((r) => r.includes("@"));

/** Nudges the engine has no opinion about — a nameless character loads fine,
 *  it just isn't finished. Whether a spec is *valid* is the engine's call, not
 *  ours: see `check_spec`. Two sources of truth is how you end up drawing a cat
 *  and watching a cloud walk past. */
export function nits(c: Character): string[] {
  const out: string[] = [];
  if (!c.name.trim()) out.push("give him a name");
  if (c.rows.every((r) => !r.trim())) out.push("nothing drawn yet");
  return out;
}

const quote = (s: string) => `"${s.replace(/["\\]/g, "\\$&")}"`;
const rowList = (rows: string[]) => rows.map((r) => `    ${quote(r)},`).join("\n");

/** The spec file, ready to commit. */
export function toToml(c: Character): string {
  // -1 when nothing has eyes yet. Emitted as-is rather than papered over with
  // 0: the engine will say "rows[0] must contain '@' eyes", which is true and
  // useful, where a quietly-wrong 0 was neither.
  const er = eyeRow(c.rows);
  const legs = c.rows[H - 1];
  return `# ${c.name} — drawn at codewithwan.github.io/awan
# Glyph language: '#' solid · '+' dense · '-' light · '@' eye · ' ' empty.
# Face variants (blinks, glances, happy eyes, open mouth) are derived by the
# engine from the eye/mouth rows.

spec_version = 1

[character]
name = ${quote(c.name)}
author = ${quote(c.author)}
description = ${quote(c.description)}

[character.palette]
body = ${quote(c.body.toUpperCase())}
eye = ${quote(c.eye.toUpperCase())}

[sprite]
rows = [
${rowList(c.rows)}
]
sit_rows = [
${rowList([" ".repeat(W), ...c.rows.slice(0, H - 1)])}
]
leg_frames = [
${rowList([legs, left(legs), legs, right(legs)])}
]
eye_row = ${er}
mouth_row = ${Math.min(Math.max(er, 0) + 1, H - 2)}
legs_row = ${H - 1}

[personality]
blink_rate = 1.0
walk_speed = 1.0
chaos = 0.2

[reactions]
"cmd.failed" = "charred"
"task.done" = "celebrate"
"idle" = "sleep"
`;
}
