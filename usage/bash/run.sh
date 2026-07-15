#!/usr/bin/env bash
# Use case: wrap a slow task with a companion. In the shell, awan IS the client —
# you just call the command, nothing to import or spawn by hand.
set -uo pipefail

awan busy "crunching numbers" &   # he works while you do…
busy=$!
sleep 2.5                          # …something slow here
kill "$busy" 2>/dev/null

awan react task.done               # 🎉  (swap for "cmd.failed" on failure)
