# Use awan in your project

awan is a **binary plus a text protocol**, not a library you have to link. Any
language that can spawn a process and write a line of text can drive it — Rust,
Node, Python, Go, shell, anything. There is no SDK to depend on.

```
             your program  ──spawn / write events──▶  awan (the character)
```

## Install the binary

```sh
npm i -g @codewithwan/awan          # prebuilt, no toolchain
cargo install awan-cli # from source
# or grab a binary from https://github.com/codewithwan/awan/releases
```

## Three ways to integrate

### 1. One-shot reaction — `awan react <event>`

Play a short reaction and exit. Perfect for the end of a task.

```sh
awan react task.done     # celebrates
awan react cmd.failed    # chars
```

### 2. Living progress — `awan busy "<label>"`

A working-indicator loop with an animated caption. Run it while a job runs,
then stop it.

```sh
awan busy "compiling" &   # background it
cargo build               # …your real work…
kill %1                   # stop the loop
```

### 3. Ambient companion — `awan watch`

A character that sits in a pane and reacts to a **stream of events** you feed it
on stdin (or a named pipe with `--pipe <fifo>`), one event per line.

```sh
printf 'cmd.start\n' >> events   # he goes busy
printf 'cmd.ok\n'    >> events   # he celebrates
```

### 4. Prompt / statusline — `awan statusline [text]`

Prints one static line — a tiny face, the character's name, and optional status
text — then exits. Re-render it on every prompt. It colours output unless
`NO_COLOR` is set, so it works even when the prompt captures its output.

```sh
# bash/zsh prompt
PROMPT_COMMAND='awan statusline "$(git branch --show-current 2>/dev/null)"'

# tmux status line (~/.tmux.conf)
set -g status-right '#(awan statusline)'

# Claude Code — .claude/settings.json
# { "statusLine": { "type": "command", "command": "awan statusline" } }

# starship custom command
# [custom.awan]
# command = "awan statusline"
# when = true
```

## The event vocabulary

Events are plain lowercase lines. The character's TOML `[reactions]` table maps
each event to a scene, so what an event *does* is per-character data.

| Event        | Meaning                        | Default reaction |
|--------------|--------------------------------|------------------|
| `cmd.start`  | a command began                | switch to busy   |
| `cmd.ok`     | a command succeeded            | back to the show |
| `cmd.failed` | a command failed               | chars (oof)      |
| `cmd.done`   | a command finished             | back to the show |
| `task.done`  | a unit of work completed       | celebrate (yay)  |
| `idle`       | nothing happening for a while  | sleep (zzz)      |

Unknown events are ignored, so you can emit your own without breaking anything.

## Language snippets

Ready-made thin wrappers live in [`clients/`](../clients):

- **Node** — `npm i @codewithwan/awan`, then `require("@codewithwan/awan")` ([API](../packages/npm/README.md))
- **Python** — `pip install awan-cli` ([client](../clients/python/awan.py))
- **Go** — [`clients/go/awan.go`](../clients/go/awan.go)
- **Shell** — [`clients/bash/awan.sh`](../clients/bash/awan.sh)

```python
import awan
awan.react("task.done")
with awan.Watch() as buddy:
    buddy.emit("cmd.start"); buddy.emit("cmd.ok")
```

```js
const awan = require("@codewithwan/awan");
const job = awan.busy("deploying");
await deploy();
job.stop();
```

## Embed in a Rust program

Rust projects can skip the process boundary and use the engine directly:

```rust
use awan_core::{Character, Stage};

let stage = Stage::show(Character::default());
print!("{}", stage.frame(30, true)); // one deterministic frame
```

## Shell hooks (zero code)

To make awan react to your interactive shell automatically, source
[`shell/awan.zsh`](../shell/awan.zsh) and run `awan watch --pipe` in a spare
pane. It maps `preexec`/`precmd` to `cmd.start`/`cmd.ok`/`cmd.failed` for you.
