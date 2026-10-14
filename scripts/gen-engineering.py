#!/usr/bin/env python3
"""Generate data/catalog/engineering.json: ship engineering blueprints, experimentals, which
module takes which, and display names for journal module / ship symbols.

Usage: scripts/gen-engineering.py <coriolis-data checkout> <FDevIDs checkout>

Sources (player-visible game data, see NOTICE.md):
- EDCD/coriolis-data: modifications/{blueprints,specials,modules}.json, modules/*/*.json
- EDCD/FDevIDs: material.csv (journal material symbols), outfitting.csv, shipyard.csv
Material names in coriolis are display names; they are mapped to journal symbols (lowercase,
letters and digits only, the same key astromech-materials uses).
"""
import collections, csv, glob, json, os, re, subprocess, sys

# coriolis display-name typos / trailing spaces → FDevIDs display name
MATERIAL_ALIASES = {"adaptive encyptors capture": "adaptive encryptors capture"}
GROUP_NAMES = {"bh": "Armour", "mr": "Missile Rack"}


def key(s):
    return re.sub(r"[^a-z0-9]", "", s.lower())


def head(path):
    try:
        return subprocess.check_output(["git", "-C", path, "rev-parse", "--short", "HEAD"], text=True).strip()
    except Exception:
        return "unknown"


def main(cor, fdev):
    load = lambda p: json.load(open(os.path.join(cor, p), encoding="utf-8"))
    rows = lambda p: list(csv.DictReader(open(os.path.join(fdev, p), encoding="utf-8")))

    mats = {r["name"].strip().lower(): key(r["symbol"]) for r in rows("material.csv")}

    def mat(name):
        n = name.strip().lower()
        n = MATERIAL_ALIASES.get(n, n)
        if n not in mats:
            sys.exit(f"unknown material {name!r}")
        return mats[n]

    def recipe(components):
        return {mat(k): v for k, v in sorted(components.items())}

    bps_raw = load("modifications/blueprints.json")
    specials_raw = load("modifications/specials.json")
    mod_map = load("modifications/modules.json")

    engineers = collections.defaultdict(lambda: collections.defaultdict(set))
    for g in mod_map.values():
        for bp, b in g.get("blueprints", {}).items():
            for grade, x in b.get("grades", {}).items():
                engineers[bp][int(grade)].update(x.get("engineers", []))

    blueprints = {}
    for fd, b in sorted(bps_raw.items()):
        grades = []
        for grade, x in sorted(b["grades"].items(), key=lambda t: int(t[0])):
            grades.append(
                {
                    "grade": int(grade),
                    "materials": recipe(x["components"]),
                    "engineers": sorted(engineers[fd][int(grade)]),
                }
            )
        blueprints[fd] = {"name": b["name"], "grades": grades}

    experimentals = {
        fd: {"name": s["name"], "materials": recipe(s["components"])}
        for fd, s in sorted(specials_raw.items())
        if s.get("components")
    }

    # symbol → coriolis group, from coriolis module files
    sym_grp = {}
    for f in glob.glob(os.path.join(cor, "modules", "*", "*.json")):
        for v in json.load(open(f, encoding="utf-8")).values():
            for x in v:
                sym_grp[x["symbol"].lower()] = x["grp"]
    # prefix (before _size) → group, when every symbol with that prefix agrees
    by_prefix = collections.defaultdict(set)
    for s, g in sym_grp.items():
        by_prefix[s.split("_size")[0]].add(g)

    def group_of(sym):
        s = sym.lower()
        if "_armour_" in s:
            return "bh"
        for cand in (s, re.sub(r"_free$", "", s)):
            if cand in sym_grp:
                return sym_grp[cand]
        p = by_prefix.get(s.split("_size")[0])
        return next(iter(p)) if p and len(p) == 1 else None

    modules, group_votes = {}, collections.defaultdict(collections.Counter)
    for r in rows("outfitting.csv"):
        g = group_of(r["symbol"])
        if g is None or g not in mod_map:
            continue
        entry = {"group": g, "name": r["name"].strip()}
        if r["class"]:
            entry["class"] = int(r["class"])
        if r["rating"]:
            entry["rating"] = r["rating"]
        if r["mount"]:
            entry["mount"] = r["mount"]
        modules[r["symbol"].lower()] = entry
        group_votes[g][r["name"].strip()] += 1

    groups = {}
    for g, m in sorted(mod_map.items()):
        bps = sorted(b for b in m.get("blueprints", {}) if b in blueprints)
        exps = [s for s in m.get("specials", []) if s in experimentals]
        if not bps and not exps:
            continue
        name = GROUP_NAMES.get(g) or (group_votes[g].most_common(1)[0][0] if group_votes[g] else g)
        groups[g] = {"name": name, "blueprints": bps, "experimentals": exps}

    ships = {r["symbol"].lower(): r["name"].strip() for r in rows("shipyard.csv")}

    out = {
        "source": f"EDCD/coriolis-data@{head(cor)}, EDCD/FDevIDs@{head(fdev)} (scripts/gen-engineering.py)",
        "blueprints": blueprints,
        "experimentals": experimentals,
        "groups": groups,
        "modules": dict(sorted(modules.items())),
        "ships": dict(sorted(ships.items())),
    }
    dst = os.path.join(os.path.dirname(__file__), "..", "data", "catalog", "engineering.json")
    with open(dst, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=1, ensure_ascii=False)
        f.write("\n")
    print(f"{len(blueprints)} blueprints, {len(experimentals)} experimentals, {len(groups)} groups, "
          f"{len(modules)} modules, {len(ships)} ships → {os.path.normpath(dst)}")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    main(sys.argv[1], sys.argv[2])
