#!/usr/bin/env node
// CLI entry: exec the `awan` binary, forwarding args, stdio, and the exit code
// so `npx @codewithwan/awan demo` behaves exactly like the native binary. The
// binary is fetched on first run if npm skipped the postinstall hook.
const fs = require("fs");
const { spawnSync } = require("child_process");
const { binPath, ensureSync } = require("./index.js");

ensureSync();
if (!fs.existsSync(binPath)) {
  console.error("awan: could not obtain the binary. Build from source instead:");
  console.error("      cargo install awan-cli");
  process.exit(1);
}

const { status, signal } = spawnSync(binPath, process.argv.slice(2), { stdio: "inherit" });
if (signal) process.kill(process.pid, signal);
process.exit(status ?? 1);
