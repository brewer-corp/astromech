#!/usr/bin/env python3
"""Build the neutron star file for the local neutron plotter from Spansh's neutron dump
(https://downloads.spansh.co.uk/systems_neutron.json.gz, community EDDN data, ~180 MB).

Usage:
  scripts/gen-neutron.py --dump systems_neutron.json.gz --out neutron.bin [--built 2026-10-22]

Keeps only systems whose main star is a neutron star (the dump also holds black holes and a few
others; the jet-cone boost comes from the arrival star). --built sets the build date in the
header (default: today, UTC); the tests pass it so the output is reproducible. A dump whose name
doesn't end in .gz is read as plain JSON (the test sample).

File format, version 2 (all integers little-endian). crates/astromech-galaxy/src/neutron.rs
reads it and must change with it; scripts/test_gen_neutron.py and the reader's tests share the
fixture in crates/astromech-galaxy/tests/fixtures/.

  header, 36 bytes
    0  8 bytes  magic b"ASTRNEUT"
    8  u32      format version (2)
   12  u32      cell edge, ly (128)
   16  u32      star count N
   20  u32      cell count C
   24  i64      build time, Unix seconds UTC
   32  u32      name group G (256): names per name index entry
  cells, C x 10 bytes, sorted by (cx, cy, cz)
       i16 cx, i16 cy, i16 cz, u32 first star
       Only non-empty cells. Cell (cx, cy, cz) holds the stars with floor(q / (edge * 32)) ==
       (cx, cy, cz) per axis, q the coordinate in 1/32 ly. Its stars are the contiguous range
       first .. next cell's first (N for the last cell).
  positions, N x 12 bytes
       i32 x, i32 y, i32 z in 1/32 ly (the game's grid), in cell order, and within a cell
       sorted by (x, y, z, name).
  name index, ceil(N / G) x u32
       byte offset into the name text of the name of star G * i.
  name text, to the end of the file
       the names in star order, UTF-8, each followed by b"\\n".

A plotter loads the header, cells and positions (about 75 MB for 5 M stars) and finds the stars
within r ly of a point by scanning the cells that overlap the sphere. Names stay on disk: the
name of star i is read by seeking to its group of G and counting newlines.
"""
import argparse, datetime, gzip, json, os, struct, sys

MAGIC = b"ASTRNEUT"
VERSION = 2
CELL_LY = 128
NAME_GROUP = 256


def read_dump(path):
    """Yields (x, y, z, name), coordinates in 1/32 ly, for each neutron star in the dump."""
    opener = gzip.open if path.endswith(".gz") else open
    seen = 0
    with opener(path, "rb") as f:
        for line in f:
            line = line.strip().rstrip(b",")
            if not line.startswith(b"{"):
                continue
            seen += 1
            if seen % 1_000_000 == 0:
                print(f"... {seen:,} systems read", file=sys.stderr, flush=True)
            o = json.loads(line)
            if o.get("mainStar") != "Neutron Star":
                continue
            c, name = o["coords"], o["name"]
            if "\n" in name:
                sys.exit(f"name with a newline: {name!r}")
            yield round(c["x"] * 32), round(c["y"] * 32), round(c["z"] * 32), name


def build(stars, built):
    """The file's bytes for stars [(x, y, z, name)] in 1/32 ly; built is Unix seconds."""
    span = CELL_LY * 32
    keyed = sorted(((x // span, y // span, z // span), x, y, z, name) for x, y, z, name in stars)
    cells, positions, names, index, text_len = [], [], [], [], 0
    for i, (cell, x, y, z, name) in enumerate(keyed):
        if not cells or cells[-1][0] != cell:
            if not all(-32768 <= c < 32768 for c in cell):
                sys.exit(f"cell {cell} doesn't fit an i16")
            cells.append((cell, i))
        positions.append(struct.pack("<iii", x, y, z))
        if i % NAME_GROUP == 0:
            index.append(text_len)
        b = name.encode("utf-8") + b"\n"
        names.append(b)
        text_len += len(b)
    out = [MAGIC, struct.pack("<IIIIqI", VERSION, CELL_LY, len(keyed), len(cells), built, NAME_GROUP)]
    out += [struct.pack("<hhhI", *cell, first) for cell, first in cells]
    out += positions
    out += [struct.pack("<I", o) for o in index]
    out += names
    return b"".join(out), len(cells)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--dump", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--built", help="build date, YYYY-MM-DD (default: today, UTC)")
    a = ap.parse_args()

    day = datetime.date.fromisoformat(a.built) if a.built else datetime.datetime.now(datetime.timezone.utc).date()
    built = int(datetime.datetime(day.year, day.month, day.day, tzinfo=datetime.timezone.utc).timestamp())
    stars = list(read_dump(a.dump))
    if not stars:
        sys.exit("no neutron stars in the dump; has its format changed?")
    data, cells = build(stars, built)
    os.makedirs(os.path.dirname(os.path.abspath(a.out)), exist_ok=True)
    with open(a.out, "wb") as f:
        f.write(data)
    print(f"{len(stars):,} neutron stars in {cells:,} cells -> {a.out} ({len(data) / 1e6:.1f} MB)")


if __name__ == "__main__":
    main()
