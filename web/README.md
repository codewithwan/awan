# awan web — build your banner

The config editor. Arrange the scenes, watch it play, take the three files.

**It is a static page and must stay one.** The whole promise of the Action is
that your banner keeps working whether or not we do: your `awan.json` lives in
your repo and the workflow runs on your runner. The moment this page grows a
server, it becomes something that can go down and take READMEs with it. So:
no API routes, no database, no accounts, no stored links.

## Layout

```
crate/          wasm-bindgen bridge over awan-core
src/lib/        data and pure helpers — acts, config, sample numbers, pixel art
src/ui/         primitives — Button, Card, Field, PixelIcon, Stepper
src/stage/      the reel: clock, canvas, overlays, transport, meter
src/story/      the running order: list, row, shelf
src/steps/      one file per step of the wizard
```

`Card` doesn't nest on purpose. A card inside a card is two borders and two
shadows saying the same thing; if something inside needs separating, space or a
rule does it.

## The preview is the engine

`crate/` is a thin `wasm-bindgen` bridge over `awan-core`. The engine is a pure
function of `(tick, character)` — no clock, no RNG, no I/O — which is why it
survives the trip to wasm, and why the canvas here draws the same frames CI
does rather than an impression of them.

That split matters for the overlays too. The engine draws a scene's *shapes*
and leaves its *words and numbers* to the renderer; `overlays.ts` is that
renderer, doing for the canvas what `profile/src/` does for the GIF.

**The canvas is 1056×416, including the caption strip, in the engine's own
font8x8 glyphs at the renderer's own scale.** That is not decoration. An earlier
build drew the caption in HTML underneath in Courier, and it looked close enough
to ship and wrong enough to notice — which is the worst thing a preview can be,
because someone builds a config against it and CI hands them something else.

Two things caught that, and both are worth knowing about:

- **Integer division.** Rust computes `303 / 2 = 151`; JavaScript gives `151.5`,
  which lands a glyph on a half-pixel, and the canvas antialiases every edge. A
  third of the ink stops matching. Every position here is `Math.floor`d.
- **Colour by hand.** `[150, 150, 160]` is `#9696a0`, not `#96969f`. Convert;
  don't eyeball.

You can check it: render a GIF and the preview from the same config and seed,
then compare the caption strip. It should be identical, pixel for pixel — ink
count and bounding box both.

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
