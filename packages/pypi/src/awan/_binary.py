"""Resolve the prebuilt ``awan`` binary, downloading it on first use.

The binary lives in the project's GitHub Releases (asset names come from
.github/workflows/release.yml). We cache it under the user cache dir so a
plain ``pip install awan-cli`` needs no compiler and no install-time hooks.
"""

from __future__ import annotations

import io
import os
import platform
import stat
import tarfile
import urllib.request
import zipfile

BINARY_VERSION = "0.0.9"
REPO = "codewithwan/awan"

# (system, machine) -> Rust target triple
_TARGETS = {
    ("Darwin", "arm64"): "aarch64-apple-darwin",
    ("Darwin", "x86_64"): "x86_64-apple-darwin",
    ("Linux", "x86_64"): "x86_64-unknown-linux-gnu",
    ("Linux", "aarch64"): "aarch64-unknown-linux-gnu",
    ("Windows", "AMD64"): "x86_64-pc-windows-msvc",
}


def _cache_dir() -> str:
    base = os.environ.get("XDG_CACHE_HOME") or os.path.join(os.path.expanduser("~"), ".cache")
    path = os.path.join(base, "awan", BINARY_VERSION)
    os.makedirs(path, exist_ok=True)
    return path


def binary_path() -> str:
    name = "awan.exe" if platform.system() == "Windows" else "awan"
    return os.path.join(_cache_dir(), name)


def ensure() -> str:
    """Return the path to the binary, downloading it the first time."""
    dest = binary_path()
    if os.path.exists(dest):
        return dest

    target = _TARGETS.get((platform.system(), platform.machine()))
    if not target:
        raise RuntimeError(
            f"no prebuilt awan binary for {platform.system()}/{platform.machine()}; "
            "install it with `cargo install awan-cli` instead"
        )

    win = platform.system() == "Windows"
    ext = "zip" if win else "tar.gz"
    url = f"https://github.com/{REPO}/releases/download/v{BINARY_VERSION}/awan-{target}.{ext}"
    req = urllib.request.Request(url, headers={"User-Agent": "awan-pypi"})
    data = urllib.request.urlopen(req).read()  # follows redirects

    if win:
        with zipfile.ZipFile(io.BytesIO(data)) as z:
            z.extractall(_cache_dir())
    else:
        with tarfile.open(fileobj=io.BytesIO(data)) as t:
            _safe_extract(t, _cache_dir())
        os.chmod(dest, os.stat(dest).st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    return dest


def _safe_extract(tar: tarfile.TarFile, path: str) -> None:
    # Python 3.12 added an extraction filter; use it where available.
    try:
        tar.extractall(path, filter="data")
    except TypeError:
        tar.extractall(path)
