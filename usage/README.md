# Usage examples

What is awan *for*? It's a **personality layer** for command-line tools — drop a
little character into the moments your program already has (a long task running,
a job finishing, a prompt redrawing) so the terminal feels alive instead of
silent.

Each folder is a **self-contained, runnable example** for one language, showing
the same use case: *keep a companion on screen while a slow task runs, then
react to the result.*

| Language | Run it | You write |
|---|---|---|
| [node](node) | `cd node && npm install && npm start` | `awan.busy(…)`, `awan.react(…)` |
| [python](python) | `cd python && pip install -r requirements.txt && python main.py` | `awan.busy(…)`, `awan.react(…)` |
| [go](go) | `cd go && go run .` | `awan.Busy(…)`, `awan.React(…)` |
| [bash](bash) | `cd bash && ./run.sh` | `awan busy …`, `awan react …` |
| [rust](rust) | `cd rust && cargo run` | `Stage::show(…).frame(…)` |

## "Wait — is it just running a binary?"

Under the hood, yes: awan is one small binary. But **you never spawn it
yourself**. The Node, Python and Go packages give you a normal API —
`awan.react("task.done")` — and hide the process boundary completely. In the
shell, the `awan` command *is* the API, which is as simple as it gets. And in
Rust you skip the binary entirely and link the engine (`awan-core`) directly.

So from your code it reads like any other library — no `exec`, no `spawn`, no
plumbing.

## The events

`busy`/`react` take plain event names — `cmd.start`, `cmd.ok`, `cmd.failed`,
`task.done`, `idle` — and each character's `[reactions]` decides what they do.
Full protocol: [docs/INTEGRATE.md](../docs/INTEGRATE.md).
