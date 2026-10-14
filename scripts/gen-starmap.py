#!/usr/bin/env python3
"""Build the star map preload (SQLite) and the ship engineers' locations from Spansh's systems
dump (https://downloads.spansh.co.uk/systems.json.gz, community EDDN data, ~6.4 GB).

Usage:
  scripts/gen-starmap.py --dump systems.json.gz --out starmap.db [--radius 500]
                         [--engineers-csv FDevIDs/engineers.csv --engineers-json data/catalog/engineers.json]
  scripts/gen-starmap.py --empty --out starmap.db      # schema only (CI builds of the shell)

Keeps every system within --radius ly of Sol, plus the engineer systems wherever they are.
Coordinates are stored as integers in 1/32 ly (the game's grid). The schema must match
crates/astromech-galaxy/src/store.rs.
"""
import argparse, csv, datetime, gzip, json, math, os, sqlite3, sys

SCHEMA = """
CREATE TABLE IF NOT EXISTS systems(id64 INTEGER PRIMARY KEY, name TEXT NOT NULL,
  x INTEGER NOT NULL, y INTEGER NOT NULL, z INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS coverage(id INTEGER PRIMARY KEY, x REAL NOT NULL, y REAL NOT NULL,
  z REAL NOT NULL, radius REAL NOT NULL, source TEXT NOT NULL, fetched TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS meta(key TEXT PRIMARY KEY, value TEXT NOT NULL);
"""

SHIP_ENGINEER_IDS = range(300000, 400000)  # 3000xx: ship engineers; 4000xx: on-foot (Odyssey)


def open_db(path):
    if os.path.exists(path):
        os.remove(path)
    os.makedirs(os.path.dirname(os.path.abspath(path)), exist_ok=True)
    db = sqlite3.connect(path)
    db.executescript(SCHEMA)
    return db


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--dump")
    ap.add_argument("--out", required=True)
    ap.add_argument("--radius", type=float, default=500.0)
    ap.add_argument("--empty", action="store_true")
    ap.add_argument("--engineers-csv")
    ap.add_argument("--engineers-json")
    a = ap.parse_args()

    db = open_db(a.out)
    now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    if a.empty:
        db.execute("INSERT INTO meta VALUES ('source', 'empty (CI placeholder)')")
        db.commit()
        print(f"empty star map -> {a.out}")
        return
    if not a.dump:
        sys.exit("--dump is required unless --empty")

    engineers = {}
    if a.engineers_csv:
        for r in csv.DictReader(open(a.engineers_csv, encoding="utf-8")):
            if int(r["id"]) in SHIP_ENGINEER_IDS:
                engineers[int(r["system_address"])] = engineers.get(int(r["system_address"]), []) + [r]
    eng_sys = {}

    r2 = a.radius * a.radius
    rows, seen = [], 0
    key = b'"coords":{"x":'
    with gzip.open(a.dump, "rb") as f:
        for line in f:
            i = line.find(key)
            if i < 0:
                continue
            seen += 1
            j = line.find(b"}", i)
            p = line[i + len(key) : j].split(b",")
            x = float(p[0]); y = float(p[1].split(b":")[1]); z = float(p[2].split(b":")[1])
            near = x * x + y * y + z * z <= r2
            if not near and not engineers:
                continue
            o = None
            if not near:
                k = line.find(b'"id64":')
                id64 = int(line[k + 7 : line.find(b",", k)])
                if id64 not in engineers:
                    continue
            o = json.loads(line.strip().rstrip(b","))
            row = (o["id64"], o["name"], round(x * 32), round(y * 32), round(z * 32))
            rows.append(row)
            if o["id64"] in engineers:
                eng_sys[o["id64"]] = (o["name"], x, y, z)
            if len(rows) >= 200_000:
                db.executemany("INSERT OR REPLACE INTO systems VALUES (?,?,?,?,?)", rows)
                rows = []
            if seen % 20_000_000 == 0:
                print(f"... {seen:,} systems read", file=sys.stderr, flush=True)
    db.executemany("INSERT OR REPLACE INTO systems VALUES (?,?,?,?,?)", rows)
    db.execute("INSERT INTO coverage(x, y, z, radius, source, fetched) VALUES (0, 0, 0, ?, 'preload', ?)", (a.radius, now))
    db.execute("INSERT INTO meta VALUES ('source', ?)", (f"Spansh systems dump, {a.radius:g} ly of Sol, built {now}",))
    db.commit()
    n = db.execute("SELECT count(*) FROM systems").fetchone()[0]
    db.execute("VACUUM")
    db.close()
    print(f"{n:,} systems of {seen:,} -> {a.out} ({os.path.getsize(a.out) / 1e6:.1f} MB)")

    if a.engineers_json:
        out = []
        for id64, rs in engineers.items():
            for r in rs:
                name, x, y, z = eng_sys.get(id64, (None, None, None, None))
                if name is None:
                    print(f"warning: no system {id64} for {r['name']}", file=sys.stderr)
                    continue
                out.append({
                    "id": int(r["id"]), "name": r["name"], "system": name, "id64": id64,
                    "market_id": int(r["market_id"]) if r["market_id"] else None,
                    "coords": [x, y, z], "colonia": math.dist((x, y, z), (-9530.5, -910.28125, 19808.125)) < 1000,
                })
        out.sort(key=lambda e: e["id"])
        with open(a.engineers_json, "w", encoding="utf-8") as f:
            json.dump({"source": "EDCD/FDevIDs engineers.csv + Spansh systems dump (scripts/gen-starmap.py)", "engineers": out}, f, indent=1, ensure_ascii=False)
            f.write("\n")
        print(f"{len(out)} ship engineers -> {a.engineers_json}")


if __name__ == "__main__":
    main()
