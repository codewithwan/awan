# Homebrew

[`awan.rb`](awan.rb) installs the prebuilt `awan` binary from GitHub Releases
on macOS and Linux (Apple Silicon, Intel, and ARM/x86 Linux).

## Publish as a tap

```sh
# one-time: create a repo named `homebrew-awan` under your account
brew tap-new codewithwan/awan
cp awan.rb "$(brew --repository)/Library/Taps/codewithwan/homebrew-awan/Formula/"
# push that repo to github.com/codewithwan/homebrew-awan
```

Then anyone can:

```sh
brew install codewithwan/awan/awan
```

## Per release

Bump `version` and the four `sha256`s. `../../release.sh` prints the sums for a
tagged release; paste them in and push the tap repo.
