// Postinstall: fetch the prebuilt `awan` binary for this platform from the
// matching GitHub Release (asset names come from .github/workflows/release.yml)
// and drop it in ./bin. Failure is non-fatal — run.js prints how to recover.
const fs = require("fs");
const os = require("os");
const path = require("path");
const https = require("https");
const { execFileSync } = require("child_process");

const { version, repository } = require("./package.json");
const REPO = repository.url.replace(/^github:/, "");

// Node platform/arch -> Rust target triple (must match the release matrix).
const TARGETS = {
  "darwin-arm64": "aarch64-apple-darwin",
  "darwin-x64": "x86_64-apple-darwin",
  "linux-x64": "x86_64-unknown-linux-gnu",
  "linux-arm64": "aarch64-unknown-linux-gnu",
  "win32-x64": "x86_64-pc-windows-msvc",
};

function main() {
  const key = `${process.platform}-${process.arch}`;
  const target = TARGETS[key];
  if (!target) return bail(`no prebuilt binary for ${key}`);

  const win = process.platform === "win32";
  const ext = win ? "zip" : "tar.gz";
  const url = `https://github.com/${REPO}/releases/download/v${version}/awan-${target}.${ext}`;
  const binDir = path.join(__dirname, "bin");
  fs.mkdirSync(binDir, { recursive: true });
  const archive = path.join(binDir, `awan.${ext}`);

  download(url, archive, () => {
    try {
      if (win) {
        execFileSync("powershell", ["-command", `Expand-Archive -Force '${archive}' '${binDir}'`]);
      } else {
        execFileSync("tar", ["-xzf", archive, "-C", binDir]);
        fs.chmodSync(path.join(binDir, "awan"), 0o755);
      }
      fs.unlinkSync(archive);
      console.log(`awan ${version} installed (${target}).`);
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

// Never hard-fail the install; leave a breadcrumb for run.js to surface.
function bail(msg) {
  console.warn(`awan: ${msg}`);
  console.warn("      the binary will be fetched on first run, or build from source:");
  console.warn("      cargo install awan-cli");
  process.exit(0);
}

main();
