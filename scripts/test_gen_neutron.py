#!/usr/bin/env python3
"""Tests for scripts/gen-neutron.py: the sample dump builds exactly the fixture the Rust reader's
tests read (crates/astromech-galaxy/tests/neutron.rs). Runs on any OS:
python3 scripts/test_gen_neutron.py

If the format changes on purpose, rebuild the fixture with
scripts/gen-neutron.py --dump crates/astromech-galaxy/tests/fixtures/neutron-sample.json \\
  --out crates/astromech-galaxy/tests/fixtures/neutron-sample.bin --built 2026-10-22
and update the reader to match."""

import struct
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FIXTURES = ROOT / "crates/astromech-galaxy/tests/fixtures"


class GenNeutron(unittest.TestCase):
    def build(self, out):
        subprocess.run([sys.executable, str(ROOT / "scripts/gen-neutron.py"), "--dump",
                        str(FIXTURES / "neutron-sample.json"), "--out", str(out), "--built", "2026-10-22"],
                       check=True, capture_output=True)
        return out.read_bytes()

    def test_sample_builds_the_fixture(self):
        with tempfile.TemporaryDirectory() as d:
            data = self.build(Path(d) / "neutron.bin")
        self.assertEqual(data, (FIXTURES / "neutron-sample.bin").read_bytes(),
                         "gen-neutron.py output differs from the committed fixture")

    def test_keeps_only_neutron_main_stars(self):
        with tempfile.TemporaryDirectory() as d:
            data = self.build(Path(d) / "neutron.bin")
        magic, version, cell, stars, cells, built = struct.unpack_from("<8sIIIIq", data)
        self.assertEqual((magic, version, cell, stars), (b"ASTRNEUT", 1, 128, 35))
        for dropped in (b"Pyroomoa AA-A h0", b"Schao Pruae YE-A g0", b"HD 117024", b"HR 6164"):
            self.assertNotIn(dropped, data)
        self.assertIn(b"Jackson's Lighthouse\n", data)


if __name__ == "__main__":
    unittest.main()
