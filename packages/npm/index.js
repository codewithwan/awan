// Programmatic API for Node projects. Every call just drives the `awan`
// binary — the same personality layer any language reaches over the process
// boundary. See docs/INTEGRATE.md for the full event protocol.
const path = require("path");
const { spawn, spawnSync } = require("child_process");

const binPath = path.join(__dirname, "bin", process.platform === "win32" ? "awan.exe" : "awan");

function charArgs(opts = {}) {
  return opts.character ? ["-c", opts.character] : [];
}

// Play a character's one-shot reaction to an event, then return. Blocking.
function react(event, opts = {}) {
  spawnSync(binPath, ["react", event, ...charArgs(opts)], { stdio: "inherit" });
}

// Show the "working…" loop with a caption while a task runs. Returns a handle;
// call .stop() when the work is done.
function busy(label, opts = {}) {
  const child = spawn(binPath, ["busy", label, ...charArgs(opts)], { stdio: "inherit" });
  return { stop: () => child.kill("SIGINT") };
}

// Start an ambient companion you feed events to over time. Returns a handle:
// .emit("cmd.ok") pushes an event, .stop() ends it.
function watch(opts = {}) {
  const child = spawn(binPath, ["watch", ...charArgs(opts)], {
    stdio: ["pipe", "inherit", "inherit"],
  });
  return {
    emit: (event) => child.stdin.write(`${event}\n`),
    stop: () => child.kill("SIGINT"),
  };
}

module.exports = { binPath, react, busy, watch };
