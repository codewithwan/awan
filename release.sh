#!/usr/bin/env bash
# Cut a release in one command:  ./release.sh 0.0.4
#
# Bumps every version in lockstep (cargo workspace + npm wrapper + the binary
# tag npm downloads), runs the checks, tags and pushes (which builds the
# binaries via .github/workflows/release.yml), then publishes the crates.
# npm publish needs your 2FA one-time code, so that last step stays manual.
set -euo pipefail

NEW="${1:-}"
[[ "$NEW" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "usage: ./release.sh X.Y.Z"; exit 1; }
cd "$(dirname "$0")"

[[ -z "$(git status --porcelain)" ]] || { echo "working tree not clean — commit first"; exit 1; }
[[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ]] || { echo "not on main"; exit 1; }

OLD="$(grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2)"
echo "==> $OLD -> $NEW"

# 1. bump versions
sed -i '' "s/\"$OLD\"/\"$NEW\"/g" Cargo.toml
( cd packages/npm && npm version --no-git-tag-version "$NEW" >/dev/null && npm pkg set "binaryVersion=$NEW" )
cargo build -q   # refresh Cargo.lock

# 2. checks
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace

# 3. commit + tag + push (push of the tag triggers the binary build)
git add Cargo.toml Cargo.lock packages/npm/package.json
git commit -q -m "chore: release $NEW"
git tag "v$NEW"
git push origin main
git push origin "v$NEW"

# 4. publish the crates in dependency order
for crate in awan-core awan-render awan awan-cli; do
  echo "==> cargo publish $crate"
  cargo publish -p "$crate"
  sleep 5
done

cat <<EOF

Done on the Rust side. Remaining manual step (needs your npm 2FA):

  gh run watch --repo codewithwan/awan          # wait for the binary build
  cd packages/npm && npm publish --otp=<code>   # publish the npm wrapper

EOF
