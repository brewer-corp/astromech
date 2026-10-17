#!/usr/bin/env python3
"""Generate data/catalog/odyssey.json: Odyssey on-foot micro-resources (names, bartender barter and
sale values), suit and weapon grade upgrades, suit and weapon modifications, and the on-foot
engineers (where, what they fit, how to get invited and referred, and who refers whom).

Usage: scripts/gen-odyssey.py <cache dir> [<out>]

Sources (player-visible game facts, see NOTICE.md), fetched over HTTP and kept in <cache dir>:
- EDCD/FDevIDs microresources.csv (journal symbols, categories, English names) and engineers.csv
  (on-foot engineer ids 4000xx and their system addresses).
- The Elite Dangerous Wiki (elite-dangerous.fandom.com) via its MediaWiki API: each suit and
  weapon page's Upgrades table, each modification page's cost and engineer list, each item's
  infobox (barter value, sale value), and the Engineers page's on-foot tables.
Facts are taken (numbers, names) plus the Engineers page's short invite and referral task
sentences, which are copied as written (CC BY-SA, credited in NOTICE.md). Facts the sources
disagree on, or leave blank, are listed under "uncertain" in the output.
"""
import csv, datetime, io, json, os, re, sys, urllib.parse, urllib.request

WIKI = "https://elite-dangerous.fandom.com/api.php"
FDEV = "https://raw.githubusercontent.com/EDCD/FDevIDs/master/"
UA = "astromech gen-odyssey (github.com/spadaval/astromech)"

SUITS = ["Maverick Suit", "Dominator Suit", "Artemis Suit"]
WEAPONS = [
    "Karma AR-50", "Karma C-44", "Karma L-6", "Karma P-15",
    "Manticore Executioner", "Manticore Intimidator", "Manticore Oppressor", "Manticore Tormentor",
    "TK Aphelion", "TK Eclipse", "TK Zenith",
]
# Weapon modification costs differ by maker; the wiki splits them under "=== <Maker> weapons ===".
MAKERS = {"Karma": "kinematic", "Manticore": "manticore", "TK": "takada"}
MAKER_HEADINGS = {"kinematic armaments": "kinematic", "kinematic": "kinematic", "karma": "kinematic",
                  "takada": "takada", "manticore": "manticore"}
SUIT_MODS = [
    "Added Melee Damage", "Combat Movement Speed", "Damage Resistance", "Enhanced Tracking",
    "Extra Ammo Capacity", "Extra Backpack Capacity", "Faster Shield Regen", "Improved Battery Capacity",
    "Improved Jump Assist", "Increased Air Reserves", "Increased Sprint Duration", "Night Vision",
    "Quieter Footsteps", "Reduced Tool Battery Consumption",
]
WEAPON_MODS = [
    "Audio Masking", "Faster Handling", "Greater Range", "Headshot Damage", "Improved Hip Fire Accuracy",
    "Magazine Size", "Noise Suppressor", "Reload Speed", "Scope", "Stability", "Stowed Reloading",
]
# The names the journal logs (SuitLoadout, BuySuit, BuyWeapon, ...), lowercase, for the owned gear
# on the On foot page. Suits are `<symbol>_class<grade>`; mods are `suit_*` / `weapon_*`. Sources:
# EDCD/EDDI DataDefinitions (Suit.cs, HandWeapon.cs, SuitMod.cs, HandWeaponMod.cs), cross-checked
# with rster2002/ed-journals (suit_mod.rs, weapon_mod.rs) and the example lines in
# jixxed/ed-journal-schemas (schemas/BuySuit, BuyWeapon, SuitLoadout, SwitchSuitLoadout).
JOURNAL_GEAR = {
    "Maverick Suit": "utilitysuit", "Dominator Suit": "tacticalsuit", "Artemis Suit": "explorationsuit",
    "Karma AR-50": "wpn_m_assaultrifle_kinetic_fauto", "Karma C-44": "wpn_m_submachinegun_kinetic_fauto",
    "Karma L-6": "wpn_m_launcher_rocket_sauto", "Karma P-15": "wpn_s_pistol_kinetic_sauto",
    "Manticore Executioner": "wpn_m_sniper_plasma_charged",
    "Manticore Intimidator": "wpn_m_shotgun_plasma_doublebarrel",
    "Manticore Oppressor": "wpn_m_assaultrifle_plasma_fauto", "Manticore Tormentor": "wpn_s_pistol_plasma_charged",
    "TK Aphelion": "wpn_m_assaultrifle_laser_fauto", "TK Eclipse": "wpn_m_submachinegun_laser_fauto",
    "TK Zenith": "wpn_s_pistol_laser_sauto",
}
JOURNAL_MODS = {
    "Added Melee Damage": "suit_increasedmeleedamage", "Combat Movement Speed": "suit_adsmovementspeed",
    "Damage Resistance": "suit_improvedarmourrating", "Enhanced Tracking": "suit_improvedradar",
    "Extra Ammo Capacity": "suit_increasedammoreserves", "Extra Backpack Capacity": "suit_backpackcapacity",
    "Faster Shield Regen": "suit_increasedshieldregen", "Improved Battery Capacity": "suit_increasedbatterycapacity",
    "Improved Jump Assist": "suit_improvedjumpassist", "Increased Air Reserves": "suit_increasedo2capacity",
    "Increased Sprint Duration": "suit_increasedsprintduration", "Night Vision": "suit_nightvision",
    "Quieter Footsteps": "suit_quieterfootsteps",
    "Reduced Tool Battery Consumption": "suit_reducedtoolbatteryconsumption",
    "Audio Masking": "weapon_suppression_unpressurised", "Faster Handling": "weapon_handling",
    "Greater Range": "weapon_range", "Headshot Damage": "weapon_headshotdamage",
    "Improved Hip Fire Accuracy": "weapon_accuracy", "Magazine Size": "weapon_clipsize",
    "Noise Suppressor": "weapon_suppression_pressurised", "Reload Speed": "weapon_reloadspeed",
    "Scope": "weapon_scope", "Stability": "weapon_stability", "Stowed Reloading": "weapon_backpackreloading",
}
# Wiki page titles that differ from the in-game name.
PAGE = {"Night Vision": "Night Vision (Modification)"}
# Engineers-page names for mods that differ from the mod's in-game name.
MOD_ALIASES = {
    "Reduced Energy Use": "Reduced Tool Battery Consumption",
    "Shield Regen": "Faster Shield Regen",
    "Night Vision (Modification)": "Night Vision",
}
# FDevIDs English names that differ from the wiki page (the in-game name).
WIKI_NAMES = {
    "Insight": "InSight",
    "Insight Data Bank": "InSight Data Bank",
    "Insight Entertainment Suite": "InSight Entertainment Suite",
    "Employment History": "Employee History",
    "Seed Geneaology": "Seed Genealogy",
}


def key(s):
    return re.sub(r"[^a-z0-9]", "", s.lower())


def fetch(cache, name, url):
    path = os.path.join(cache, name)
    if not os.path.exists(path):
        req = urllib.request.Request(url, headers={"User-Agent": UA})
        with urllib.request.urlopen(req, timeout=60) as r:
            data = r.read()
        with open(path, "wb") as f:
            f.write(data)
    with open(path, encoding="utf-8") as f:
        return f.read()


def wikitext(cache, title):
    q = urllib.parse.urlencode({"action": "parse", "format": "json", "prop": "wikitext", "redirects": 1, "page": title})
    # Titles are case-sensitive ("Insight" is missing, "InSight" is not), so the file name is too.
    d = json.loads(fetch(cache, "wiki-" + urllib.parse.quote(title, safe="") + ".json", f"{WIKI}?{q}"))
    if "error" in d:
        return None
    return d["parse"]["wikitext"]["*"]


def link_text(s):
    """`[[A|B]]` or `[[A]]` -> A (the page, which is the in-game name for items)."""
    return re.sub(r"\[\[([^]|]+)(\|[^]]*)?\]\]", r"\1", s).strip()


def section(text, heading):
    """The body under `heading`, up to the next heading of the same or a higher level."""
    m = re.search(r"^(==+)\s*" + re.escape(heading) + r"\s*==+\s*$", text, re.M)
    if not m:
        return ""
    level = len(m.group(1))
    end = re.compile(r"^={2,%d}[^=]" % level, re.M).search(text, m.end())
    return text[m.end() : end.start() if end else len(text)]


def credits(s):
    m = re.search(r"([\d,?]+)\s*CR", s)
    if not m:
        return None
    v = m.group(1).replace(",", "")
    return int(v) if v.isdigit() else None


def parse_cost(cell, item_key, uncertain, where):
    """'2x [[Graphene]]<br/>600,000 CR' -> ({symbol: 2}, 600000)."""
    mats = {}
    cr = None
    for part in re.split(r"<br\s*/?>|\n\*", cell):
        part = part.strip(" *")
        if not part:
            continue
        m = re.match(r"(\d+)\s*x\s*(.+)", part)
        if m:
            name = link_text(m.group(2))
            sym = item_key(name)
            shown = re.search(r"\|([^]]+)\]\]", m.group(2))
            if not sym and shown and item_key(shown.group(1)):
                # A mislinked item ("[[Greater Range|RDX]]"): trust the text the reader sees.
                uncertain.append(f"{where}: wiki links {shown.group(1)} to {name}")
                sym = item_key(shown.group(1))
            if not sym:
                sys.exit(f"{where}: unknown item {name!r}")
            mats[sym] = mats.get(sym, 0) + int(m.group(1))
        elif "CR" in part:
            cr = credits(part)
            if cr is None:
                uncertain.append(f"{where}: credits not on the wiki")
    if mats and cr is None and "CR" not in cell:
        uncertain.append(f"{where}: credits not on the wiki")
    return mats, cr


def upgrades(text, item_key, uncertain, where):
    body = section(text, "Upgrades")
    out = []
    for row in body.split("|-")[1:]:
        cells = [l[1:].strip() for l in row.strip().splitlines() if l.startswith("|") and not l.startswith("|}")]
        if len(cells) < 2 or not cells[0].isdigit() or cells[1] in ("N/A", ""):
            continue
        grade = int(cells[0])
        mats, cr = parse_cost(cells[1], item_key, uncertain, f"{where} grade {grade}")
        out.append({"grade": grade, "materials": mats, "credits": cr})
    if [u["grade"] for u in out] != [2, 3, 4, 5]:
        sys.exit(f"{where}: expected grades 2-5, got {[u['grade'] for u in out]}")
    return out


def wiki_engineers(text):
    return [link_text(l.strip("* ")) for l in text.splitlines() if l.strip().startswith("*") and "[[" in l]


def table_rows(text):
    rows = []
    for row in text.split("|-")[1:]:
        cells, cur = [], None
        for l in row.strip().splitlines():
            if l.startswith("|}"):
                break
            if l.startswith("|") or l.startswith("!"):
                if cur is not None:
                    cells.append(cur)
                cur = l[1:].strip()
            elif cur is not None:
                cur += "\n" + l
        if cur is not None:
            cells.append(cur)
        rows.append(cells)
    return rows


def main(cache, out):
    os.makedirs(cache, exist_ok=True)
    uncertain = []

    micro = list(csv.DictReader(io.StringIO(fetch(cache, "microresources.csv", FDEV + "microresources.csv"))))
    names = {key(r["English name"]): r["symbol"].lower() for r in micro}
    names.update({key(w): names[key(f)] for f, w in WIKI_NAMES.items()})

    def item_key(name):
        return names.get(key(name))

    items = {}
    for r in sorted(micro, key=lambda r: r["symbol"].lower()):
        sym = r["symbol"].lower()
        name = WIKI_NAMES.get(r["English name"], r["English name"])
        text = wikitext(cache, name) or ""
        box = dict(re.findall(r"^\|\s*(\w+)\s*=\s*(.*?)\s*$", text, re.M))
        barter = int(box["barter"]) if box.get("barter", "").isdigit() else None
        value = box.get("value", "").replace(",", "")
        value = int(value) if value.isdigit() else None
        cat = r["category"].lower()
        # The wiki gives barter values for assets (components) only; goods and data are sold for
        # credits. Consumables are neither bartered nor sold.
        if cat != "consumable" and value is None:
            uncertain.append(f"item {name}: sale value not on the wiki")
        if cat == "component" and barter is None:
            uncertain.append(f"item {name}: barter value not on the wiki")
        items[sym] = {"name": name, "category": cat, "barter": barter, "value": value}

    suits = []
    for s in SUITS:
        text = wikitext(cache, s)
        suits.append({"id": key(s.replace(" Suit", "")), "name": s, "journal": JOURNAL_GEAR[s], "upgrades": upgrades(text, item_key, uncertain, s)})

    weapons = []
    for w in WEAPONS:
        text = wikitext(cache, w)
        maker = MAKERS[w.split()[0]]
        weapons.append({"id": key(w), "name": w, "journal": JOURNAL_GEAR[w], "maker": maker, "upgrades": upgrades(text, item_key, uncertain, w)})

    mod_engineers = {}
    mods = []
    for kind, names_ in (("suit", SUIT_MODS), ("weapon", WEAPON_MODS)):
        for m in names_:
            text = wikitext(cache, PAGE.get(m, m))
            if text is None:
                sys.exit(f"no wiki page for {m}")
            cost = section(text, "Modification Cost") or section(text, "Cost")
            parts = re.split(r"^===+\s*(.*?)\s*===+\s*$", cost, flags=re.M)
            costs = []
            if parts[0].strip(" \n") and "x" in parts[0]:
                mats, cr = parse_cost(parts[0], item_key, uncertain, m)
                if mats:
                    costs.append({"maker": None, "materials": mats, "credits": cr})
            for heading, body in zip(parts[1::2], parts[2::2]):
                h = re.sub(r"\s*weapons?$", "", link_text(heading).strip(), flags=re.I).lower()
                if h not in MAKER_HEADINGS:
                    sys.exit(f"{m}: unknown cost heading {heading!r}")
                mats, cr = parse_cost(body, item_key, uncertain, f"{m} ({heading})")
                costs.append({"maker": MAKER_HEADINGS[h], "materials": mats, "credits": cr})
            if not costs or any(not c["materials"] for c in costs):
                sys.exit(f"{m}: no modification cost")
            if kind == "weapon" and costs[0]["maker"] is None and len(costs) == 1:
                pass  # one cost for every maker
            elif kind == "weapon" and sorted(c["maker"] for c in costs) != ["kinematic", "manticore", "takada"]:
                uncertain.append(f"{m}: costs only for {sorted(str(c['maker']) for c in costs)}")
            mod_engineers[m] = set(wiki_engineers(section(text, "Availability")))
            mods.append({"id": key(m), "name": m, "kind": kind, "journal": JOURNAL_MODS[m], "costs": costs})

    # Engineers page: the overview table (system, mods) and the summary table (how to find, invite,
    # referral). The two tables and the mod pages do not always agree on who fits what.
    eng = wikitext(cache, "Engineers")
    fdev_eng = {
        key(r["name"]): r
        for r in csv.DictReader(io.StringIO(fetch(cache, "engineers.csv", FDEV + "engineers.csv")))
        if r["id"].startswith("4000")
    }
    overview = {}
    sec = section(eng, "Pilot Equipment Engineers")  # the first one: Engineers and Workshops
    for cells in table_rows(sec):
        if len(cells) >= 4 and cells[0].startswith("[["):
            overview[link_text(cells[0])] = {
                "system": cells[3].strip(),
                "mods": {MOD_ALIASES.get(link_text(x), link_text(x)) for x in wiki_engineers(cells[2])},
            }
    summary = {}
    sec2 = eng[eng.index("== Engineer Summary ==") :]
    sec2 = section(sec2, "Pilot Equipment Engineers")
    for cells in table_rows(sec2):
        if len(cells) >= 6 and cells[0].startswith("[["):
            summary[link_text(cells[0])] = {
                "mods": {MOD_ALIASES.get(x.strip(), x.strip()) for x in cells[1].split(",")},
                "system": cells[2].strip(),
                "find": link_text(cells[3]),
                "invite": link_text(cells[4]),
                "referral": link_text(cells[5]),
            }
    if len(summary) != 13 or set(summary) != set(overview):
        sys.exit(f"expected 13 on-foot engineers, got {sorted(summary)} / {sorted(overview)}")

    all_mods = {m["name"] for m in mods}
    engineers = []
    for name, s in summary.items():
        f = fdev_eng.get(key(name))
        if not f:
            sys.exit(f"{name} not in FDevIDs engineers.csv")
        from_mod_pages = {m for m, es in mod_engineers.items() if name in es}
        sources = [s["mods"], overview[name]["mods"], from_mod_pages]
        offered = set().union(*sources) & all_mods
        unknown = set().union(*sources) - all_mods
        if unknown:
            sys.exit(f"{name}: unknown mods {sorted(unknown)}")
        shaky = sorted(m for m in offered if not all(m in x for x in sources))
        for m in shaky:
            uncertain.append(f"engineer {name}: {m} (the wiki's tables and mod pages disagree)")
        if overview[name]["system"] != s["system"]:
            uncertain.append(f"engineer {name}: system {s['system']} or {overview[name]['system']}")
        found = s["find"]
        referred_by = []
        if found.lower().startswith("learned from"):
            referred_by = [link_text(x).strip() for x in re.split(r",|&|\band\b", found[len("learned from") :]) if x.strip()]
        referral = None if s["referral"] in ("N/A", "") else s["referral"]
        engineers.append(
            {
                "id": int(f["id"]),
                "name": name,
                "system": s["system"],
                "id64": int(f["system_address"]),
                "colonia": s["system"] in ("Deriso", "Desy", "Kojeara", "Einheriar"),
                "mods": sorted(key(m) for m in offered),
                "unconfirmed_mods": [key(m) for m in shaky],
                "referred_by": referred_by,
                "invite": s["invite"],
                "referral_task": referral,
            }
        )
    engineers.sort(key=lambda e: e["id"])
    by_name = {e["name"] for e in engineers}
    for e in engineers:
        for r in e["referred_by"]:
            if r not in by_name:
                sys.exit(f"{e['name']}: referred by unknown {r!r}")

    doc = {
        "source": "Elite Dangerous Wiki (elite-dangerous.fandom.com) and EDCD/FDevIDs, via scripts/gen-odyssey.py",
        "generated": datetime.date.today().isoformat(),
        "note": "Suit and weapon grade upgrades are bought at Pioneer Supplies; modifications are fitted by the engineers listed.",
        "items": items,
        "suits": suits,
        "weapons": weapons,
        "mods": mods,
        "engineers": engineers,
        "uncertain": sorted(set(uncertain)),
    }
    with open(out, "w", encoding="utf-8") as f:
        json.dump(doc, f, indent=1, ensure_ascii=False)
        f.write("\n")
    print(f"{out}: {len(items)} items, {len(suits)} suits, {len(weapons)} weapons, {len(mods)} mods, "
          f"{len(engineers)} engineers, {len(doc['uncertain'])} uncertain")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    here = os.path.dirname(os.path.abspath(__file__))
    main(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else os.path.join(here, "..", "data", "catalog", "odyssey.json"))
