#!/usr/bin/env bash
# Cross-process smoke test for the event protocol the language clients rely on.
# Builds the binary, then drives it exactly the way a client would.
#
#   ./tests/protocol.sh
set -euo pipefail

cd "$(dirname "$0")/.."
cargo build --quiet -p awan-cli --release
BIN=target/release/awan

echo "1/3  one-shot reaction"
"$BIN" react task.done >/dev/null
"$BIN" react cmd.failed >/dev/null

echo "2/3  named-pipe companion"
fifo="$(mktemp -u)"; mkfifo "$fifo"
"$BIN" watch --pipe "$fifo" >/dev/null 2>&1 &
watch_pid=$!
sleep 0.3
for ev in cmd.start cmd.ok task.done idle; do printf '%s\n' "$ev" >"$fifo"; done
sleep 0.3
kill "$watch_pid" 2>/dev/null || true
rm -f "$fifo"

echo "3/3  shell client wrapper"
AWAN_BIN="$BIN" bash -c 'source clients/bash/awan.sh && awan_react task.done >/dev/null'

echo "OK — protocol smoke test passed"
