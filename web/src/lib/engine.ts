import init, { Preview } from "../wasm/awan_wasm";

/** One init for the page. wasm-bindgen throws if you call it twice, and React
 *  in strict mode will absolutely try. */
let started: Promise<unknown> | null = null;
export const loadEngine = () => (started ??= init());

export type { Preview };
export * as engine from "../wasm/awan_wasm";
