import awanToml from "../../../characters/awan.toml?raw";
import oyenToml from "../../../characters/oyen.toml?raw";

/** The cast, read straight out of the repo's own TOML at build time — so the
 *  preview restyles from the same specs the CLI does, and a new character in
 *  characters/ only has to be listed here. */
export type Cast = { id: string; label: string; blurb: string; toml: string; path: string };

export const CAST: Cast[] = [
  {
    id: "awan",
    label: "Awan",
    blurb: "the reference cloud buddy",
    toml: awanToml,
    path: "",
  },
  {
    id: "oyen",
    label: "Oyen",
    blurb: "a chunky orange cat",
    toml: oyenToml,
    path: "characters/oyen.toml",
  },
];

/** The cast, plus whoever you've drawn. A drawn character isn't a special case
 *  in the preview — it's a spec like any other, which is the point of specs. */
export const castOf = (id: string, mine?: Cast): Cast =>
  (mine && mine.id === id ? mine : CAST.find((c) => c.id === id)) ?? CAST[0];
