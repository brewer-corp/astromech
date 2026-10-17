---
schema: atelier.review
schema_version: 1
id: astromech-x78w
title: 'R8 [info] Checked and fine: scripts/gen-exobiology.py reproduces data/catalog/exobiology.json byte-for-byte equivalent (sorted JSON identical) from ExplorerLite 6a5d65a, EDR 198ebf5 and the wiki page fetched today. All Odyssey species symbols and Odyssey genus symbols match the journal symbols (Observatory Botanist.cs genus list, including the Clepeus spelling; species symbols cross-checked). Colony ranges match the wiki and community values for all 15 Odyssey genera (Aleoida 150, Bacterium 500, Cactoida 300, Clypeus 150, Concha 150, Electricae 1000, Fonticulua 500, Frutexa 150, Fumerola 100, Fungoida 300, Osseus 800, Recepta 150, Stratum 500, Tubus 800, Tussock 200). The other uncertain entries resolve sensibly (Bacterium Nebulus/Scopulum: the wiki''s post-Update-15 values over EDR''s older ones; Tussock Ventusa: EDR typo). NOTICE.md carries the ExplorerLite and ED Assistant MIT texts and the EDR facts-only statement; apart from R1 it is accurate. Crate boundaries pass scripts/check-deps.py; astromech-exobiology depends only on journal and status, exploration on exobiology. cargo test -p astromech-exobiology -p astromech-exploration: all pass.'
target: astromech-8e8c
identity: 'review-agent (opus, independent, part A: data and prediction)'
commit: e5f94c22467a3514e2f7f88d5e0f7a58780e5631
created_at: 2026-10-17T05:57:53.786456796Z
event:
  event: finding
  recommendation: no_action
---

R8 [info] Checked and fine: scripts/gen-exobiology.py reproduces data/catalog/exobiology.json byte-for-byte equivalent (sorted JSON identical) from ExplorerLite 6a5d65a, EDR 198ebf5 and the wiki page fetched today. All Odyssey species symbols and Odyssey genus symbols match the journal symbols (Observatory Botanist.cs genus list, including the Clepeus spelling; species symbols cross-checked). Colony ranges match the wiki and community values for all 15 Odyssey genera (Aleoida 150, Bacterium 500, Cactoida 300, Clypeus 150, Concha 150, Electricae 1000, Fonticulua 500, Frutexa 150, Fumerola 100, Fungoida 300, Osseus 800, Recepta 150, Stratum 500, Tubus 800, Tussock 200). The other uncertain entries resolve sensibly (Bacterium Nebulus/Scopulum: the wiki's post-Update-15 values over EDR's older ones; Tussock Ventusa: EDR typo). NOTICE.md carries the ExplorerLite and ED Assistant MIT texts and the EDR facts-only statement; apart from R1 it is accurate. Crate boundaries pass scripts/check-deps.py; astromech-exobiology depends only on journal and status, exploration on exobiology. cargo test -p astromech-exobiology -p astromech-exploration: all pass.
