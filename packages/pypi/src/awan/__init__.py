"""Drive the ``awan`` terminal character from Python.

``pip install awan-cli`` gives you both the ``awan`` command and this API. Every
call spawns the binary — the same personality layer any language reaches over
the process boundary. See docs/INTEGRATE.md for the event vocabulary.

    import awan
    awan.react("task.done")
    with awan.Watch() as buddy:
        buddy.emit("cmd.start"); buddy.emit("cmd.ok")
"""

from __future__ import annotations

import subprocess
from typing import List, Optional

from ._binary import binary_path, ensure

__all__ = ["react", "busy", "Watch", "binary_path", "ensure"]


def _char(character: Optional[str]) -> List[str]:
    return ["-c", character] if character else []


def react(event: str, character: Optional[str] = None) -> None:
    """Play the one-shot reaction to *event* (e.g. "task.done"), then return."""
    subprocess.run([ensure(), "react", event, *_char(character)], check=False)


def busy(label: str, character: Optional[str] = None) -> subprocess.Popen:
    """Start the "working…" loop with *label*; call .terminate() when done."""
    return subprocess.Popen([ensure(), "busy", label, *_char(character)])


class Watch:
    """An ambient companion you feed events to over time (a context manager)."""

    def __init__(self, character: Optional[str] = None) -> None:
        self._p = subprocess.Popen([ensure(), "watch", *_char(character)], stdin=subprocess.PIPE)

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
