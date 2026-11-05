---
created_at: "2026-11-05T16:01:41.085310274+00:00"
id: "astromech-fol6"
evidence_type: "validation"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:01:41.085234142+00:00"
path: "docs/design/validation/places-home-1600.png"
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
title: "Claim 3: Galaxy opens on Places with a home grid of finder tiles, one page and URL per finder covering Services, Outfitting, Shipyard, material traders, Sell, Buy, Sell my cargo, raw sites, faction states and Mining hotspots; old URLs and palette entries still open them. Observed: Galaxy tab opens #/places with page tabs PLACES and ROUTE only; tiles Sell, Buy, Sell my cargo (Trade), Services, Outfitting, Shipyard (Ship), Material traders, Raw sites, Faction states (Engineering), Mining hotspots (Mining); no tile carries a figure until a finder has answered, and after opening Services the tile reads 'Interstellar Factors · 0 jumps' linking to #/places/services/interstellar_factors (no request on the home page). Each finder is its own page with a Places crumb, title and main input, at #/places/<finder>/<input>; defaults match the guide (Interstellar Factors, Painite, FSD 5A, Krait MkII, Raw, Crystalline shards, Boom, Painite). Old URLs: #/services -> #/places/services/interstellar_factors, #/services/black_market -> …/services/black_market, #/outfitting -> …/outfitting/Frame%20Shift%20Drive?class=5&rating=A, #/outfitting/Pulse%20Laser?class=2&rating=E&mount=Fixed and the older #/outfitting/Pulse%20Laser/2E/Fixed both -> …/outfitting/Pulse%20Laser?class=2&rating=E&mount=Fixed, #/shipyard/Python -> …/shipyard/Python, #/shipyard -> …/shipyard/Krait%20MkII; #/trade, #/materials, #/materials/iron and #/activities still open their pages; a mangled #/places/sell/%E0%A4%A opens Sell on Painite; the hash is rewritten in place. Note: ?class=2&rating=F&mount=Fixed drops mount because 2F Fixed is not a combination the pulse laser comes in (same settle() rule as main). Find or plot entries 'interstellar factors', 'outfitting fsd', 'shipyard python', 'raw trader' open the finders filled in. Screenshots docs/design/validation/places-home-{1600,1100}.png, places-home-last-answer-1600.png, finder-*-1600.png. Tests: places.test.ts (8), palette.test.ts (5), App.test.ts old-link and tile tests pass."
updated_at: "2026-11-05T16:01:41.085917036+00:00"
---

Claim 3: Galaxy opens on Places with a home grid of finder tiles, one page and URL per finder covering Services, Outfitting, Shipyard, material traders, Sell, Buy, Sell my cargo, raw sites, faction states and Mining hotspots; old URLs and palette entries still open them. Observed: Galaxy tab opens #/places with page tabs PLACES and ROUTE only; tiles Sell, Buy, Sell my cargo (Trade), Services, Outfitting, Shipyard (Ship), Material traders, Raw sites, Faction states (Engineering), Mining hotspots (Mining); no tile carries a figure until a finder has answered, and after opening Services the tile reads 'Interstellar Factors · 0 jumps' linking to #/places/services/interstellar_factors (no request on the home page). Each finder is its own page with a Places crumb, title and main input, at #/places/<finder>/<input>; defaults match the guide (Interstellar Factors, Painite, FSD 5A, Krait MkII, Raw, Crystalline shards, Boom, Painite). Old URLs: #/services -> #/places/services/interstellar_factors, #/services/black_market -> …/services/black_market, #/outfitting -> …/outfitting/Frame%20Shift%20Drive?class=5&rating=A, #/outfitting/Pulse%20Laser?class=2&rating=E&mount=Fixed and the older #/outfitting/Pulse%20Laser/2E/Fixed both -> …/outfitting/Pulse%20Laser?class=2&rating=E&mount=Fixed, #/shipyard/Python -> …/shipyard/Python, #/shipyard -> …/shipyard/Krait%20MkII; #/trade, #/materials, #/materials/iron and #/activities still open their pages; a mangled #/places/sell/%E0%A4%A opens Sell on Painite; the hash is rewritten in place. Note: ?class=2&rating=F&mount=Fixed drops mount because 2F Fixed is not a combination the pulse laser comes in (same settle() rule as main). Find or plot entries 'interstellar factors', 'outfitting fsd', 'shipyard python', 'raw trader' open the finders filled in. Screenshots docs/design/validation/places-home-{1600,1100}.png, places-home-last-answer-1600.png, finder-*-1600.png. Tests: places.test.ts (8), palette.test.ts (5), App.test.ts old-link and tile tests pass.
