// Programmatic API for Node projects. Every call just drives the `awan`
// binary — the same personality layer any language reaches over the process
// boundary. See docs/INTEGRATE.md for the full event protocol.
const fs = require("fs");
const path = require("path");
const { spawn, spawnSync } = require("child_process");

const binPath = path.join(__dirname, "bin", process.platform === "win32" ? "awan.exe" : "awan");

// Ensure the binary exists, fetching it on first use. Modern npm blocks the
// postinstall hook by default, so we download lazily here too — synchronously,
// so the API stays blocking-friendly.
function ensureSync() {
  if (!fs.existsSync(binPath)) {
    spawnSync(process.execPath, [path.join(__dirname, "install.js")], { stdio: "inherit" });
  }
  return binPath;
}

function charArgs(opts = {}) {
  return opts.character ? ["-c", opts.character] : [];
}

// Play a character's one-shot reaction to an event, then return. Blocking.
function react(event, opts = {}) {
  spawnSync(ensureSync(), ["react", event, ...charArgs(opts)], { stdio: "inherit" });
}

// Show the "working…" loop with a caption while a task runs. Returns a handle;
// call .stop() when the work is done.
function busy(label, opts = {}) {
  const child = spawn(ensureSync(), ["busy", label, ...charArgs(opts)], { stdio: "inherit" });
  return { stop: () => child.kill("SIGINT") };
}

// Start an ambient companion you feed events to over time. Returns a handle:
// .emit("cmd.ok") pushes an event, .stop() ends it.
function watch(opts = {}) {
  const child = spawn(ensureSync(), ["watch", ...charArgs(opts)], {
    stdio: ["pipe", "inherit", "inherit"],
  });
  return {
    emit: (event) => child.stdin.write(`${event}\n`),
    stop: () => child.kill("SIGINT"),
  };
}

module.exports = { binPath, ensureSync, react, busy, watch };
