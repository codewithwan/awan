import { zipSync, strToU8 } from "fflate";

/** Hand the whole thing over as a folder, not as three clipboards.
 *
 *  Copy buttons make you the build system: you have to know that awan.yml goes
 *  in .github/workflows/ and not next to it, and that's the step people get
 *  wrong at midnight. A zip carries the paths, so unzipping *is* the setup.
 *
 *  Stored, not deflated — three text files, and the browser's own download
 *  layer compresses the transfer anyway.
 */
export function downloadZip(name: string, files: Record<string, string>) {
  const entries = Object.fromEntries(
    Object.entries(files).map(([path, body]) => [path, strToU8(body)]),
  );
  const blob = new Blob([zipSync(entries, { level: 0 })], { type: "application/zip" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = name;
  a.click();
  URL.revokeObjectURL(url);
}
