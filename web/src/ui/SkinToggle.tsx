import { useEffect } from "react";
import { useDraft } from "../lib/store";

/** Light or dark, remembered.
 *
 *  It only ever repaints the page. The canvas keeps its dark backdrop because
 *  that is the backdrop the GIF has — turning the stage white would be showing
 *  someone a banner they aren't going to get. */
export function SkinToggle() {
  const [skin, setSkin] = useDraft<"dark" | "light">("skin", "dark");

  useEffect(() => {
    document.documentElement.dataset.skin = skin;
  }, [skin]);

  return (
    <button
      onClick={() => setSkin(skin === "dark" ? "light" : "dark")}
      className="nb-btn bg-slab px-3 py-1.5 text-[10px] uppercase text-ink"
      aria-label={`Switch to ${skin === "dark" ? "light" : "dark"} mode`}
    >
      {skin === "dark" ? "☀ light" : "☾ dark"}
    </button>
  );
}
