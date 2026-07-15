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
      sha256 "396df28c118aeb08550e7640eb870f896bc7afd0e6f7981c22bbf4d9f6d3a273"
    end
    on_intel do
      url "#{base}/awan-x86_64-apple-darwin.tar.gz"
      sha256 "51c53baf37af00623ade97130b2fb165f846740bc3296ee41f9678ecd3a8504b"
    end
  end

  on_linux do
    on_arm do
      url "#{base}/awan-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "fa4dca8de101a1e3335337cba430001a1f7503fa8606971eab2e621e8f355738"
    end
    on_intel do
      url "#{base}/awan-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "f8cfee0d9f5df37950a983d5243024ea081f03ec27cd5ffae652a552e67f70ba"
    end
  end

  def install
    bin.install "awan"
  end

  test do
    assert_match "awan #{version}", shell_output("#{bin}/awan --version")
  end
end
