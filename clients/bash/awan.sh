# Drive the `awan` character from any POSIX shell script. Source this file:
#
#     source clients/bash/awan.sh
#     awan_react task.done
#     awan_busy "compiling" &  BUSY=$!;  make;  kill "$BUSY"
#
# For a live companion that reacts to your interactive shell instead, use the
# ready-made hooks in shell/awan.zsh. See docs/INTEGRATE.md for the events.

AWAN_BIN="${AWAN_BIN:-awan}"

# awan_react <event> [character.toml] — one-shot reaction, then returns.
awan_react() {
  local event="$1" char="${2:-}"
  if [ -n "$char" ]; then "$AWAN_BIN" react "$event" -c "$char"
  else "$AWAN_BIN" react "$event"; fi
}

# awan_busy <label> [character.toml] — the "working…" loop (run in background).
awan_busy() {
  local label="$1" char="${2:-}"
  if [ -n "$char" ]; then "$AWAN_BIN" busy "$label" -c "$char"
  else "$AWAN_BIN" busy "$label"; fi
}

# awan_emit <fifo> <event> — push an event to a running `awan watch --pipe`.
# Non-blocking: never stalls your script if nothing is reading.
awan_emit() {
  local fifo="$1" event="$2"
  [ -p "$fifo" ] && ( : ; printf '%s\n' "$event" >"$fifo" & ) 2>/dev/null
}
