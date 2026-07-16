# awan-cli

A tiny pixel character that walks your GitHub contribution year.

**[Build yours →](https://codewithwan.github.io/awan/)** · [source](https://github.com/codewithwan/awan)

He also lives in your terminal — that's what this package installs.

On first run it downloads the prebuilt binary for your platform from the
project's GitHub Releases and caches it under `~/.cache/awan`. No Rust
toolchain required.

## Use it from Python

```python
import awan

awan.react("task.done")            # one-shot celebration

job = awan.busy("compiling")       # living progress indicator
# …do work…
job.terminate()

with awan.Watch() as buddy:         # ambient companion you feed events to
    buddy.emit("cmd.start")
    buddy.emit("cmd.ok")
```

Full guide and the event protocol:
<https://github.com/codewithwan/awan/blob/main/docs/INTEGRATE.md>.

## License

MIT OR Apache-2.0.
