---
schema: atelier.review
schema_version: 1
id: astromech-otpp
title: finding on astromech-51ue @0b6806b9
target: astromech-51ue
identity: reviewer-sonnet-f7i0
commit: 0b6806b99ee217651e9f848144dc2f608e9b5db2
created_at: 2026-11-05T09:13:10.071556231Z
event:
  event: finding
  recommendation: defer
---

low: astromech-f7i0 app/src/lib/hotspots.ts:20 ORES (symbol, name) is a hand copy of astromech_journal::ore::VALUE_ORDER and the voice slot (mining::spoken_ores) with no drift test (hotspots.test.ts only checks one symbol is present). An ore added or renamed in Rust is missing from the finder, tile and palette, or a TS typo reaches Want.ore and is named by catalog::name_of's fallback. Fix: a vitest that reads the Rust list (a generated JSON, like the other gen-*.py catalogues) or a Rust test that parses ORES.
