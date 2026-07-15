// Fetch the prebuilt `awan` binary for this platform from the GitHub Release
// and drop it in ./bin. Runs as a postinstall hook, but modern npm blocks
// install scripts by default — so run.js/index.js also invoke this lazily on
// first use (see ensureSync in index.js). Safe to run repeatedly.
const fs = require("fs");
const path = require("path");
const https = require("https");
const { execFileSync } = require("child_process");

const { binaryVersion } = require("./package.json");
const REPO = "codewithwan/awan";
const TAG = `v${binaryVersion}`;

// Node platform/arch -> Rust target triple (must match the release matrix).
const TARGETS = {
  "darwin-arm64": "aarch64-apple-darwin",
  "darwin-x64": "x86_64-apple-darwin",
  "linux-x64": "x86_64-unknown-linux-gnu",
  "linux-arm64": "aarch64-unknown-linux-gnu",
  "win32-x64": "x86_64-pc-windows-msvc",
};

const win = process.platform === "win32";
const binDir = path.join(__dirname, "bin");
const binPath = path.join(binDir, win ? "awan.exe" : "awan");

function main() {
  if (fs.existsSync(binPath)) return; // already installed
  const target = TARGETS[`${process.platform}-${process.arch}`];
  if (!target) return bail(`no prebuilt binary for ${process.platform}-${process.arch}`);

  const ext = win ? "zip" : "tar.gz";
  const url = `https://github.com/${REPO}/releases/download/${TAG}/awan-${target}.${ext}`;
  fs.mkdirSync(binDir, { recursive: true });
  const archive = path.join(binDir, `awan.${ext}`);

  download(url, archive, () => {
    try {
      if (win) {
        execFileSync("powershell", ["-command", `Expand-Archive -Force '${archive}' '${binDir}'`]);
      } else {
        execFileSync("tar", ["-xzf", archive, "-C", binDir]);
        fs.chmodSync(binPath, 0o755);
      }
      fs.unlinkSync(archive);
      console.log(`awan ${binaryVersion} installed (${target}).`);
    } catch (e) {
      bail(`could not unpack the binary: ${e.message}`);
    }
  });
}

// Download following GitHub's redirect to the asset host.
function download(url, dest, done, hops = 0) {
  https
    .get(url, { headers: { "User-Agent": "awan-npm" } }, (res) => {
      if ([301, 302, 307, 308].includes(res.statusCode) && res.headers.location) {
        if (hops > 5) return bail("too many redirects");
        return download(res.headers.location, dest, done, hops + 1);
      }
      if (res.statusCode !== 200) return bail(`download failed (HTTP ${res.statusCode})`);
      const file = fs.createWriteStream(dest);
      res.pipe(file);
      file.on("finish", () => file.close(done));
    })
    .on("error", (e) => bail(e.message));
}

// Never hard-fail the install; run.js surfaces a missing binary at call time.
function bail(msg) {
  console.warn(`awan: ${msg}`);
  console.warn("      build from source instead: cargo install awan-cli");
  process.exit(0);
}

main();
