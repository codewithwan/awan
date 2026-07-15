"""Console entry point: fetch the binary on first use, then forward argv."""

import subprocess
import sys

from ._binary import ensure


def main() -> None:
    sys.exit(subprocess.run([ensure(), *sys.argv[1:]]).returncode)


if __name__ == "__main__":
    main()
