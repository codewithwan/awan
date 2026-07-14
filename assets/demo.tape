# Banner recording for the README.
#
#   brew install vhs        # one-time (pulls ttyd + ffmpeg)
#   vhs assets/demo.tape    # writes assets/demo.gif
#
# The GIF loops on its own — VHS captures a slice of `awan demo` and stitches
# it into a seamless loop.

Output assets/demo.gif

Set FontSize 22
Set Width 1050
Set Height 540
Set Padding 26
Set Framerate 25
Set Theme "Catppuccin Mocha"

# Build quietly and clear the screen before we start recording frames.
Hide
Type "cargo build -q -p awan-cli --release && clear" Enter
Sleep 4s
Show

Type "./target/release/awan demo" Enter
Sleep 60s
Ctrl+C
Sleep 400ms
