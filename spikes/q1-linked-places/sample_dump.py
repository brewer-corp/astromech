#!/usr/bin/env python3
"""Sample a downloaded prefix of a Spansh dump (dump-sample.sh) and estimate a slim local
file: systems, stations with services and trader kind, rings with hotspots, as SQLite.

    python3 -I sample_dump.py dumps/galaxy_stations.json.gz.head <full .gz size in bytes> out.sqlite

The prefix is a cut-off gzip stream of one JSON system per line; the cut line is dropped.
Sizes are extrapolated by compressed bytes read, so they inherit the dump's ordering bias.
"""
import gzip, json, os, sqlite3, sys, zlib

ORES = ["Alexandrite", "Benitoite", "Bromellite", "Grandidierite", "Low Temperature Diamonds", "Monazite",
        "Musgravite", "Painite", "Platinum", "Rhodplumsite", "Serendibite", "Tritium", "Void Opal", "Osmium",
        "Palladium"]
# The game picks the trader kind from the station's economy; Spansh does the same.
TRADER = {"Extraction": "Raw", "Refinery": "Raw", "Industrial": "Manufactured", "High Tech": "Encoded",
          "Military": "Encoded"}
CARRIERS = {"Drake-Class Carrier"}


def lines(path):
    """Complete lines of a truncated gzip file, and the compressed bytes they came from."""
    d = zlib.decompressobj(16 + zlib.MAX_WBITS)
    buf = b""
    with open(path, "rb") as f:
        while True:
            chunk = f.read(1 << 20)
            if not chunk:
                break
            buf += d.decompress(chunk)
            *done, buf = buf.split(b"\n")
            for l in done:
                yield l, f.tell()


def main(path, full_size, out):
    if os.path.exists(out):
        os.remove(out)
    db = sqlite3.connect(out)
    db.executescript("""
    CREATE TABLE systems (id64 INTEGER PRIMARY KEY, name TEXT, x REAL, y REAL, z REAL, population INTEGER,
                          allegiance TEXT, economy TEXT);
    CREATE VIRTUAL TABLE systems_pos USING rtree(id64, x0, x1, y0, y1, z0, z1);
    CREATE TABLE stations (market_id INTEGER PRIMARY KEY, system_id64 INTEGER, name TEXT, type TEXT,
                           arrival_ls REAL, pad INTEGER, services INTEGER, trader TEXT, broker TEXT);
    CREATE TABLE rings (id64 INTEGER PRIMARY KEY, system_id64 INTEGER, name TEXT, type TEXT, reserve TEXT,
                        scanned_at TEXT);
    CREATE TABLE ring_hotspots (ring_id64 INTEGER, ore INTEGER, count INTEGER, PRIMARY KEY (ring_id64, ore))
        WITHOUT ROWID;
    """)
    services = {}
    n_sys = n_st = n_carrier = n_rings = n_hot_rings = n_hot = n_trader = n_pop = 0
    used = 0
    for raw, at in lines(path):
        raw = raw.strip().rstrip(b",")
        if not raw.startswith(b"{"):
            continue
        try:
            s = json.loads(raw)
        except ValueError:
            continue
        used = at
        n_sys += 1
        c = s["coords"]
        pop = s.get("population") or 0
        n_pop += pop > 0
        db.execute("INSERT OR IGNORE INTO systems VALUES (?,?,?,?,?,?,?,?)",
                   (s["id64"], s["name"], c["x"], c["y"], c["z"], pop, s.get("allegiance"),
                    s.get("primaryEconomy")))
        db.execute("INSERT OR IGNORE INTO systems_pos VALUES (?,?,?,?,?,?,?)",
                   (s["id64"], c["x"], c["x"], c["y"], c["y"], c["z"], c["z"]))
        for st in s.get("stations") or []:
            if st.get("type") in CARRIERS:
                n_carrier += 1
                continue
            n_st += 1
            mask = 0
            for name in st.get("services") or []:
                mask |= 1 << services.setdefault(name, len(services))
            sv = st.get("services") or []
            trader = TRADER.get(st.get("primaryEconomy") or "") if "Material Trader" in sv else None
            n_trader += trader is not None
            pads = st.get("landingPads") or {}
            pad = 3 if pads.get("large") else 2 if pads.get("medium") else 1 if pads.get("small") else 0
            db.execute("INSERT OR IGNORE INTO stations VALUES (?,?,?,?,?,?,?,?,?)",
                       (st.get("id"), s["id64"], st.get("name"), st.get("type"), st.get("distanceToArrival"),
                        pad, mask, trader, "?" if "Technology Broker" in sv else None))
        for b in s.get("bodies") or []:
            for r in b.get("rings") or []:
                n_rings += 1
                sig = (r.get("signals") or {}).get("signals") or {}
                hot = {k: v for k, v in sig.items() if k in ORES}
                if not hot:
                    continue
                n_hot_rings += 1
                db.execute("INSERT OR IGNORE INTO rings VALUES (?,?,?,?,?,?)",
                           (r.get("id64"), s["id64"], r["name"], r.get("type"), b.get("reserveLevel"),
                            (r.get("signals") or {}).get("updateTime")))
                for k, v in hot.items():
                    n_hot += 1
                    db.execute("INSERT OR IGNORE INTO ring_hotspots VALUES (?,?,?)", (r.get("id64"), ORES.index(k), v))
    db.execute("CREATE INDEX stations_system ON stations(system_id64)")
    db.execute("CREATE INDEX rings_system ON rings(system_id64)")
    db.commit()
    db.execute("VACUUM")
    db.close()
    scale = full_size / used
    size = os.path.getsize(out)
    with open(out, "rb") as f:
        gz = len(gzip.compress(f.read(), 6))
    print(f"sample: {used / 1e6:.1f} MB compressed of {full_size / 1e9:.2f} GB (x{scale:.0f})")
    print(f"systems {n_sys} (populated {n_pop}), stations {n_st} (+{n_carrier} carriers skipped), "
          f"material traders {n_trader}, rings {n_rings}, rings with hotspots {n_hot_rings}, hotspot rows {n_hot}")
    print(f"extrapolated: systems {n_sys * scale:,.0f}, stations {n_st * scale:,.0f}, "
          f"traders {n_trader * scale:,.0f}, hotspot rings {n_hot_rings * scale:,.0f}")
    print(f"sample SQLite {size / 1e6:.2f} MB ({gz / 1e6:.2f} MB gzip) -> extrapolated "
          f"{size * scale / 1e6:,.0f} MB ({gz * scale / 1e6:,.0f} MB gzip)")
    print(f"distinct services {len(services)}")


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]), sys.argv[3])
