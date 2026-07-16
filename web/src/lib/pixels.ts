/** 8×8 icons for the act shelf, drawn rather than borrowed.
 *
 *  System emoji were the first thing here and they were wrong: they arrive in
 *  whatever style the reader's OS ships, at whatever weight, anti-aliased,
 *  next to a character made of hard 33px squares. Two art directions in one
 *  row, and neither of them ours.
 *
 *  These are written as pictures so they can be edited as pictures. `#` is on,
 *  `.` is off, and the row order is the same as the engine's icons — top down.
 */
export type Art = { rows: string[]; colour: string };

const art = (colour: string, ...rows: string[]): Art => ({ rows, colour });

export const PIXEL_ART: Record<string, Art> = {
  wave: art(
    "var(--color-gold-ink)",
    "..#..#..", ".#.##.#.", ".#.##.#.", ".######.",
    "..####..", "...##...", "...##...", "..####..",
  ),
  present: art(
    "var(--color-punch-ink)",
    "..#..#..", ".######.", ".#.##.#.", "########",
    "#..##..#", "#..##..#", "#..##..#", "########",
  ),
  stroll: art(
    "var(--color-sky-ink)",
    "..###...", "..###...", "...#....", "..###...",
    ".#.#.#..", "...#....", "..#.#...", ".#...#..",
  ),
  stats: art(
    "var(--color-lime-ink)",
    "########", "#......#", "#.#....#", "#.#..#.#",
    "#.#.##.#", "#.#.##.#", "#......#", "########",
  ),
  contributions: art(
    "var(--color-lime-ink)",
    "#.##.#.#", ".##.##.#", "##.#.###", "#.###.#.",
    ".#.##.##", "##.#.#.#", "#.###.##", ".##.#.#.",
  ),
  rocket: art(
    "var(--color-mute)",
    "...##...", "..####..", "..#..#..", "..####..",
    ".######.", "#.####.#", "...##...", "..#..#..",
  ),
  launch: art(
    "var(--color-punch-ink)",
    "...##...", "..####..", "..####..", ".######.",
    "#.####.#", "..#..#..", ".#.##.#.", "#..##..#",
  ),
  bake: art(
    "var(--color-gold-ink)",
    "...#....", "..#.#...", "...#....", ".######.",
    "########", "########", "########", ".######.",
  ),
  campfire: art(
    "var(--color-punch-ink)",
    "...#....", "..###...", "..###...", ".#####..",
    ".#####..", "..###...", "#..#...#", ".######.",
  ),
  sing: art(
    "var(--color-grape-ink)",
    "..####..", ".#....#.", ".#....#.", ".#....#.",
    "..####..", "...##...", "...##...", "..####..",
  ),
  soccer: art(
    "var(--color-ink)",
    "..####..", ".#.##.#.", "#..##..#", "##....##",
    "##....##", "#..##..#", ".#.##.#.", "..####..",
  ),
  dance: art(
    "var(--color-grape-ink)",
    "...##...", "...##...", "#.####.#", ".######.",
    "...##...", "..#..#..", ".#....#.", "#......#",
  ),
  sleep: art(
    "var(--color-cloud-ink)",
    "#####...", "....#...", "...#....", "#####...",
    "...####.", "....#...", "...#....", "..####..",
  ),
  "{verdict}": art(
    "var(--color-gold-ink)",
    "########", "#......#", "#.##...#", "#......#",
    "#..##..#", "#......#", "#...##.#", "########",
  ),
  cloud: art(
    "var(--color-cloud-ink)",
    "..####..", ".######.", "########", "########",
    "########", ".######.", "..#..#..", "..#..#..",
  ),
};

export const pixelArt = (id: string): Art => PIXEL_ART[id] ?? PIXEL_ART["{verdict}"];
