#!/usr/bin/env python3
"""Generate data/catalog/outfitting.json: every module a station can sell (by name, with each
class, rating and mount it comes in, and the journal symbols of each) and every ship, for the outfitting and shipyard lookups.

Usage: scripts/gen-outfitting.py <FDevIDs checkout>

Source (player-visible game data, see NOTICE.md): EDCD/FDevIDs outfitting.csv and shipyard.csv.
Kept: only modules Spansh lists as sold at stations (its `field_values/modules`, recorded in
crates/astromech-galaxy/tests/fixtures/spansh-module-names.json), so the pickers offer nothing
that is never sold. Ship-specific armour is one module per grade, with every ship's symbol.
Pre-engineered modules (FDevIDs' "mercgear") go in their base module's category without
symbols: they share their base module's symbol, so they are told apart by name.
"""
import collections, csv, json, os, subprocess, sys

CATEGORIES = {"standard": 0, "internal": 1, "hardpoint": 2, "utility": 3}
MOUNTS = {"Fixed": 0, "Gimballed": 1, "Turreted": 2, "": 3}
SYMBOL_MOUNTS = {"Fixed": "fixed", "Gimballed": "gimbal", "Turreted": "turret"}
ROOT = os.path.join(os.path.dirname(__file__), "..")
SPANSH_NAMES = os.path.join(ROOT, "crates", "astromech-galaxy", "tests", "fixtures", "spansh-module-names.json")
# Catalogue names Spansh lists under another name (astromech-galaxy's outfitting::SPANSH_NAMES).
SPANSH_RENAMES = {
    "Vessel Hangar": "Fighter Hangar",
    "MkII Business Class Passenger Cabin": "Mk II Business Class Passenger Cabin",
    "MkII Economy Class Passenger Cabin": "Mk II Economy Class Passenger Cabin",
}


def head(path):
    try:
        return subprocess.check_output(["git", "-C", path, "rev-parse", "--short", "HEAD"], text=True).strip()
    except Exception:
        return "unknown"


def main(fdev):
    rows = lambda p: list(csv.DictReader(open(os.path.join(fdev, p), encoding="utf-8")))
    sold = set(json.load(open(SPANSH_NAMES, encoding="utf-8"))["values"]["name"])
    all_rows = rows("outfitting.csv")
    base = {r["symbol"].lower(): r["category"] for r in all_rows if r["category"] in CATEGORIES}
    by_name = collections.defaultdict(list)
    category = {}
    for r in all_rows:
        name = r["name"].strip()
        if r["entitlement"] == "removed" or SPANSH_RENAMES.get(name, name) not in sold:
            continue
        pre_engineered = r["category"] == "mercgear"
        cat = base.get(r["symbol"].lower()) if pre_engineered else r["category"]
        if cat not in CATEGORIES:
            continue
        category.setdefault(name, cat)
        size = {"class": int(r["class"]), "rating": r["rating"]}
        mount = r["mount"] if r["mount"] in MOUNTS else ""  # one row has "mount" there
        if not mount and cat == "hardpoint":  # pre-engineered rows leave it out; the symbol has it
            parts = r["symbol"].lower().split("_")
            mount = next((m for m, p in SYMBOL_MOUNTS.items() if p in parts), "")
        if mount:
            size["mount"] = mount
        same = next((x for x in by_name[name] if {k: v for k, v in x.items() if k != "symbols"} == size), None)
        if same is None:
            same = dict(size, symbols=[])
            by_name[name].append(same)
        if not pre_engineered:
            same["symbols"].append(r["symbol"].lower())
    modules = [
        {
            "name": n,
            "category": category[n],
            "sizes": sorted(s, key=lambda x: (x["class"], x["rating"], MOUNTS[x.get("mount", "")])),
        }
        for n, s in by_name.items()
    ]
    modules.sort(key=lambda m: (CATEGORIES[m["category"]], m["name"]))
    ships = sorted(({"symbol": r["symbol"].lower(), "name": r["name"].strip()} for r in rows("shipyard.csv")),
                   key=lambda s: s["name"])
    out = {
        "source": f"EDCD/FDevIDs@{head(fdev)} (scripts/gen-outfitting.py)",
        "modules": modules,
        "ships": ships,
    }
    dst = os.path.join(ROOT, "data", "catalog", "outfitting.json")
    with open(dst, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=1, ensure_ascii=False)
        f.write("\n")
    print(f"{len(modules)} modules, {len(ships)} ships → {os.path.normpath(dst)}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    main(sys.argv[1])
