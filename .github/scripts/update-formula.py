# Copyright 2026 Heath Stewart.
# Licensed under the MIT License. See LICENSE.txt in the project root for license information.

import hashlib
from pathlib import Path
import re
import sys


def update_formula(formula: Path, tag: str, packages: Path) -> None:
    if not re.fullmatch(r"v[0-9][0-9A-Za-z.+-]*", tag):
        raise ValueError(f"Invalid release tag: {tag}")

    pattern = re.compile(
        r'(?P<indent>^[ \t]*)url "https://github\.com/heaths/wslawake/releases/download/'
        r'[^/"]+/linux-(?P<arch>arm64|x64)\.tar\.gz"\n'
        r'(?P=indent)sha256 "[0-9a-f]{64}"',
        re.MULTILINE,
    )
    content = formula.read_text()
    architectures = [match["arch"] for match in pattern.finditer(content)]
    if sorted(architectures) != ["arm64", "x64"]:
        raise ValueError("Expected exactly one ARM64 and one x64 URL and SHA-256 pair")

    def replace(match: re.Match) -> str:
        name = f"linux-{match['arch']}.tar.gz"
        digest = hashlib.sha256((packages / name).read_bytes()).hexdigest()
        indent = match["indent"]
        return (
            f'{indent}url "https://github.com/heaths/wslawake/releases/download/{tag}/{name}"\n'
            f'{indent}sha256 "{digest}"'
        )

    formula.write_text(pattern.sub(replace, content))


if __name__ == "__main__":
    update_formula(Path(sys.argv[1]), sys.argv[2], Path(sys.argv[3]))
