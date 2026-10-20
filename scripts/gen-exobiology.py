#!/usr/bin/env python3
"""Generate data/catalog/exobiology.json: every exobiology genus with its journal symbol and
colony range (the least distance between samples), and its species with their base value
(Vista Genomics, before the first-logged bonus), journal symbol and, for the Odyssey genera, the
conditions they were seen growing in. Also writes the held-out test bodies
(crates/astromech-exobiology/tests/fixtures/spansh-held-out.json) and prints the prediction's
recall on them.

Usage: scripts/gen-exobiology.py <EDR biology.json> <EDR commit> <Spansh cache dir> [wiki wikitext file]

Sources (see NOTICE.md):
- Values and colony ranges: the Elite Dangerous wiki's "Exobiology Sample Values and Details"
  page (elite-dangerous.fandom.com), read through its MediaWiki API (or the saved wikitext).
  Only the numbers are used. `VALUE_FIXES` overrides a value the wiki has wrong, with the reason.
- Species symbols (`$codex_ent_stratum_07_name;`): EDR's `edr/data/biology.json` (lekeno/edr,
  Apache-2.0), facts only (symbol, name, credits). Its credits are compared with the wiki's; a
  disagreement is listed under `uncertain`. A species EDR knows and the wiki's page doesn't
  (Radicoida Unicus) is kept with EDR's value and colony range, marked uncertain.
- Horizons-era species symbols (`$codex_ent_sphereabcd_01_name;`), which EDR doesn't list:
  Canonn's codex reference (its public API, `query/codex/ref`; the answer is kept in the cache
  directory and reused), facts only (symbol and English name).
- Genus symbols: the journal's `Genus` values, as listed by Elite Observatory
  (Xjph/ObservatoryCore, MIT).
- Conditions: measured here from Spansh's bodies search (spansh.co.uk, EDDN data: bodies other
  players scanned, with the species they sampled there as codex landmarks). For each Odyssey
  species the script asks for the bodies with that species nearest four points across the
  galaxy (`landmark_subtype` filter, 100 bodies each; answers are kept in the cache directory and
  reused). A quarter of the bodies (by a hash of the id64) is held out and never used for the conditions;
  the rest give, per species and per (atmosphere type, body class) seen at least twice, the
  observed temperature, gravity and pressure range, and, from enough sightings, the nearest star
  classes and volcanism seen. No other tool's condition tables are read.
- The nearest star: the search gives only a body's immediate parent, so a moon or a body around
  a pair of stars would have none. The script asks Spansh's bodies search for the stars of every
  system with a sighting (cached in the cache directory) and reads the nearest star from the
  body's name, as the game names bodies: the longest star name the body's name starts with
  ("Sys A 4 a": star A; "Sys 4 d": the brown dwarf "Sys 4" it orbits); "Sys 4 a" in a system
  with one star (or a star named as the system) is that star's, and "Sys A 4" with no star
  "Sys A" orbits the star named as the system (the primary, where the other stars have catalogue
  names: "Eme" and "Gliese 425 B"); "Sys AB 1" orbits a pair of stars and, like the journal's
  `Parents` with no star before the barycentre, has none. A body the name gives no star keeps its
  immediate parent's, if a star ("LFT 1504 B 3" in a system of "LFT 1504" and "Gl 773 B").
"""
import hashlib, json, os, re, sys, time, urllib.parse, urllib.request

WIKI_PAGE = "Exobiology Sample Values and Details"
WIKI_API = "https://elite-dangerous.fandom.com/api.php"
SPANSH = "https://spansh.co.uk/api/bodies/search"
CANONN_CODEX = "https://us-central1-canonn-api-236217.cloudfunctions.net/query/codex/ref"
AGENT = "astromech gen-exobiology.py (+https://github.com/brewer-corp/astromech)"
# Where the species are looked for: Sol, Colonia, the galactic centre, the far side of the core.
REFS = [(0.0, 0.0, 0.0), (-9530.5, -910.28, 19808.13), (25.2, -20.9, 25899.97), (1500.0, 0.0, 40000.0)]
PER_REF = 100
# Held out of the derivation, for the recall test: a quarter of the bodies, by a hash of the id64
# (its low bits follow the system's position, so id64 % 4 would not be a fair quarter).
HELD_OUT = lambda id64: hashlib.sha256(str(id64).encode()).digest()[0] % 4 == 0
# The held-out bodies written as the test fixture.
FIXTURE_BODIES = 400
# Star classes and volcanism become conditions only from this many sightings of a group.
GATE_MIN = 10
# A group (atmosphere type, body class) becomes a condition only from this many sightings: one
# sighting is as likely a data slip as a habitat.
MIN_SEEN = 2
# The system's id64 is the low 55 bits of a body's.
SYSTEM_MASK = (1 << 55) - 1
# Systems per stars search (a system has one to a few stars; a page holds 500).
STAR_BATCH = 200

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
    # Horizons-era surface biology, sampled the same way; no conditions.
    "Amphora Plant": ["Vents"],
    "Anemone": ["Sphere"],
    "Bark Mound": ["Cone"],
    # Elite Observatory has seen brain trees under `Seed` (the Roseum Brain Tree's species symbol) too.
    "Brain Tree": ["Brancae", "Seed"],
    "Crystalline Shard": ["Ground_Struct_Ice"],
    "Sinuous Tuber": ["Tube"],
}
HORIZONS = {"Amphora Plant", "Anemone", "Bark Mound", "Brain Tree", "Crystalline Shard", "Sinuous Tuber"}

# Horizons-era genera whose species the game names "<epithet> <genus>" ("Roseum Brain Tree").
EPITHET_FIRST = {"Anemone", "Brain Tree", "Sinuous Tuber"}

# Canonn's codex names for Horizons-era species, where they differ from the wiki's beyond the plural.
CODEX_NAME_FIXES = {"Roseum Sinuous Tubers": "Roseus Sinuous Tuber"}

# The one value per Horizons-era genus in ED Assistant's seed data (what astromech used before this
# catalogue), for the `uncertain` comparison.
ED_ASSISTANT = {
    "Anemone": 1_499_900,
    "Brain Tree": 1_593_700,
    "Sinuous Tuber": 1_514_500,
    "Bark Mound": 1_471_900,
    "Amphora Plant": 1_628_800,
    "Crystalline Shard": 1_628_800,
}

# Values the wiki's page has wrong: (value, why).
VALUE_FIXES = {
    "Fonticulua Fluctus": (
        20_000_000,
        "wiki 16,777,215 (2^24-1, the figure it also gives Concha Biconcavis); EDR says 20,000,000",
    ),
}


def get_json(url, body=None, tries=3):
    data = json.dumps(body).encode() if body is not None else None
    headers = {"User-Agent": AGENT, "Content-Type": "application/json"}
    for i in range(tries):
        try:
            with urllib.request.urlopen(urllib.request.Request(url, data=data, headers=headers), timeout=120) as r:
                return json.load(r)
        except Exception as e:  # network hiccup: wait and retry
            if i + 1 == tries:
                raise
            print(f"retrying after {e}", file=sys.stderr)
            time.sleep(10)


def wikitext(path):
    if path:
        return open(path).read()
    q = urllib.parse.urlencode({"action": "parse", "page": WIKI_PAGE, "prop": "wikitext", "format": "json"})
    return get_json(f"{WIKI_API}?{q}")["parse"]["wikitext"]["*"]


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


def horizons_symbols(cache, wiki):
    """{wiki species name: journal symbol, lower case} for the Horizons-era species, from Canonn's
    codex reference (cached). The codex names a species in the plural ("Roseum Sinuous Tubers")."""
    path = os.path.join(cache, "canonn-codex-ref.json")
    if not os.path.exists(path):
        ref = get_json(CANONN_CODEX)
        with open(path, "w") as f:
            json.dump(ref, f)
    ref = json.load(open(path))
    names = {n.lower(): n for g, (_, sp) in wiki.items() if g in HORIZONS for n in sp}
    out = {}
    for e in ref.values():
        if e.get("hud_category") != "Biology" or not e.get("name", "").startswith("$Codex_Ent_"):
            continue
        name = CODEX_NAME_FIXES.get(e["english_name"], e["english_name"]).removesuffix("s")
        if name.lower() in names:
            out[names[name.lower()]] = e["name"].lower()
    missing = sorted(set(names.values()) - set(out))
    if missing:
        sys.exit(f"no codex symbol for {missing}")
    return out


# --- Spansh bodies, in the journal's terms ---

PLANET_CLASS = {
    "High metal content world": "High metal content body",
    "Rocky Ice world": "Rocky ice body",
    "Metal-rich body": "Metal rich body",
}


def atmosphere_type(spansh):
    """Spansh's "Thin Carbon dioxide-rich" as the journal's `AtmosphereType` `CarbonDioxideRich`."""
    if not spansh or spansh == "No atmosphere":
        return "None"
    a = re.sub(r"^((hot|thin|thick) )+", "", spansh, flags=re.I)
    rich = a.endswith("-rich")
    a = a.removesuffix("-rich")
    return "".join(w[:1].upper() + w[1:] for w in re.split(r"[ -]", a)) + ("Rich" if rich else "")


def star_class(spansh):
    """The nearest parent star's class as astromech_exobiology::catalog::star_class gives it for
    the journal's `StarType`."""
    s = spansh or ""
    for prefix, c in [("T Tauri", "TTS"), ("Herbig", "AeBe"), ("Neutron", "N"), ("Black Hole", "H"),
                      ("Supermassive", "H"), ("Wolf-Rayet", "W"), ("White Dwarf", "D"), ("C Star", "C"),
                      ("CN Star", "C"), ("CJ Star", "C"), ("MS-type", "S"), ("S-type", "S")]:
        if s.startswith(prefix):
            return c
    m = re.match(r"([OBAFGKMLTY]) ", s)
    return m.group(1) if m else None


def volcanism(spansh):
    """The volcanism's material, as astromech_exobiology::predict::volcanism_kind gives it for the
    journal's `Volcanism` text: "none", "water", "carbon dioxide", "silicate", ... None if unknown."""
    if spansh is None:
        return None
    return volcanism_kind(spansh)


def volcanism_kind(text):
    words = [w for w in text.lower().replace("volcanism", "").split() if w not in ("minor", "major", "geysers", "magma", "vapour", "no")]
    return " ".join(words) or "none"


def system_stars(cache, systems):
    """{system id64: [system name, {star name: class}]} for `systems`, from Spansh's bodies search
    (the stars only), kept in the cache directory and extended with the systems it lacks."""
    path = os.path.join(cache, "system-stars.json")
    known = json.load(open(path)) if os.path.exists(path) else {}
    missing = sorted(s for s in systems if str(s) not in known)
    for i in range(0, len(missing), STAR_BATCH):
        batch = missing[i:i + STAR_BATCH]
        found = {s: [None, {}] for s in batch}
        page = 0
        while True:
            q = {"filters": {"system_id64": {"value": batch}, "type": {"value": ["Star"]}},
                 "sort": [{"id64": {"direction": "asc"}}], "size": 500, "page": page}
            d = get_json(SPANSH, q)
            for r in d.get("results", []):
                f = found[r["system_id64"]]
                f[0] = r["system_name"]
                f[1][r["name"]] = star_class(r.get("subtype"))
            time.sleep(1.5)
            page += 1
            if page * 500 >= d.get("count", 0):
                break
        known.update({str(k): v for k, v in found.items()})
        with open(path, "w") as f:
            json.dump(known, f)
        print(f"stars: {min(i + STAR_BATCH, len(missing))}/{len(missing)} systems", file=sys.stderr)
    return {int(k): v for k, v in known.items()}


def star_by_name(o, systems):
    """The class of the body's nearest star, from its name and its system's stars (see the module
    doc). When the name gives none, the body's immediate parent's class, if a star."""
    return star_from_name(o, systems) or o["star"]


def star_from_name(o, systems):
    name, stars = systems.get(o["id64"] & SYSTEM_MASK, (None, {}))
    if not name or not o["name"].startswith(name + " "):
        return None
    rest = o["name"][len(name) + 1:].split()
    # The longest star name it starts with: "Sys 4 d" is a moon of the brown dwarf "Sys 4".
    for k in range(len(rest) - 1, 0, -1):
        if stars.get(f"{name} {' '.join(rest[:k])}"):
            return stars[f"{name} {' '.join(rest[:k])}"]
    first = rest[0]
    if first.isdigit():
        return stars.get(name) or (next(iter(stars.values())) if len(stars) == 1 else None)
    if first.isalpha() and first.isupper() and len(first) == 1:
        # The primary, A, may carry the system's own name where the others have catalogue
        # names ("Eme A 1 a" in a system of the stars "Eme" and "Gliese 425 B").
        return stars.get(f"{name} {first}") or (stars.get(name) if first == "A" else None)
    return None


def nearest_star(b):
    for p in b.get("parents") or []:
        if p.get("type") == "Star":
            return star_class(p.get("subtype"))
    return None


def observation(b):
    return {
        "id64": b["id64"],
        "name": b["name"],
        "planet_class": PLANET_CLASS.get(b.get("subtype"), b.get("subtype")),
        "atmosphere_type": atmosphere_type(b.get("atmosphere")),
        "volcanism": volcanism(b.get("volcanism_type")),
        "temperature_k": b.get("surface_temperature"),
        "gravity_g": b.get("gravity"),
        "pressure_atm": b.get("surface_pressure"),
        "star": nearest_star(b),
        "signals": sum(s["count"] for s in b.get("signals") or [] if s["name"] == "Biological"),
    }


def fetch(cache, species):
    bodies = {}
    for i, (x, y, z) in enumerate(REFS):
        path = os.path.join(cache, f"{species.replace(' ', '_')}-{i}.json")
        if not os.path.exists(path):
            q = {"filters": {"landmark_subtype": {"value": [species]}}, "sort": [{"distance": {"direction": "asc"}}],
                 "size": PER_REF, "page": 0, "reference_coords": {"x": x, "y": y, "z": z}}
            d = get_json(SPANSH, q)
            keep = []
            for b in d.get("results", []):
                o = observation(b)
                o["species"] = sorted({l["subtype"] for l in b.get("landmarks") or [] if l.get("subtype")})
                keep.append(o)
            json.dump(keep, open(path, "w"))
            time.sleep(1.5)
        for o in json.load(open(path)):
            # Answers cached before "Hot thin" was handled.
            o["atmosphere_type"] = re.sub(r"^(Hot|Thin|Thick)+", "", o["atmosphere_type"]) or "None"
            bodies[o["id64"]] = o
    return bodies


def span(values):
    v = [x for x in values if x is not None]
    return (round(min(v), 6), round(max(v), 6)) if v else (None, None)


def derive(name, sightings):
    """Conditions for one species: one per (atmosphere type, body class) it was seen on."""
    groups = {}
    for o in sightings:
        if o["planet_class"] and o["atmosphere_type"] != "None":
            groups.setdefault((o["atmosphere_type"], o["planet_class"]), []).append(o)
    rules = []
    for (atmosphere, body), obs in sorted(groups.items()):
        if len(obs) < MIN_SEEN:
            continue
        r = {"atmosphere": [atmosphere], "body_types": [body], "seen": len(obs)}
        for key, field in [("temp_k", "temperature_k"), ("gravity_g", "gravity_g"), ("pressure_atm", "pressure_atm")]:
            lo, hi = span(o[field] for o in obs)
            if lo is not None:
                r[f"min_{key}"], r[f"max_{key}"] = lo, hi
        stars = [o["star"] for o in obs if o["star"]]
        if len(stars) >= GATE_MIN:
            r["star_types"] = sorted(set(stars))
        volc = [o["volcanism"] for o in obs if o["volcanism"]]
        if len(volc) >= GATE_MIN:
            r["volcanism"] = sorted(set(volc))
        rules.append(r)
    return rules


# --- The prediction as the Rust code does it (for the printed recall) ---

MARGIN = 0.15


def range_fit(v, lo, hi):
    if lo is None and hi is None:
        return 1.0
    if (lo is None or v >= lo) and (hi is None or v <= hi):
        return 1.0
    edge = lo if lo is not None and v < lo else hi
    width = (hi - lo) if lo is not None and hi is not None and hi > lo else abs(edge)
    margin = width * MARGIN
    over = (lo - v) if lo is not None and v < lo else (v - hi)
    return 0.0 if margin <= 0 or over >= margin else 1.0 - over / margin


def fits(r, o):
    if o["atmosphere_type"] not in r["atmosphere"] or o["planet_class"] not in r["body_types"]:
        return 0.0
    if o["star"] and "star_types" in r and o["star"] not in r["star_types"]:
        return 0.0
    if o["volcanism"] and "volcanism" in r and o["volcanism"] not in r["volcanism"]:
        return 0.0
    f = 1.0
    for key, field in [("temp_k", "temperature_k"), ("gravity_g", "gravity_g"), ("pressure_atm", "pressure_atm")]:
        if o[field] is not None:
            f = min(f, range_fit(o[field], r.get(f"min_{key}"), r.get(f"max_{key}")))
    return f


def main():
    if len(sys.argv) < 4:
        sys.exit(__doc__)
    edr_path, edr_commit, cache = sys.argv[1:4]
    os.makedirs(cache, exist_ok=True)
    wiki = wiki_tables(wikitext(sys.argv[4] if len(sys.argv) > 4 else None))
    if set(wiki) != set(CODEX):
        sys.exit(f"wiki genera differ: {sorted(set(wiki) ^ set(CODEX))}")
    edr = json.load(open(edr_path))
    edr_species = {v["name"].strip(): (k.strip(), v["credits"]) for k, v in edr["species"].items()}
    codex_symbols = horizons_symbols(cache, wiki)

    # Sightings of every Odyssey species.
    odyssey = sorted(n for g, (_, sp) in wiki.items() if g not in HORIZONS for n in sp)
    bodies = {}
    for n in odyssey:
        for id64, o in fetch(cache, n).items():
            bodies.setdefault(id64, o)
    known = set(odyssey)
    # The nearest star from the system's stars; the search's immediate parent kept to check it.
    systems = system_stars(cache, {o["id64"] & SYSTEM_MASK for o in bodies.values()})
    agree = differ = 0
    for o in bodies.values():
        parent = o["star"]
        o["star"] = star_by_name(o, systems)
        if parent and o["star"]:
            agree += parent == o["star"]
            differ += parent != o["star"]
    train = [o for o in bodies.values() if not HELD_OUT(o["id64"])]
    held = sorted((o for o in bodies.values() if HELD_OUT(o["id64"])), key=lambda o: o["id64"])
    by_species = {}
    for o in train:
        for s in o["species"]:
            if s in known:
                by_species.setdefault(s, []).append(o)

    genera, uncertain = [], []
    for codex, wiki_name in CODEX_NAME_FIXES.items():
        uncertain.append(f"{wiki_name}: the wiki's name; the codex says {codex}")
    for genus in sorted(wiki):
        colony, values = wiki[genus]
        species = []
        for name, value in sorted(values.items()):
            if name in VALUE_FIXES:
                fixed, why = VALUE_FIXES[name]
                uncertain.append(f"{name}: {fixed:,} used; {why}")
                value = fixed
            s = {"name": name, "value": value}
            if name in edr_species:
                symbol, credits = edr_species.pop(name)
                s["symbol"] = symbol
                if credits != value:
                    uncertain.append(f"{name}: {value:,} used (wiki), EDR {credits:,}")
            elif name in codex_symbols:
                s["symbol"] = codex_symbols[name]
            if genus not in HORIZONS:
                s["rules"] = derive(name, by_species.get(name, []))
                if not s["rules"]:
                    uncertain.append(f"{name}: no sightings in the Spansh sample, so never predicted")
            species.append(s)
        if genus in ED_ASSISTANT and any(s["value"] != ED_ASSISTANT[genus] for s in species):
            uncertain.append(f"{genus}: ED Assistant {ED_ASSISTANT[genus]:,} for every species; wiki per species")
        genera.append({"name": genus, "codex": CODEX[genus], "colony_range_m": colony, "species": species})
    # Species EDR has that the wiki's page doesn't: their own genus, from EDR's symbols.
    extra = {}
    for name, (symbol, credits) in sorted(edr_species.items()):
        m = re.fullmatch(r"\$codex_ent_([a-z]+)_[a-z0-9]+_name;", symbol)
        if not m:
            sys.exit(f"EDR species {name} has an unexpected symbol {symbol}")
        extra.setdefault((name.split()[0], m.group(1)), []).append({"name": name, "value": credits, "symbol": symbol})
    for (genus, codex), species in sorted(extra.items()):
        ccr = edr["genuses"].get(f"$codex_ent_{codex}_genus_name;", {}).get("ccr")
        if ccr is None:
            sys.exit(f"EDR has no colony range for {genus}")
        uncertain.append(f"{genus}: only EDR lists it ({', '.join(s['name'] for s in species)}; colony range {ccr} m)")
        genera.append({"name": genus, "codex": [codex.capitalize()], "colony_range_m": ccr, "species": species})
    genera.sort(key=lambda g: g["name"])

    rules = {s["name"]: s.get("rules", []) for g in genera for s in g["species"]}
    out = {
        "source": f"values and colony ranges: Elite Dangerous wiki, \"{WIKI_PAGE}\" (fixes under uncertain); "
        f"species symbols: lekeno/edr (Apache-2.0, facts only) at {edr_commit}, edr/data/biology.json, "
        "and for the Horizons-era species Canonn's codex reference; "
        f"conditions: measured from Spansh's bodies search (EDDN data) on {time.strftime('%Y-%m-%d')}, "
        f"{len(train)} bodies with {sum(len(v) for v in by_species.values())} sightings "
        f"({len(held)} more held out for the recall test)",
        "units": "gravity in g, temperature in K, pressure in atmospheres; value in credits before the first-logged bonus; "
        "seen: how many bodies the condition comes from",
        "uncertain": uncertain,
        "genera": genera,
    }
    root = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
    with open(os.path.join(root, "data", "catalog", "exobiology.json"), "w") as f:
        json.dump(out, f, indent=1, ensure_ascii=False)
        f.write("\n")

    # The recall on the held-out bodies, and the fixture for the Rust test.
    hits = total = cands = 0
    star_hits = star_total = star_cands = star_bodies = 0
    for o in held:
        found = {n for n, rs in rules.items() if any(fits(r, o) > 0 for r in rs)}
        cands += len(found)
        star_bodies += o["star"] is not None
        star_cands += len(found) if o["star"] else 0
        for s in o["species"]:
            if s in known:
                total += 1
                hits += s in found
                if o["star"]:
                    star_total += 1
                    star_hits += s in found
    step = max(1, len(held) // FIXTURE_BODIES)
    fixture = [{k: v for k, v in o.items()} for o in held[::step][:FIXTURE_BODIES]]
    path = os.path.join(root, "crates", "astromech-exobiology", "tests", "fixtures", "spansh-held-out.json")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump({"source": "Spansh bodies search, held out of data/catalog/exobiology.json's conditions", "bodies": fixture}, f, indent=0)
        f.write("\n")
    print(f"{len(genera)} genera; conditions from {len(train)} bodies; held out {len(held)}: recall "
          f"{hits}/{total} = {hits / max(total, 1):.3f}, {cands / max(len(held), 1):.1f} candidates per body; "
          f"fixture {len(fixture)} bodies")
    print(f"nearest star known for {star_bodies}/{len(held)} held-out bodies: recall "
          f"{star_hits}/{star_total} = {star_hits / max(star_total, 1):.3f}, "
          f"{star_cands / max(star_bodies, 1):.1f} candidates per body; "
          f"the name and the immediate parent agree on {agree} of {agree + differ} bodies")
    single = sum(r["seen"] < MIN_SEEN for rs in rules.values() for r in rs)
    print(f"{sum(len(rs) for rs in rules.values())} conditions, {single} from fewer than {MIN_SEEN} sightings")
    print("uncertain:", *uncertain, sep="\n  ")


if __name__ == "__main__":
    main()
