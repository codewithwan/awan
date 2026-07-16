# awan web — build your banner

The config editor. Arrange the scenes, watch it play, take the three files.

**It is a static page and must stay one.** The whole promise of the Action is
that your banner keeps working whether or not we do: your `awan.json` lives in
your repo and the workflow runs on your runner. The moment this page grows a
server, it becomes something that can go down and take READMEs with it. So:
no API routes, no database, no accounts, no stored links.

## The preview is the engine

`crate/` is a thin `wasm-bindgen` bridge over `awan-core`. The engine is a pure
function of `(tick, character)` — no clock, no RNG, no I/O — which is why it
survives the trip to wasm, and why the canvas here draws the same frames CI
does rather than an impression of them.

That split matters for the overlays too. The engine draws a scene's *shapes*
and leaves its *words and numbers* to the renderer; `overlays.ts` is that
renderer, doing for the canvas what `profile/src/` does for the GIF.

## What the preview can't know

The stats a profile shows are one unauthenticated call away, but the
contribution calendar lives only in GraphQL, and GraphQL wants a token. Nobody
should paste a token into a web page to look at a cartoon, so the preview uses
a plausible year from a fixed seed and says so on the page. CI has a token of
its own and fills in the real one.

## Running it

```sh
npm install
npm run wasm     # wasm-pack build → src/wasm (gitignored)
npm run dev
```

`npm run wasm` needs [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) and
the `wasm32-unknown-unknown` target.

`crate/` is its own workspace on purpose: wasm-bindgen's dependency tree is
heavier than the engine's MSRV, and the root workspace builds every member on
Rust 1.85. Keeping it out means the web app moves without dragging the engine's
floor up with it.
