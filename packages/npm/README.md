# awan

A tiny pixel character that walks your GitHub contribution year.

**[Build yours →](https://codewithwan.github.io/awan/)** · [source](https://github.com/codewithwan/awan)

He also lives in your terminal — that's what this package installs.

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
