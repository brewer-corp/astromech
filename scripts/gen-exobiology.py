#!/usr/bin/env python3
"""Generate data/catalog/exobiology.json: every exobiology genus with its journal symbol and
colony range (the least distance between samples), and its species with their base value
(Vista Genomics, before the first-logged bonus) and, for the Odyssey genera, the conditions they
grow in (atmosphere, body class, star, gravity, temperature, pressure, volcanism).

Usage: scripts/gen-exobiology.py <EDMC-ExplorerLite checkout> <commit> [wiki wikitext file]

Sources (see NOTICE.md):
- Values and colony ranges: the Elite Dangerous wiki's "Exobiology Sample Values and Details"
  page (elite-dangerous.fandom.com), read through its MediaWiki API (or the saved wikitext).
  Only the numbers are used.
- Conditions: dwomble/EDMC-ExplorerLite (MIT), `data/species_rulesets.json`, transcribed by its
  author from public sources. Its own values (`explorer/valuation/exobiology_data.py`, read with
  `ast.literal_eval`, never imported) are compared with the wiki's; a disagreement is listed
  under `uncertain` and the wiki's value is kept.
- Genus symbols (`$Codex_Ent_Stratum_Genus_Name;`): the journal's `Genus` values, as listed by
  Elite Observatory (Xjph/ObservatoryCore, MIT).
"""
import ast, json, os, re, sys, urllib.parse, urllib.request

WIKI_PAGE = "Exobiology Sample Values and Details"
WIKI_API = "https://elite-dangerous.fandom.com/api.php"

# Journal genus symbol (without `$Codex_Ent_`, `_Genus`, `_Name;`) for each genus.
CODEX = {
    "Aleoida": ["Aleoids"],
    "Bacterium": ["Bacterial"],
    "Cactoida": ["Cactoid"],
    # The game spells it Clepeus in the symbol.
    "Clypeus": ["Clepeus", "Clypeus"],
    "Concha": ["Conchas"],
    "Electricae": ["Electricae"],
    "Fonticulua": ["Fonticulus"],
    "Frutexa": ["Shrubs"],
    "Fumerola": ["Fumerolas"],
    "Fungoida": ["Fungoids"],
    "Osseus": ["Osseus"],
    "Recepta": ["Recepta"],
    "Stratum": ["Stratum"],
    "Tubus": ["Tubus"],
    "Tussock": ["Tussocks"],
    # Horizons-era surface biology, sampled the same way; no prediction rules.
    "Amphora Plant": ["Vents"],
    "Anemone": ["Sphere"],
    "Bark Mound": ["Cone"],
    "Brain Tree": ["Brancae"],
    "Crystalline Shard": ["Ground_Struct_Ice"],
    "Sinuous Tuber": ["Tube"],
}

# Horizons-era genera whose species the game names "<epithet> <genus>" ("Roseum Brain Tree").
EPITHET_FIRST = {"Anemone", "Brain Tree", "Sinuous Tuber"}


def wikitext(path):
    if path:
        return open(path).read()
    q = urllib.parse.urlencode({"action": "parse", "page": WIKI_PAGE, "prop": "wikitext", "format": "json"})
    req = urllib.request.Request(f"{WIKI_API}?{q}", headers={"User-Agent": "astromech gen-exobiology.py"})
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.load(r)["parse"]["wikitext"]["*"]


def wiki_tables(text):
    """{genus: (colony range m, {species name: value})} from the page's per-genus sections."""
    out = {}
    parts = re.split(r"^===\[\[([^\]|]+)(?:\|[^\]]*)?\]\]===\s*$", text, flags=re.M)
    for genus, body in zip(parts[1::2], parts[2::2]):
        genus = genus.strip()
        if genus not in CODEX:
            continue  # Thargoid biology isn't sampled with the genetic sampler
        m = re.search(r"(\d+)\s*m\b", body)
        if not m:
            sys.exit(f"no colony range for {genus}")
        species = {}
        for row in body.split("|-")[1:]:
            cells = [c.strip()[1:].strip() for c in row.strip().splitlines() if c.strip().startswith("|")]
            if len(cells) < 2:
                continue
            epithet = re.sub(r"\[\[|\]\]|''", "", cells[0]).strip()
            value = re.sub(r"[^0-9]", "", cells[1])
            if not epithet or not value:
                continue
            if epithet == genus or epithet.endswith(" " + genus):
                name = epithet
            elif genus in EPITHET_FIRST:
                name = f"{epithet} {genus}"
            else:
                name = f"{genus} {epithet}"
            species[name] = int(value)
        out[genus] = (int(m.group(1)), species)
    return out


def py_dict(src, name):
    m = re.search(rf"^{name}\s*:[^=]*=\s*(\{{.*?^\}})", src, re.S | re.M)
    if not m:
        sys.exit(f"{name} not found in exobiology_data.py")
    return ast.literal_eval(m.group(1))


def explorerlite_name(genus, species):
    """ExplorerLite keys "<genus> <epithet>"; the game writes some genera epithet first."""
    if genus in EPITHET_FIRST and species.startswith(genus + " "):
        return species[len(genus) + 1:] + " " + genus
    return species


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    root, commit = sys.argv[1], sys.argv[2]
    wiki = wiki_tables(wikitext(sys.argv[3] if len(sys.argv) > 3 else None))
    if set(wiki) != set(CODEX):
        sys.exit(f"wiki genera differ: {sorted(set(wiki) ^ set(CODEX))}")
    rules = json.load(open(os.path.join(root, "data", "species_rulesets.json")))
    src = open(os.path.join(root, "explorer", "valuation", "exobiology_data.py")).read()
    el_values = {
        explorerlite_name(g, s): v for g, by in py_dict(src, "SPECIES_VALUE").items() for s, v in by.items()
    }
    el_ranges = py_dict(src, "GENUS_MIN_DISTANCE_M")
    el_rules = {s: r for by in rules.values() for s, r in by.items()}
    genera, uncertain = [], []
    for genus in sorted(wiki):
        colony, values = wiki[genus]
        if el_ranges.get(genus) != colony:
            uncertain.append(f"{genus} colony range: wiki {colony} m, ExplorerLite {el_ranges.get(genus)} m")
        species = []
        for name, value in sorted(values.items()):
            if name not in el_values:
                sys.exit(f"ExplorerLite has no {name}")
            if el_values[name] != value:
                uncertain.append(f"{name}: wiki {value:,}, ExplorerLite {el_values[name]:,}")
            s = {"name": name, "value": value}
            if genus in rules:
                if name not in el_rules:
                    sys.exit(f"no conditions for {name}")
                s["rules"] = el_rules[name]
            species.append(s)
        if genus in rules and set(rules[genus]) != {s["name"] for s in species}:
            sys.exit(f"{genus}: species lists differ: {sorted(set(rules[genus]) ^ {s['name'] for s in species})}")
        genera.append({"name": genus, "codex": CODEX[genus], "colony_range_m": colony, "species": species})
    out = {
        "source": f"values and colony ranges: Elite Dangerous wiki, \"{WIKI_PAGE}\"; conditions: "
        f"dwomble/EDMC-ExplorerLite (MIT) at {commit}, data/species_rulesets.json",
        "units": "gravity in g, temperature in K, pressure in atmospheres; value in credits before the first-logged bonus",
        "uncertain": uncertain,
        "genera": genera,
    }
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "data", "catalog", "exobiology.json")
    with open(path, "w") as f:
        json.dump(out, f, indent=1, ensure_ascii=False)
        f.write("\n")
    print(f"wrote {len(genera)} genera, {sum(len(g['species']) for g in genera)} species; uncertain: {uncertain}")


if __name__ == "__main__":
    main()
