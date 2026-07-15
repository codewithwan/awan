# Homebrew formula — installs the prebuilt `awan` binary from GitHub Releases.
#
# Publish it as a tap so users can `brew install codewithwan/awan/awan`:
#   1. create a repo named `homebrew-awan` under your account
#   2. copy this file to Formula/awan.rb there
#   3. bump `version` + the four `sha256`s on each release (release.sh prints them)
#
# Or install straight from this checkout: `brew install --formula ./packages/homebrew/awan.rb`
class Awan < Formula
  desc "Tiny living character for your terminal — a personality layer any CLI can embed"
  homepage "https://github.com/codewithwan/awan"
  version "0.0.4"
  license any_of: ["MIT", "Apache-2.0"]

  base = "https://github.com/codewithwan/awan/releases/download/v#{version}"

  on_macos do
    on_arm do
      url "#{base}/awan-aarch64-apple-darwin.tar.gz"
      sha256 "3323f0ed8378d3911410c9793b527c069511f6a6792cc2eecc3869fd20eee1a9"
    end
    on_intel do
      url "#{base}/awan-x86_64-apple-darwin.tar.gz"
      sha256 "a72d21268918a8ce46f750af81a3201d9535db1ea6784fd1f580b5ff1182af7a"
    end
  end

  on_linux do
    on_arm do
      url "#{base}/awan-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "12940243e4953e8f47046c4a220eba68c9b42241ffcca22b48b610fced40f518"
    end
    on_intel do
      url "#{base}/awan-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "3338fcd115de3cbba390d60daabb562dfc9379a0ffcc8089468558cbfc6a1853"
    end
  end

  def install
    bin.install "awan"
  end

  test do
    assert_match "awan #{version}", shell_output("#{bin}/awan --version")
  end
end
