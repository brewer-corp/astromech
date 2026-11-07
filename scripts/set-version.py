#!/usr/bin/env python3
"""Stamp a release version into every file that carries the app's version.

The repository keeps the placeholder 0.0.0-dev, which sorts below every release, so a build
from a checkout never looks newer than a published update. release.yml runs this on a tag
before building, so the installer, latest.json and every binary's --version say the tag:

    python3 scripts/set-version.py 1.0.0-alpha.5     (or v1.0.0-alpha.5)

It edits [workspace.package] in Cargo.toml, the workspace members' entries in Cargo.lock (so
`--locked` still passes), app/package.json and app/src-tauri/tauri.conf.json. Exits 1 on a
version that isn't MAJOR.MINOR.PATCH[-PRERELEASE] or on a file it can't find its place in.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VERSION = re.compile(r"\d+\.\d+\.\d+(-[0-9A-Za-z]+(\.[0-9A-Za-z]+)*)?")


def sub_once(path: Path, pattern: str, repl: str, flags=0) -> None:
    text = path.read_text(encoding="utf-8")
    new, n = re.subn(pattern, repl, text, count=1, flags=flags)
    if n != 1:
        raise SystemExit(f"{path.relative_to(ROOT)}: version not found")
    path.write_text(new, encoding="utf-8")


def set_lock(path: Path, version: str) -> int:
    # A workspace member's [[package]] entry has no `source` line; registry and git ones do.
    blocks = path.read_text(encoding="utf-8").split("\n[[package]]\n")
    n = 0
    for i, b in enumerate(blocks):
        if i == 0 or "\nsource = " in b:
            continue
        b2, k = re.subn(r'^version = "[^"]*"$', f'version = "{version}"', b, count=1, flags=re.M)
        blocks[i] = b2
        n += k
    path.write_text("\n[[package]]\n".join(blocks), encoding="utf-8")
    return n


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    version = sys.argv[1].removeprefix("v")
    if not VERSION.fullmatch(version):
        raise SystemExit(f"not a version: {sys.argv[1]!r} (want MAJOR.MINOR.PATCH[-PRERELEASE])")

    cargo = ROOT / "Cargo.toml"
    text = cargo.read_text(encoding="utf-8")
    start = text.index("[workspace.package]")
    end = text.find("\n[", start + 1)
    end = len(text) if end < 0 else end
    section, n = re.subn(r'^version = "[^"]*"$', f'version = "{version}"', text[start:end], count=1, flags=re.M)
    if n != 1:
        raise SystemExit("Cargo.toml: no version in [workspace.package]")
    cargo.write_text(text[:start] + section + text[end:], encoding="utf-8")

    members = set_lock(ROOT / "Cargo.lock", version)
    if members == 0:
        raise SystemExit("Cargo.lock: no workspace members found")

    for rel in ("app/package.json", "app/src-tauri/tauri.conf.json"):
        p = ROOT / rel
        # Rewrite only the top-level "version" value, keeping the file's own formatting.
        if json.loads(p.read_text(encoding="utf-8")).get("version") is None:
            raise SystemExit(f"{rel}: no top-level version")
        sub_once(p, r'^(  "version": )"[^"]*"', rf'\g<1>"{version}"', re.M)

    print(f"version {version}: Cargo.toml, Cargo.lock ({members} workspace crates), app/package.json, tauri.conf.json")


if __name__ == "__main__":
    main()
