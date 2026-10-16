#!/usr/bin/env python3
"""Generate data/catalog/ships.json: base stats of every ship and module that astromech-ship's
calculations use, keyed by journal symbol (lowercase).

Usage: scripts/gen-ships.py <coriolis-data checkout> <FDevIDs checkout>

Sources (player-visible game data, see NOTICE.md):
- EDCD/coriolis-data: ships/*.json, modules/*/*.json (module order from modules/index.js)
- EDCD/FDevIDs: shipyard.csv and outfitting.csv, to map coriolis's edID to journal symbols

Several coriolis modules share a symbol (a pre-engineered variant after the plain one). Coriolis's
journal import takes the first in its search order (standard, hardpoints, internal; group order as
in modules/index.js), so this does too.
"""
import csv, json, os, re, subprocess, sys

SHIP_FIELDS = {
    "hullMass": "hull_mass", "speed": "speed", "boost": "boost", "minthrust": "minthrust",
    "boostEnergy": "boost_energy", "baseShieldStrength": "base_shield", "baseArmour": "base_armour",
    "hardness": "hardness", "reserveFuelCapacity": "reserve_fuel",
}
# Module stats the calculations read (coriolis names, which are also the engineering stat names).
STATS = [
    "mass", "power", "pgen", "fuel", "cargo", "passengers", "integrity",
    "optmass", "maxfuel", "fuelmul", "fuelpower", "jumpboost",
    "minmass", "maxmass", "minmul", "optmul", "maxmul", "minmulspeed", "optmulspeed", "maxmulspeed",
    "engcap", "engrate", "syscap", "sysrate", "wepcap", "weprate",
    "explres", "kinres", "thermres", "causres", "shieldboost", "hullboost", "hullreinforcement",
    "protection", "shieldaddition", "regen", "brokenregen", "distdraw",
    "damage", "fireint", "burst", "burstrof", "charge", "clip", "reload", "roundspershot",
]


def head(path):
    try:
        return subprocess.check_output(["git", "-C", path, "rev-parse", "--short", "HEAD"], text=True).strip()
    except Exception:
        return "unknown"


def stats(m):
    out = {k: m[k] for k in STATS if isinstance(m.get(k), (int, float)) and not isinstance(m.get(k), bool)}
    return dict(sorted(out.items()))


def main(cor, fdev):
    load = lambda p: json.load(open(os.path.join(cor, p), encoding="utf-8"))
    rows = lambda p: list(csv.DictReader(open(os.path.join(fdev, p), encoding="utf-8")))
    outfit_sym = {int(r["id"]): r["symbol"].lower() for r in rows("outfitting.csv")}
    ship_sym = {int(r["id"]): r["symbol"].lower() for r in rows("shipyard.csv")}
    ship_by_name = {r["name"].strip(): r["symbol"].lower() for r in rows("shipyard.csv")}

    modules = {}
    index = open(os.path.join(cor, "modules", "index.js"), encoding="utf-8").read()
    for cat in ("standard", "hardpoints", "internal"):
        block = re.search(cat + r":\s*{(.*?)}", index, re.S).group(1)
        for path, grp in re.findall(r"require\('\./([\w/]+?)(?:\.json)?'\)\.(\w+)", block):
            for m in load(f"modules/{path}.json")[grp]:
                sym = m.get("symbol", "").lower()
                if not sym or sym in modules:
                    continue
                d = m.get("damagedist")
                if d and abs(sum(d.get(k, 0) for k in "AEKT") - 1) > 1e-9:
                    sys.exit(f"{sym}: damage types don't sum to 1 ({d})")
                entry = {"grp": m["grp"], "class": m["class"], "rating": m["rating"], "stats": stats(m)}
                if m.get("passive"):
                    entry["passive"] = True
                modules[sym] = entry

    ships = {}
    for f in sorted(os.listdir(os.path.join(cor, "ships"))):
        if not f.endswith(".json"):
            continue
        for s in load(f"ships/{f}").values():
            p = s["properties"]
            # coriolis has a few edIDs FDevIDs doesn't (Caspian Explorer); fall back to the name
            sym = ship_sym.get(s["edID"]) or ship_by_name.get(p["name"])
            if sym is None:
                sys.exit(f"{f}: edID {s['edID']} / {p['name']!r} not in FDevIDs shipyard.csv")
            ship = {"name": p["name"]}
            ship.update({v: p[k] for k, v in SHIP_FIELDS.items() if k in p})
            for b in s["bulkheads"]:
                bsym = outfit_sym.get(b["edID"])
                if bsym is None:
                    print(f"skipped: {f} bulkhead {b['name']!r}, edID {b['edID']} not in FDevIDs", file=sys.stderr)
                    continue
                modules[bsym] = {"grp": "bh", "class": 1, "rating": "I", "stats": stats(b)}
            ships[sym] = ship

    out = {
        "source": f"EDCD/coriolis-data@{head(cor)}, EDCD/FDevIDs@{head(fdev)} (scripts/gen-ships.py)",
        "ships": dict(sorted(ships.items())),
        "modules": dict(sorted(modules.items())),
    }
    dst = os.path.join(os.path.dirname(__file__), "..", "data", "catalog", "ships.json")
    with open(dst, "w", encoding="utf-8") as f:
        # One ship or module per line: compact, and diffs stay readable.
        f.write("{\n" + f' "source": {json.dumps(out["source"])},\n')
        for key in ("ships", "modules"):
            items = list(out[key].items())
            f.write(f' "{key}": {{\n')
            for i, (k, v) in enumerate(items):
                sep = "," if i < len(items) - 1 else ""
                f.write(f"  {json.dumps(k)}: {json.dumps(v, ensure_ascii=False, separators=(',', ':'))}{sep}\n")
            f.write(" }" + ("," if key == "ships" else "") + "\n")
        f.write("}\n")
    print(f"{len(ships)} ships, {len(modules)} modules → {os.path.normpath(dst)}")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    main(sys.argv[1], sys.argv[2])
