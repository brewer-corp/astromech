---
created_at: "2026-11-05T16:01:42.726889565+00:00"
id: "astromech-cls0"
evidence_type: "validation"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:01:42.726809736+00:00"
path: "docs/design/validation/find-or-plot-system-sol-1600.png"
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
title: "Claim 6: the header has one Find or plot box, also reached with Ctrl+K, that plots or adds a system and opens a finder from a phrase. Observed: one header box, placeholder 'Find or plot', 320 px wide (.find; the input inside it 276 px beside the hint), 'Ctrl+K' at its right end; Ctrl+K pressed on the Materials page moves focus to the combobox; typing 'sol' lists 'Sol · System · PLOT' first (with Add stop and Set destination icon buttons) then 'Frontline Solutions · Services', materials and a page, and the clear icon replaces 'Ctrl+K'; Enter on 'maia' opens the plot panel 'Plot to Maia' (SOL TO Maia, Neutron/Exact, efficiency, the neutron-file note); Add stop on Deciat from the popover makes it the next stop. Phrases typed then Enter: 'raw trader' -> #/places/material-traders/raw, 'sell painite' and 'sel painite' -> #/places/sell/painite, 'painite hotspots' -> #/places/mining-hotspots/painite, 'outfitting fsd' -> #/places/outfitting/Frame%20Shift%20Drive, 'shipyard python' -> #/places/shipyard/Python, 'crystalline shards' -> #/places/raw-sites/crystalline_shards, 'interstellar factors' -> #/places/services/interstellar_factors, 'boom' -> #/places/faction-states/proto (the Boom HGE pool), 'powerplay' -> #/powerplay. The popover fits at 1100 px (one 44 px tab row below the header) and at 1600 px (one header row). Screenshots docs/design/validation/find-or-plot-*.png. Tests: PlotBar.test.ts (9), find.test.ts (9) pass. Proof limits: Spansh autocomplete and real plotting are stubbed in the preview (PREVIEW_SYSTEMS, recorded plots); the popover row is 35 px where DESIGN.md says 34."
updated_at: "2026-11-05T16:01:42.727449099+00:00"
---

Claim 6: the header has one Find or plot box, also reached with Ctrl+K, that plots or adds a system and opens a finder from a phrase. Observed: one header box, placeholder 'Find or plot', 320 px wide (.find; the input inside it 276 px beside the hint), 'Ctrl+K' at its right end; Ctrl+K pressed on the Materials page moves focus to the combobox; typing 'sol' lists 'Sol · System · PLOT' first (with Add stop and Set destination icon buttons) then 'Frontline Solutions · Services', materials and a page, and the clear icon replaces 'Ctrl+K'; Enter on 'maia' opens the plot panel 'Plot to Maia' (SOL TO Maia, Neutron/Exact, efficiency, the neutron-file note); Add stop on Deciat from the popover makes it the next stop. Phrases typed then Enter: 'raw trader' -> #/places/material-traders/raw, 'sell painite' and 'sel painite' -> #/places/sell/painite, 'painite hotspots' -> #/places/mining-hotspots/painite, 'outfitting fsd' -> #/places/outfitting/Frame%20Shift%20Drive, 'shipyard python' -> #/places/shipyard/Python, 'crystalline shards' -> #/places/raw-sites/crystalline_shards, 'interstellar factors' -> #/places/services/interstellar_factors, 'boom' -> #/places/faction-states/proto (the Boom HGE pool), 'powerplay' -> #/powerplay. The popover fits at 1100 px (one 44 px tab row below the header) and at 1600 px (one header row). Screenshots docs/design/validation/find-or-plot-*.png. Tests: PlotBar.test.ts (9), find.test.ts (9) pass. Proof limits: Spansh autocomplete and real plotting are stubbed in the preview (PREVIEW_SYSTEMS, recorded plots); the popover row is 35 px where DESIGN.md says 34.
