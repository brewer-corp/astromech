---
created_at: "2026-11-05T16:01:43.331917963+00:00"
id: "astromech-27pq"
evidence_type: "validation"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:01:43.331842192+00:00"
path: "docs/guide/places.md"
agent_identity: "validator-fable-kyqw"
target:
  kind: "issue"
  id: "astromech-kyqw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kyqw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Claim 7: each new finder has a voice query, and the guide describes what shipped (scripts/check-guide.py passes). The one new finder, Mining hotspots, has the mining.hotspots intent ('nearest {mineral} hotspot', 'where is the nearest {mineral} hotspot', 'find a {mineral} hotspot', crates/astromech-intent/src/lib.rs:295) and is listed in docs/guide/voice-commands.md and mining.md#finding-hotspots; crates/astromech-core/tests/hotspot_voice.rs (3 tests) and lookups.rs hotspot_phrases_resolve_to_the_mining_lookup pass in the fresh cargo capture; every other finder keeps its existing intent (find.service/module/ship, trade.sell/buy/sell_cargo, sites.*, faction.*). python3 scripts/check-guide.py: 'checked 22 guide pages: 0 problem(s)' (captured). Guide statements checked against the preview: Galaxy pages Places and Route; tiles and sections; finder defaults (Interstellar Factors, Painite, FSD 5A, Krait MkII, Raw, Crystalline shards, Boom, Painite any ring/any reserve); tile keeps the last answer and reopens it; Find or plot phrases 'raw trader', 'outfitting fsd', 'shipyard python', 'sell painite', 'sel painite', 'crystalline shards', 'painite hotspots'; Ctrl+K; route strip lines and 'stops left' after Add stop on a plotted route; Add stop as next stop labelled with the station; Set destination says Replaced with Resume; Route page shows PAUSED with RESUME; Mining hotspots Around offers 'Next stop: Arietis Sector LD-H a11-1'; Trade, Materials and Exploration link into the finders. README's guide index lists Places and Routes. No mismatch found."
updated_at: "2026-11-05T16:01:43.332474692+00:00"
---

Claim 7: each new finder has a voice query, and the guide describes what shipped (scripts/check-guide.py passes). The one new finder, Mining hotspots, has the mining.hotspots intent ('nearest {mineral} hotspot', 'where is the nearest {mineral} hotspot', 'find a {mineral} hotspot', crates/astromech-intent/src/lib.rs:295) and is listed in docs/guide/voice-commands.md and mining.md#finding-hotspots; crates/astromech-core/tests/hotspot_voice.rs (3 tests) and lookups.rs hotspot_phrases_resolve_to_the_mining_lookup pass in the fresh cargo capture; every other finder keeps its existing intent (find.service/module/ship, trade.sell/buy/sell_cargo, sites.*, faction.*). python3 scripts/check-guide.py: 'checked 22 guide pages: 0 problem(s)' (captured). Guide statements checked against the preview: Galaxy pages Places and Route; tiles and sections; finder defaults (Interstellar Factors, Painite, FSD 5A, Krait MkII, Raw, Crystalline shards, Boom, Painite any ring/any reserve); tile keeps the last answer and reopens it; Find or plot phrases 'raw trader', 'outfitting fsd', 'shipyard python', 'sell painite', 'sel painite', 'crystalline shards', 'painite hotspots'; Ctrl+K; route strip lines and 'stops left' after Add stop on a plotted route; Add stop as next stop labelled with the station; Set destination says Replaced with Resume; Route page shows PAUSED with RESUME; Mining hotspots Around offers 'Next stop: Arietis Sector LD-H a11-1'; Trade, Materials and Exploration link into the finders. README's guide index lists Places and Routes. No mismatch found.
