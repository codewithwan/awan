#!/usr/bin/env node
// CLI entry: exec the downloaded `awan` binary, forwarding args, stdio, and
// the exit code so `npx awan demo` behaves exactly like the native binary.
const { spawnSync } = require("child_process");
const { binPath } = require("./index.js");
const fs = require("fs");

if (!fs.existsSync(binPath)) {
  console.error("awan: binary not found. Try `npm rebuild awan`, or build from source:");
  console.error("      cargo install awan-cli");
  process.exit(1);
}

const { status, signal } = spawnSync(binPath, process.argv.slice(2), { stdio: "inherit" });
if (signal) process.kill(process.pid, signal);
process.exit(status ?? 1);
