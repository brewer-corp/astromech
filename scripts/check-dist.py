#!/usr/bin/env python3
"""Check that a release build of the window carries none of the browser preview (app/src/preview/)
and none of the dev bridge (app/src/lib/bridge.ts, app/dev/): no fixture and none of their own
strings in app/dist. The preview is loaded only when import.meta.env.DEV or VITE_PREVIEW is set,
and the bridge only in the dev server with its plugin on (app/src/lib/transport.ts), so a plain
`pnpm build` must drop both. Usage: scripts/check-dist.py [dist dir, default app/dist]. Exits 1 on
a find."""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# One string from a fixture and from each part of the fake, none of which the app itself says.
MARKERS = [
    "Arietis Sector LD-H a11-1",  # preview/fixtures/plot.json, the sample route
    "failed on purpose",  # preview/fakecore.ts, ?fail=
    "preview: no fake for",  # preview/transport.ts
    "/__astromech/call",  # dev/bridge.ts, the bridge's path
    "the dev bridge closed before",  # lib/bridge.ts
]


def main() -> int:
    dist = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "app" / "dist"
    files = [p for p in dist.rglob("*") if p.is_file() and p.suffix in {".js", ".html", ".css", ".json"}]
    if not files:
        print(f"check-dist: no build in {dist} (run `pnpm build` in app/ first)")
        return 1
    found = [(p, m) for p in files for m in MARKERS if m in p.read_text(encoding="utf-8", errors="replace")]
    for p, m in found:
        print(f"check-dist: {p.relative_to(dist)} contains the browser preview's or the dev bridge's {m!r}")
    if found:
        print("check-dist: the release build must not carry app/src/preview/ or the dev bridge; reach them only through lib/transport.ts")
        return 1
    print(f"check-dist: {len(files)} files in {dist}, no browser preview or dev bridge")
    return 0


if __name__ == "__main__":
    sys.exit(main())
