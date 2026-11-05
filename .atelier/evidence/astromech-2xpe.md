---
created_at: "2026-11-05T16:01:40.595231630+00:00"
id: "astromech-2xpe"
evidence_type: "validation"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:01:40.595155428+00:00"
path: "docs/design/validation/flow-4-services-after-set-destination.png"
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
title: "Claim 2: every lookup result offers Add stop and Set destination, and no action replaces the followed route without saying so and offering Resume. Observed (fake core, ?core=ready&route=exact, exact route Sol to Maia at 4 of 10): on all ten Places finders the number of 'Add <system> as a stop' and 'Set <system> as the destination' icon buttons equals the number of result rows (services 10/10/10, sell 8, buy 7, sell-cargo 8, outfitting 8, shipyard 8, material-traders 10, raw-sites 5, faction-states 2, mining-hotspots 10), as do Powerplay's panel (5) and Colonisation (22); the Find or plot popover's system row carries Plot, Add stop and Set destination too. Add stop on Markus Extraction Exchange (Alpha Centauri): strip becomes 'Next | Alpha Centauri | 4 of 11 | Markus Extraction Exchange | To | Maia | 8 stops left' and the Route page lists it as stop 4 of 11, labelled with the station and on the clipboard; Add stop on Deciat from the popover likewise ('Deciat is the next stop. Deciat is on your clipboard.'). Set destination on the same row: strip 'Next | Alpha Centauri | 1 of 1 | … | 1 stop left', line under the results 'Following. Alpha Centauri is on your clipboard. Replaced Sol to Maia. RESUME'; RESUME restores 'Next | Arietis Sector LD-H a11-1 | 4 of 10 | To | Maia | 7 jumps left' with 'Following. Arietis Sector LD-H a11-1 is on your clipboard.'; the Route page shows 'PAUSED Sol to Maia · 7 waypoints left' with RESUME, which also restores it. Screenshots docs/design/validation/flow-1..7-*.png, find-or-plot-add-stop-1600.png. Tests: SearchKit.test.ts 'Add stop and Set destination hand the core the row's system and label', PlotBar.test.ts add/set tests, route/View.test.ts park-and-resume, and the Rust route tests in the fresh cargo capture pass. Proof limit: the real core's clipboard write and parking were exercised only through the fake core in the browser; the Rust side rests on the cargo tests."
updated_at: "2026-11-05T16:01:40.595843652+00:00"
---

Claim 2: every lookup result offers Add stop and Set destination, and no action replaces the followed route without saying so and offering Resume. Observed (fake core, ?core=ready&route=exact, exact route Sol to Maia at 4 of 10): on all ten Places finders the number of 'Add <system> as a stop' and 'Set <system> as the destination' icon buttons equals the number of result rows (services 10/10/10, sell 8, buy 7, sell-cargo 8, outfitting 8, shipyard 8, material-traders 10, raw-sites 5, faction-states 2, mining-hotspots 10), as do Powerplay's panel (5) and Colonisation (22); the Find or plot popover's system row carries Plot, Add stop and Set destination too. Add stop on Markus Extraction Exchange (Alpha Centauri): strip becomes 'Next | Alpha Centauri | 4 of 11 | Markus Extraction Exchange | To | Maia | 8 stops left' and the Route page lists it as stop 4 of 11, labelled with the station and on the clipboard; Add stop on Deciat from the popover likewise ('Deciat is the next stop. Deciat is on your clipboard.'). Set destination on the same row: strip 'Next | Alpha Centauri | 1 of 1 | … | 1 stop left', line under the results 'Following. Alpha Centauri is on your clipboard. Replaced Sol to Maia. RESUME'; RESUME restores 'Next | Arietis Sector LD-H a11-1 | 4 of 10 | To | Maia | 7 jumps left' with 'Following. Arietis Sector LD-H a11-1 is on your clipboard.'; the Route page shows 'PAUSED Sol to Maia · 7 waypoints left' with RESUME, which also restores it. Screenshots docs/design/validation/flow-1..7-*.png, find-or-plot-add-stop-1600.png. Tests: SearchKit.test.ts 'Add stop and Set destination hand the core the row's system and label', PlotBar.test.ts add/set tests, route/View.test.ts park-and-resume, and the Rust route tests in the fresh cargo capture pass. Proof limit: the real core's clipboard write and parking were exercised only through the fake core in the browser; the Rust side rests on the cargo tests.
