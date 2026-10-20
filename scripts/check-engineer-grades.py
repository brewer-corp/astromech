#!/usr/bin/env python3
"""Compare each ship engineer's top grade per module in data/catalog/engineering.json with the
Elite Dangerous wiki's Engineers table (astromech-qhi4). Prints the grades that differ.

Usage: scripts/check-engineer-grades.py [wiki.json]
Without a file it fetches the page's wikitext from the wiki's API (one request).
"""
import json, os, re, sys, urllib.request

URL = "https://elite-dangerous.fandom.com/api.php?action=parse&page=Engineers&prop=wikitext&format=json"
# wiki module name → our group name (both as keys)
ALIAS = {"beamlasers": "beamlaser", "burstlasers": "burstlaser", "pulselasers": "pulselaser",
         "afmu": "autofieldmaintenanceunit", "ecm": "electroniccountermeasure",
         "manifestscanner": "cargoscanner", "seekermissilerack": "missilerack"}
# our groups the wiki folds into another entry (None: not on the wiki)
VARIANT = {"advancedmulticannon": "multicannon", "advancedmissilerack": "missilerack",
           "biweaveshieldgenerator": "shieldgenerator", "prismaticshieldgenerator": "shieldgenerator",
           "causticsinklauncher": None}


def key(s):
    return re.sub(r"[^a-z0-9]", "", s.lower())


def wiki_grades(wikitext):
    table = wikitext.split("=== Ship Engineers ===")[1].split("\n===")[0]
    out = {}
    for row in table.split("\n|-")[1:]:
        m = re.search(r"\[\[([^\]|]+)", row)
        if m:
            out[m.group(1)] = {ALIAS.get(key(k), key(k)): int(g) for k, g in re.findall(r"\*\s*([^\n(]+?)\s*\(G(\d)\)", row)}
    return out


def our_grades(eng):
    out = {}
    for g in eng["groups"].values():
        mod = VARIANT.get(key(g["name"]), key(g["name"]))
        if not mod:
            continue
        for grades in g.get("engineers", {}).values():
            for grade, names in grades.items():
                for n in names:
                    d = out.setdefault(key(n), {})
                    d[mod] = max(d.get(mod, 0), int(grade))
    return out


def main():
    if len(sys.argv) > 1:
        page = json.load(open(sys.argv[1]))
    else:
        req = urllib.request.Request(URL, headers={"User-Agent": "astromech (+https://github.com/brewer-corp/astromech)"})
        page = json.load(urllib.request.urlopen(req))
    wiki = wiki_grades(page["parse"]["wikitext"]["*"])
    eng = json.load(open(os.path.join(os.path.dirname(__file__), "..", "data", "catalog", "engineering.json")))
    ours = our_grades(eng)
    agree, differ = 0, []
    for name, w in wiki.items():
        o = ours.get(key(name), {})
        for m in sorted(set(w) | set(o)):
            if w.get(m) == o.get(m):
                agree += 1
            else:
                differ.append(f"{name}: {m} wiki G{w.get(m)} ours G{o.get(m)}")
    print(f"{agree} engineer/module top grades agree across {len(wiki)} engineers; {len(differ)} differ")
    print("\n".join(differ))


if __name__ == "__main__":
    main()
