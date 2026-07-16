import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwind from "@tailwindcss/vite";
import wasm from "vite-plugin-wasm";

// Static build, deployed to GitHub Pages. Nothing here may grow a server:
// the whole point is that this page can never go down in a way that breaks
// somebody's README.
export default defineConfig({
  base: "./",
  plugins: [react(), tailwind(), wasm()],
  build: { outDir: "dist", target: "esnext" },
});
