#!/usr/bin/env python3
"""Dependency-rule check (docs/ARCHITECTURE.md §4).

Fails if a workspace crate has a forbidden *direct* dependency on another workspace crate.
Only astromech-policy may drive the input executor; voice and tools go through policy.
"""
import json
import subprocess
import sys

# crate -> workspace crates it must never depend on directly
FORBIDDEN = {
    "astromech-voice": {"astromech-input"},
    "astromech-tools": {"astromech-input"},
    "astromech-overlay": {"astromech-input"},
    "astromech-intent": {"astromech-input", "astromech-policy"},
    "astromech-data": {"astromech-input", "astromech-policy"},
}
# crates that must not depend on any workspace crate
LEAVES = {"astromech-proto", "astromech-intent", "astromech-log"}


def main() -> int:
    meta = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"], text=True))
    members = {p["name"]: p for p in meta["packages"]}
    errors = []
    for name, pkg in sorted(members.items()):
        deps = {d["name"] for d in pkg["dependencies"] if d["name"] in members}
        for bad in sorted(deps & FORBIDDEN.get(name, set())):
            errors.append(f"{name} must not depend on {bad} directly (go through astromech-policy)")
        if name in LEAVES and deps:
            errors.append(f"{name} must not depend on workspace crates, found: {', '.join(sorted(deps))}")
    for e in errors:
        print(f"::error::{e}")
    if not errors:
        print(f"dependency rules OK ({len(members)} workspace crates)")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
