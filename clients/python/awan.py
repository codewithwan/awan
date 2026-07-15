"""Drive the `awan` character from Python — a thin wrapper over the binary.

The binary is the integration surface, so there is no SDK to speak of: every
function here just spawns `awan`. Install the binary first (any of):

    npm i -g awan          # prebuilt, no toolchain
    cargo install awan-cli
    # or download from https://github.com/codewithwan/awan/releases

See docs/INTEGRATE.md for the full event vocabulary.
"""

from __future__ import annotations

import shutil
import subprocess
from typing import Optional

BIN = shutil.which("awan") or "awan"


def _char(character: Optional[str]) -> list[str]:
    return ["-c", character] if character else []


def react(event: str, character: Optional[str] = None) -> None:
    """Play the one-shot reaction to *event* (e.g. "task.done"), then return."""
    subprocess.run([BIN, "react", event, *_char(character)], check=False)


def busy(label: str, character: Optional[str] = None) -> subprocess.Popen:
    """Start the "working…" loop with *label*; call .terminate() when done."""
    return subprocess.Popen([BIN, "busy", label, *_char(character)])


class Watch:
    """An ambient companion you feed events to over time.

    with Watch() as buddy:
        buddy.emit("cmd.start")
        buddy.emit("cmd.ok")
    """

    def __init__(self, character: Optional[str] = None) -> None:
        self._p = subprocess.Popen([BIN, "watch", *_char(character)], stdin=subprocess.PIPE)

    def emit(self, event: str) -> None:
        assert self._p.stdin is not None
        self._p.stdin.write(f"{event}\n".encode())
        self._p.stdin.flush()

    def stop(self) -> None:
        self._p.terminate()

    def __enter__(self) -> "Watch":
        return self

    def __exit__(self, *_exc) -> None:
        self.stop()
