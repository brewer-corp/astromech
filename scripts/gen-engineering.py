#!/usr/bin/env python3
"""Generate data/catalog/engineering.json: ship engineering blueprints, experimentals, which
module takes which, and display names for journal module / ship symbols. Also what each blueprint
grade and experimental does to a module's stats, for astromech-ship's calculations.

Usage: scripts/gen-engineering.py <coriolis-data checkout> <FDevIDs checkout>

Sources (player-visible game data, see NOTICE.md):
- EDCD/coriolis-data: modifications/{blueprints,specials,modules,modifications,modifierActions}.json,
  modules/*/*.json
- EDCD/FDevIDs: material.csv (journal material symbols), outfitting.csv, shipyard.csv
Material names in coriolis are display names; they are mapped to journal symbols (lowercase,
letters and digits only, the same key astromech-materials uses).
"""
import collections, csv, glob, json, os, re, subprocess, sys

# coriolis display-name typos / trailing spaces → FDevIDs display name
MATERIAL_ALIASES = {"adaptive encyptors capture": "adaptive encryptors capture"}
ENGINEER_ALIASES = {"Felicty Farseer": "Felicity Farseer"}
GROUP_NAMES = {"bh": "Armour", "mr": "Missile Rack"}
# coriolis-data's per-module engineers, corrected where the Elite Dangerous wiki disagrees
# (every engineer spot-checked on 16 Oct 2026, astromech-qhi4):
# - the manifest (cargo) scanner has the surface scanner's engineers; the wiki gives it the wake
#   scanner's engineers and grades;
# - Ram Tah does hatch breaker limpet controllers to G3, not G4.
SAME_ENGINEERS_AS = {"cs": "ws"}
MAX_GRADE = {("hb", "Ram Tah"): 3}


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
    stat_defs = load("modifications/modifications.json")
    actions = load("modifications/modifierActions.json")

    # Who offers each blueprint grade depends on the module: Overcharged on a beam laser is not
    # the same engineers as on a multi-cannon. Keep it per module group (astromech-qhi4).
    def group_engineers(g):
        out = {}
        for bp, b in sorted(g.get("blueprints", {}).items()):
            grades = {
                str(int(grade)): sorted(ENGINEER_ALIASES.get(e, e) for e in x.get("engineers", []))
                for grade, x in sorted(b.get("grades", {}).items(), key=lambda t: int(t[0]))
            }
            if any(grades.values()):
                out[bp] = grades
        return out

    blueprints = {}
    for fd, b in sorted(bps_raw.items()):
        grades = []
        for grade, x in sorted(b["grades"].items(), key=lambda t: int(t[0])):
            grades.append(
                {
                    "grade": int(grade),
                    "materials": recipe(x["components"]),
                    # stat -> [worst, best] roll, as a fraction (percentage stats) or an amount
                    "features": {k: v for k, v in sorted(x.get("features", {}).items())},
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

    def fixed_engineers(g):
        engs = group_engineers(mod_map[SAME_ENGINEERS_AS.get(g, g)])
        for (fg, who), top in MAX_GRADE.items():
            if fg == g:
                for grades in engs.values():
                    for grade, names in grades.items():
                        if int(grade) > top:
                            grades[grade] = [n for n in names if n != who]
        return engs

    groups = {}
    for g, m in sorted(mod_map.items()):
        bps = sorted(b for b in m.get("blueprints", {}) if b in blueprints)
        exps = [s for s in m.get("specials", []) if s in experimentals]
        if not bps and not exps:
            continue
        name = GROUP_NAMES.get(g) or (group_votes[g].most_common(1)[0][0] if group_votes[g] else g)
        engs = {b: grades for b, grades in fixed_engineers(g).items() if b in blueprints}
        groups[g] = {"name": name, "blueprints": bps, "experimentals": exps,
                     "modifications": sorted(m.get("modifications", [])), "engineers": engs}

    ships = {r["symbol"].lower(): r["name"].strip() for r in rows("shipyard.csv")}

    # How engineering changes each stat (coriolis Module.js semantics): "type" is percentage
    # (stored x10000) or numeric (x100); "method" multiplicative, additive or overwrite.
    modifiers = {
        k: {"type": d["type"], "method": d["method"], "higherbetter": d.get("higherbetter", False)}
        for k, d in sorted(stat_defs.items())
        if d["type"] in ("percentage", "numeric")
    }
    # What each experimental does on top of the blueprint (numeric effects only; the damage-type
    # overrides don't change any total astromech shows).
    effects = {
        sp: {k: v for k, v in sorted(actions[sp].items()) if isinstance(v, (int, float))}
        for sp in sorted(specials_raw)
        if sp in actions
    }

    out = {
        "source": f"EDCD/coriolis-data@{head(cor)}, EDCD/FDevIDs@{head(fdev)} (scripts/gen-engineering.py)",
        "blueprints": blueprints,
        "experimentals": experimentals,
        "groups": groups,
        "modifiers": modifiers,
        "effects": effects,
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
