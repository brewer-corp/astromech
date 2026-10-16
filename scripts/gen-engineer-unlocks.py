#!/usr/bin/env python3
"""Generate data/catalog/engineer-unlocks.json: how to find, get invited by and unlock every ship
and pilot-equipment (suit) engineer, from the Elite Dangerous wiki's Engineers page ("Engineer
Summary" tables). Player-visible game facts; credited in NOTICE.md.

Usage: scripts/gen-engineer-unlocks.py [wiki.json]
Without a file it fetches the page's wikitext from the wiki's API (one request).
"""
import json, os, re, sys, urllib.request

URL = "https://elite-dangerous.fandom.com/api.php?action=parse&page=Engineers&prop=wikitext&format=json"


def plain(s):
    s = re.sub(r"<ref[^>]*/>|<ref[^>]*>.*?</ref>", "", s, flags=re.S)
    s = re.sub(r"\[\[([^\]|]+)\|([^\]]+)\]\]", r"\2", s)
    s = re.sub(r"\[\[([^\]]+)\]\]", r"\1", s)
    s = s.replace("'''", "").replace("''", "")
    return re.sub(r"\s+", " ", s).strip()


def rows(table):
    out = []
    for row in table.split("\n|-")[1:]:
        cells = [plain(c) for c in re.split(r"\n\|", "\n" + row.split("\n|}")[0].strip("\n"))[1:]]
        if cells:
            out.append(cells)
    return out


def referrers(find):
    if find.lower().startswith("common knowledge"):
        return []
    m = re.match(r'Learned from (.+)', find)
    if not m:
        sys.exit(f"can't read 'How to find': {find!r}")
    return [n.strip().strip("\"").strip() for n in re.split(r",\s*&\s*|,\s*|\s*&\s*|\s+and\s+", m.group(1)) if n.strip()]


def system(loc):
    return re.sub(r"\s*\(Permit Required\)", "", loc).strip(), "Permit Required" in loc


def main():
    if len(sys.argv) > 1:
        page = json.load(open(sys.argv[1]))
    else:
        req = urllib.request.Request(URL, headers={"User-Agent": "astromech (+https://github.com/spadaval/astromech)"})
        page = json.load(urllib.request.urlopen(req))
    text = page["parse"]["wikitext"]["*"]
    summary = text.split("== Engineer Summary ==")[1]
    ship_t = summary.split("=== Ship Engineers ===")[1].split("=== Pilot Equipment Engineers ===")[0]
    suit_t = summary.split("=== Pilot Equipment Engineers ===")[1].split("\n==")[0]
    out = []
    # Engineer | Modifies up to | Location | How to find | How to get invite | How to unlock | How to gain reputation
    for c in rows(ship_t):
        sys_name, permit = system(c[2])
        out.append({"name": c[0], "kind": "ship", "system": sys_name, "permit": permit,
                    "introduced_by": referrers(c[3]), "invite": c[4], "unlock": c[5], "reputation": c[6]})
    # Engineer | Modifications | Location | How to find | How to get invite | How to get referral
    for c in rows(suit_t):
        sys_name, permit = system(c[2])
        referral = None if c[5] in ("N/A", "") else c[5]
        out.append({"name": c[0], "kind": "suit", "system": sys_name, "permit": permit,
                    "introduced_by": referrers(c[3]), "invite": c[4], "referral": referral})
    names = {e["name"] for e in out}
    for e in out:
        for r in e["introduced_by"]:
            if r not in names:
                sys.exit(f"{e['name']} is introduced by unknown engineer {r!r}")
    dst = os.path.join(os.path.dirname(__file__), "..", "data", "catalog", "engineer-unlocks.json")
    with open(dst, "w", encoding="utf-8") as f:
        json.dump({"source": "Elite Dangerous wiki, Engineers page, Engineer Summary (scripts/gen-engineer-unlocks.py, "
                             "16 Oct 2026)", "engineers": out}, f, indent=1, ensure_ascii=False)
        f.write("\n")
    print(f"{sum(e['kind'] == 'ship' for e in out)} ship and {sum(e['kind'] == 'suit' for e in out)} suit engineers → "
          f"{os.path.normpath(dst)}")


if __name__ == "__main__":
    main()
