# Rust example

```sh
cargo run
```

No binary, no subprocess — this links the `awan-core` engine and renders frames
directly. That's the tightest integration; every other language drives the
prebuilt binary instead. See [src/main.rs](src/main.rs).
