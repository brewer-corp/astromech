#!/usr/bin/env python3
"""Generate data/catalog/outfitting.json: every module a station can sell (by name, with each
class, rating and mount it comes in, and the journal symbols of each) and every ship, for the outfitting and shipyard lookups.

Usage: scripts/gen-outfitting.py <FDevIDs checkout>

Source (player-visible game data, see NOTICE.md): EDCD/FDevIDs outfitting.csv and shipyard.csv.
Left out: ship-specific armour, pre-engineered Powerplay modules ("mercgear") and modules
FDevIDs marks removed.
"""
import collections, csv, json, os, subprocess, sys

CATEGORIES = {"standard": 0, "internal": 1, "hardpoint": 2, "utility": 3}
MOUNTS = {"Fixed": 0, "Gimballed": 1, "Turreted": 2, "": 3}


def head(path):
    try:
        return subprocess.check_output(["git", "-C", path, "rev-parse", "--short", "HEAD"], text=True).strip()
    except Exception:
        return "unknown"


def main(fdev):
    rows = lambda p: list(csv.DictReader(open(os.path.join(fdev, p), encoding="utf-8")))
    by_name = collections.defaultdict(list)
    category = {}
    for r in rows("outfitting.csv"):
        if r["ship"] or r["category"] not in CATEGORIES or r["entitlement"] in ("removed", "?"):
            continue
        name = r["name"].strip()
        category.setdefault(name, r["category"])
        size = {"class": int(r["class"]), "rating": r["rating"]}
        if r["mount"] in MOUNTS and r["mount"]:  # one row has "mount" there
            size["mount"] = r["mount"]
        same = next((x for x in by_name[name] if {k: v for k, v in x.items() if k != "symbols"} == size), None)
        if same is None:
            same = dict(size, symbols=[])
            by_name[name].append(same)
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
    dst = os.path.join(os.path.dirname(__file__), "..", "data", "catalog", "outfitting.json")
    with open(dst, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=1, ensure_ascii=False)
        f.write("\n")
    print(f"{len(modules)} modules, {len(ships)} ships → {os.path.normpath(dst)}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    main(sys.argv[1])
