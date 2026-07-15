# awan

A tiny living character for your terminal — and a **personality layer** any
CLI can embed. This package ships the `awan` binary to Node projects.

```sh
npx @codewithwan/awan demo               # try it, no install
npm i -g @codewithwan/awan && awan demo  # or install the command
npm i @codewithwan/awan                  # or use it from your project
```

On install it downloads the prebuilt binary for your platform from the
project's GitHub Releases. No Rust toolchain required.

## Use it from Node

```js
const awan = require("@codewithwan/awan");

awan.react("task.done"); // one-shot celebration, then returns

const job = awan.busy("compiling"); // living progress indicator
// …do work…
job.stop();

const buddy = awan.watch(); // ambient companion you feed events to
buddy.emit("cmd.start");
buddy.emit("cmd.ok");
buddy.stop();
```

Every call just drives the binary, so the behaviour is identical to the CLI.
See the [event protocol and other languages](https://github.com/codewithwan/awan/blob/main/docs/INTEGRATE.md).

## License

MIT OR Apache-2.0.
