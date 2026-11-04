#!/usr/bin/env python3
"""Generate data/catalog/eddn-outfitting.json: which module symbols EDDN's outfitting/2 may carry.

Usage: scripts/gen-eddn-outfitting.py [--check] <FDevIDs checkout>

--check writes nothing: it exits 1, listing the differences, if the checkout's lists differ from
the committed file. The weekly **EDDN module list** workflow (.github/workflows/eddn-outfitting.yml)
runs it against FDevIDs' latest commit, so a module a game update adds shows up there instead of
only as refused outfitting messages in players' Diagnostics.

EDDN's outfitting README (EDCD/EDDN schemas/outfitting-README.md, "Elisions") says to drop
"items that have a non-null sku property, unless it's ELITE_HORIZONS_V_PLANETARY_LANDINGS (i.e.
PowerPlay and tech broker items)". The journal's Outfitting.json has no sku, so astromech looks
each module up by symbol in EDCD/FDevIDs outfitting.csv, whose `entitlement` column is that sku
(community data from the Frontier API, see NOTICE.md):

- `station`: no entitlement, or the Horizons one; sent.
- `commander`: any other entitlement (PowerPlay `ELITE_SPECIFIC_V_POWER_*`, Guardian and human
  tech-broker unlocks, paid-ship armour, and FDevIDs' `?` and `removed`); never sent.

A symbol in neither list can't be shown to be the station's, so astromech sends nothing for that
visit (crates/astromech-eddn/src/companion.rs). Symbols are lowercased, as the journal writes them;
a symbol any row marks commander-only is commander-only.
"""
import csv, json, os, subprocess, sys

HORIZONS = "ELITE_HORIZONS_V_PLANETARY_LANDINGS"
OUT = os.path.normpath(os.path.join(os.path.dirname(__file__), "..", "data", "catalog", "eddn-outfitting.json"))


def head(path):
    try:
        return subprocess.check_output(["git", "-C", path, "rev-parse", "--short", "HEAD"], text=True, stderr=subprocess.DEVNULL).strip()
    except Exception:
        return "unknown"


def lists(fdev):
    rows = list(csv.DictReader(open(os.path.join(fdev, "outfitting.csv"), encoding="utf-8")))
    # A pre-engineered row ("mercgear") shares its base module's symbol, sometimes with another
    # entitlement (the Enzyme Missile Rack's is a tech-broker sku, its pre-engineered one's is
    # blank). The journal can't tell them apart, so a symbol any row marks commander-only is.
    commander = sorted({r["symbol"].lower() for r in rows if r["entitlement"].strip() not in ("", HORIZONS)})
    station = sorted({r["symbol"].lower() for r in rows} - set(commander))
    return station, commander


def check(fdev):
    station, commander = lists(fdev)
    with open(OUT, encoding="utf-8") as f:
        old = json.load(f)
    was = {s: "station" for s in old["station"]} | {s: "commander" for s in old["commander"]}
    now = {s: "station" for s in station} | {s: "commander" for s in commander}
    new = sorted(s for s in now if s not in was)
    moved = sorted(s for s in now if s in was and now[s] != was[s])
    gone = sorted(s for s in was if s not in now)
    if not (new or moved or gone):
        print(f"{OUT} matches EDCD/FDevIDs@{head(fdev)}: {len(station)} station, {len(commander)} commander symbols")
        return
    for s in new:
        print(f"new: {s} ({now[s]}); until it is in the file, outfitting/2 sends nothing for a station listing it")
    for s in moved:
        print(f"moved: {s} ({was[s]} -> {now[s]})")
    for s in gone:
        print(f"gone: {s} (was {was[s]})")
    sys.exit(f"{OUT} is out of date with EDCD/FDevIDs@{head(fdev)}: run scripts/gen-eddn-outfitting.py <FDevIDs checkout> and commit it")


def main(fdev):
    station, commander = lists(fdev)
    out = {
        "source": f"EDCD/FDevIDs@{head(fdev)} outfitting.csv entitlement (scripts/gen-eddn-outfitting.py)",
        "station": station,
        "commander": commander,
    }
    with open(OUT, "w", encoding="utf-8", newline="\n") as f:
        json.dump(out, f, indent=1)
        f.write("\n")
    print(f"{len(station)} station, {len(commander)} commander symbols -> {OUT}")


if __name__ == "__main__":
    args = sys.argv[1:]
    if args[:1] == ["--check"] and len(args) == 2:
        check(args[1])
    elif len(args) == 1 and not args[0].startswith("-"):
        main(args[0])
    else:
        sys.exit(__doc__)
