#!/usr/bin/env python3
"""Tests for scripts/gen-eddn-outfitting.py --check, against a made-up FDevIDs outfitting.csv
built from the committed data/catalog/eddn-outfitting.json. No network. Runs on any OS:
python3 scripts/test_gen_eddn_outfitting.py"""

import csv
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCRIPT = ROOT / "scripts/gen-eddn-outfitting.py"
COMMITTED = json.loads((ROOT / "data/catalog/eddn-outfitting.json").read_text(encoding="utf-8"))


class Check(unittest.TestCase):
    def check(self, rows):
        """Run --check on an FDevIDs checkout whose outfitting.csv is `rows` (symbol, entitlement)."""
        with tempfile.TemporaryDirectory() as d:
            with open(Path(d) / "outfitting.csv", "w", encoding="utf-8", newline="") as f:
                w = csv.writer(f)
                w.writerow(["id", "symbol", "entitlement"])
                for i, (symbol, entitlement) in enumerate(rows):
                    w.writerow([128000000 + i, symbol, entitlement])
            return subprocess.run([sys.executable, str(SCRIPT), "--check", d], capture_output=True, text=True)

    def rows(self):
        # FDevIDs writes symbols in their game case; the script lowercases them.
        return [(s.title(), "") for s in COMMITTED["station"]] + [
            (s, "ELITE_SPECIFIC_V_POWER_100000") for s in COMMITTED["commander"]
        ]

    def test_same_lists_pass(self):
        r = self.check(self.rows())
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        self.assertIn("matches", r.stdout)

    def test_new_symbol_fails_and_names_it(self):
        r = self.check(self.rows() + [("Hpt_NewThing_Fixed_Small", ""), ("Int_NewUnlock_Size1_Class1", "?")])
        self.assertEqual(r.returncode, 1, r.stdout + r.stderr)
        self.assertIn("new: hpt_newthing_fixed_small (station)", r.stdout)
        self.assertIn("new: int_newunlock_size1_class1 (commander)", r.stdout)
        self.assertIn("out of date", r.stderr)

    def test_moved_and_gone_symbols_fail(self):
        station, commander = COMMITTED["station"], COMMITTED["commander"]
        rows = [(station[0], "ELITE_SPECIFIC_V_POWER_100000")] + [(s, "") for s in station[2:]]
        rows += [(s, "removed") for s in commander]
        r = self.check(rows)
        self.assertEqual(r.returncode, 1, r.stdout + r.stderr)
        self.assertIn(f"moved: {station[0]} (station -> commander)", r.stdout)
        self.assertIn(f"gone: {station[1]} (was station)", r.stdout)

    def test_check_writes_nothing(self):
        before = (ROOT / "data/catalog/eddn-outfitting.json").read_bytes()
        self.check([("Hpt_NewThing_Fixed_Small", "")])
        self.assertEqual((ROOT / "data/catalog/eddn-outfitting.json").read_bytes(), before)


if __name__ == "__main__":
    unittest.main()
