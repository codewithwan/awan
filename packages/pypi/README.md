# awan-cli

A tiny living character for your terminal — and a **personality layer** any CLI
can embed. This package ships the `awan` command (and a Python API) to PyPI.

```sh
pip install awan-cli
awan demo
```

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
