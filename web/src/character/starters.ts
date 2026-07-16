import { type Character, blankRows } from "../lib/spec";

/** Somewhere to start from.
 *
 *  A blank 10×6 is a cruel opening move: you can't tell what the grammar wants
 *  until you've seen one work. These are shapes, not characters — a body plan
 *  to push around. The real ones live in characters/ and are the reference.
 */
export const STARTERS: { id: string; label: string; rows: string[]; body: string; eye: string }[] = [
  {
    id: "blob",
    label: "Blob",
    body: "#7C88F0",
    eye: "#2D303C",
    rows: ["  ####  ".padEnd(10), " ######## ".slice(0, 10), "##@@##@@##", "###----###", "+########+", " ## ## ## "],
  },
  {
    id: "cat",
    label: "Cat",
    body: "#E8963C",
    eye: "#3A2A1E",
    rows: [" #+    +# ", "+########+", "##@@##@@##", "###----###", "+########+", " # #  # # "],
  },
  {
    id: "bot",
    label: "Bot",
    body: "#4DD4FF",
    eye: "#0D1117",
    rows: ["    ##    ", " ######## ", "#@@####@@#", "#--------#", "+########+", " ##    ## "],
  },
  {
    id: "empty",
    label: "Blank",
    body: "#39D353",
    eye: "#0D1117",
    rows: blankRows(),
  },
];

export const starter = (id: string): Character => {
  const s = STARTERS.find((x) => x.id === id) ?? STARTERS[0];
  return {
    name: "",
    author: "",
    description: "",
    body: s.body,
    eye: s.eye,
    rows: [...s.rows],
  };
};
