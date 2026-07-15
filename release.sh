#!/usr/bin/env bash
# Cut a release in one command:  ./release.sh 0.0.4
#
# Bumps every version in lockstep (cargo workspace, npm wrapper + its binary
# tag, the PyPI package + its binary tag, the Homebrew formula), runs the
# checks, tags and pushes (which builds the binaries via the release workflow),
# then publishes the crates. npm/PyPI publishing and the Homebrew sha256s stay
# manual — they need tokens or the freshly-built binaries — and are printed at
# the end.
set -euo pipefail

NEW="${1:-}"
[[ "$NEW" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "usage: ./release.sh X.Y.Z"; exit 1; }
cd "$(dirname "$0")"

[[ -z "$(git status --porcelain)" ]] || { echo "working tree not clean — commit first"; exit 1; }
[[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ]] || { echo "not on main"; exit 1; }

OLD="$(grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2)"
echo "==> $OLD -> $NEW"

# 1. bump versions everywhere
sed -i '' "s/\"$OLD\"/\"$NEW\"/g" Cargo.toml
( cd packages/npm && npm version --no-git-tag-version "$NEW" >/dev/null && npm pkg set "binaryVersion=$NEW" )
sed -i '' "s/^version = \".*\"/version = \"$NEW\"/" packages/pypi/pyproject.toml
sed -i '' "s/^BINARY_VERSION = \".*\"/BINARY_VERSION = \"$NEW\"/" packages/pypi/src/awan/_binary.py
sed -i '' "s/^  version \".*\"/  version \"$NEW\"/" packages/homebrew/awan.rb
cargo build -q   # refresh Cargo.lock

# 2. checks
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace

# 3. commit + tag + push (pushing the tag triggers the binary build)
git add -A
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

Crates published. Remaining manual steps:

  # wait for the binaries, then:
  gh run watch --repo codewithwan/awan

  # npm (needs your 2FA code)
  cd packages/npm && npm publish --otp=<code>

  # PyPI (needs a token)
  cd packages/pypi && python3 -m build && python3 -m twine upload dist/*

  # Homebrew: refresh the four sha256s in packages/homebrew/awan.rb, then copy
  # it to your homebrew-awan tap. Get the sums with:
  B=https://github.com/codewithwan/awan/releases/download/v$NEW
  for t in aarch64-apple-darwin x86_64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu; do
    echo "\$t \$(curl -sL \$B/awan-\$t.tar.gz | shasum -a 256 | cut -d' ' -f1)"
  done

EOF
