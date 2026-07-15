# Language clients

Thin wrappers that drive the `awan` binary from other languages. They are all a
few lines each — the real integration surface is the binary and its
[event protocol](../docs/INTEGRATE.md), so these just spawn the process.

| Language | File | Install the binary with |
|---|---|---|
| Node    | [`packages/npm`](../packages/npm) | `npm i @codewithwan/awan` |
| Python  | [`python/awan.py`](python/awan.py) | `pip install awan-cli` |
| Go      | [`go/awan.go`](go/awan.go) | same |
| Shell   | [`bash/awan.sh`](bash/awan.sh) | same |

Missing your language? It's ~20 lines: spawn `awan react <event>`, run
`awan busy "<label>"` in the background, or write event lines to `awan watch`.
Copy any file here as a template.
