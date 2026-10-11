#!/usr/bin/env python3
"""Build the in-app update zip and latest.json for a GitHub Release.

Does not sign: CI runs `cargo run -p astromech-core --example sign-update` when
ASTROMECH_UPDATE_KEY is set.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import sys
import zipfile
from pathlib import Path


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def workspace_version() -> str:
    for line in Path("Cargo.toml").read_text(encoding="utf-8").splitlines():
        if line.startswith("version = "):
            return line.split("=", 1)[1].strip().strip('"')
    raise SystemExit("could not read version from Cargo.toml")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--version", default="")
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--notes", default="")
    ap.add_argument("files", nargs="+", type=Path)
    args = ap.parse_args()
    version = (args.version or workspace_version()).lstrip("v")
    tag = os.environ.get("GITHUB_REF", "")
    if tag.startswith("refs/tags/v"):
        tagged = os.environ.get("GITHUB_REF_NAME", "").lstrip("v")
        if tagged and tagged != version:
            print(f"tag v{tagged} != Cargo.toml version {version}", file=sys.stderr)
            return 1
    args.out.mkdir(parents=True, exist_ok=True)
    zip_name = f"astromech-{version}-windows-x64.zip"
    zip_path = args.out / zip_name
    if zip_path.exists():
        zip_path.unlink()
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as z:
        for p in args.files:
            if not p.is_file():
                print(f"missing {p}", file=sys.stderr)
                return 1
            name = p.name
            if name.lower() in {"data", "uninstall.exe"} or name.startswith("data"):
                print(f"refusing to pack {p}", file=sys.stderr)
                return 1
            info = zipfile.ZipInfo(filename=name)
            info.compress_type = zipfile.ZIP_DEFLATED
            with p.open("rb") as f:
                z.writestr(info, f.read())
    digest = sha256(zip_path)
    url = f"https://github.com/spadaval/astromech/releases/download/v{version}/{zip_name}"
    manifest = {
        "version": version,
        "notes": args.notes,
        "zip": zip_name,
        "sha256": digest,
        "url": url,
    }
    latest = args.out / "latest.json"
    latest.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    sums = args.out / "SHA256SUMS.txt"
    existing = sums.read_text(encoding="utf-8") if sums.exists() else ""
    line = f"{digest}  {zip_name}\n"
    if zip_name not in existing:
        with sums.open("a", encoding="utf-8") as f:
            f.write(line)
    print(f"wrote {zip_path} sha256={digest}")
    print(f"wrote {latest}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
