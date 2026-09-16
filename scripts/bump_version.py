#!/usr/bin/env python3

import pathlib
import re
import sys

VERSION_RE = re.compile(r"^version = \"(\d+\.\d+\.\d+)\"", re.MULTILINE)


def parse_version(text: str) -> str:
    match = VERSION_RE.search(text)
    if not match:
        raise SystemExit("Cargo.toml has no semver version line")
    return match.group(1)


def bump_patch(version: str) -> str:
    major, minor, patch = (int(part) for part in version.split("."))
    return f"{major}.{minor}.{patch + 1}"


def replace_version(text: str, new_version: str) -> str:
    if not VERSION_RE.search(text):
        raise SystemExit("Cargo.toml has no semver version line")
    return VERSION_RE.sub(f'version = "{new_version}"', text, count=1)


def main() -> None:
    path = pathlib.Path("Cargo.toml")
    text = path.read_text(encoding="utf-8")
    current = parse_version(text)
    if len(sys.argv) > 1 and sys.argv[1]:
        new_version = sys.argv[1]
        if new_version == current:
            raise SystemExit(f"version is already {current}")
    else:
        new_version = bump_patch(current)
    path.write_text(replace_version(text, new_version), encoding="utf-8")
    print(new_version)


if __name__ == "__main__":
    main()
