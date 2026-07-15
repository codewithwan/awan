// Use case: keep a companion on screen while a slow task runs, then react.
// You just call the API — no spawning anything yourself.
const awan = require("@codewithwan/awan");

async function main() {
  const job = awan.busy("crunching numbers"); // he works while you do…
  await new Promise((r) => setTimeout(r, 2500)); // …something slow here
  job.stop();

  awan.react("task.done"); // 🎉  (swap for "cmd.failed" on the sad path)
}

main();
