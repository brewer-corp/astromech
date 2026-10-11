#!/usr/bin/env python3
"""Generate data/catalog/activities.json from the catalogue in docs/V1-DESIGN.md §3.3.

Each activity lists Elite binding names; ★ marks "important" (shown under "Not bound yet" when
missing). "(or `A` / `B`)" after an action lists alternatives that also count as bound.
"""
import json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
doc = open(os.path.join(HERE, "..", "docs", "V1-DESIGN.md"), encoding="utf-8").read()
sec = doc[doc.index("### 3.3 Activities"):doc.index("All names were checked")]
cat = {c["id"] for c in json.load(open(os.path.join(HERE, "..", "data", "catalog", "controls.json")))["controls"]}
acts = []
for m in re.finditer(r"\*\*([^*\n]+)\*\*\n(.*?)(?=\n\n)", sec, re.S):
    title, body = m.group(1).strip(), m.group(2)
    if title.startswith("Not bound") or title.startswith("Elsewhere"):
        continue
    items = []
    # Split on commas outside parentheses.
    depth, cur, parts = 0, "", []
    for ch in body.replace("\n", " "):
        depth += ch == "("
        depth -= ch == ")"
        if ch == "," and depth == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    parts.append(cur)
    for p in parts:
        names = re.findall(r"`([A-Za-z0-9_]+)`", p.split("(")[0])
        alts = re.findall(r"`([A-Za-z0-9_]+)`", p.split("(", 1)[1]) if "(" in p and "or `" in p else []
        # "`X_Raw` (or `_Increase` / `_Decrease`)": suffixes replace the "X_Raw" part.
        alts = [a if not a.startswith("_") else re.sub(r"_Raw$", "", names[0]) + a for a in alts]
        star = "★" in p
        for n in names:
            items.append({"id": n, "important": star, "alternatives": alts})
    unknown = [i["id"] for i in items if i["id"] not in cat] + [a for i in items for a in i["alternatives"] if a not in cat]
    acts.append({"id": re.sub(r"[^a-z]+", "-", title.lower()).strip("-"), "title": title, "actions": items, "not_in_catalogue": unknown})
out = os.path.join(HERE, "..", "data", "catalog", "activities.json")
json.dump({"source": "docs/V1-DESIGN.md §3.3 (first draft)", "activities": acts}, open(out, "w"), indent=1)
for a in acts:
    print(f"{a['title']}: {len(a['actions'])} actions, unknown {a['not_in_catalogue']}")
