---
schema: atelier.review
schema_version: 1
id: astromech-ot5l
title: 'Approved at 0815c73. Read the diff and the issue''s done-when. StateHub now folds every journal line into a MiningRun under its own mutex (no lock-order issue with the state lock or the session lock in status_json), and the pipeline''s startup replay of the current journal rebuilds it. Core::status_json reports it as mining (null with no session). MiningRun gains last and per_hour(), with the Docked/LoadGame reset moved first so a dock never updates last. The Logbook > Mining page polls the status every 3 s and shows total, t/h and tons by ore, following the existing pattern of a fixture sample outside Tauri. ARCHITECTURE.md and V1-DESIGN.md are updated. Ran cargo test -p astromech-gamestate --test mining, -p astromech-core --test mining_status (hub JSON equals app/src/lib/fixtures/mining.json, so the window sample cannot drift from the core shape) and vitest src/modules/mining: all pass. Did not see the screenshot; full gates taken from the author''s report, not rerun. R1 (non-mining limpets start a run) and R2 (last-line rate untested) are deferred, not blocking.'
target: astromech-p9r4
identity: review-agent (opus, independent)
commit: 0815c73f5f7e42021269493f59bb4d5720705ef3
created_at: 2026-10-17T04:55:16.316969490Z
event:
  event: approval
---

Approved at 0815c73. Read the diff and the issue's done-when. StateHub now folds every journal line into a MiningRun under its own mutex (no lock-order issue with the state lock or the session lock in status_json), and the pipeline's startup replay of the current journal rebuilds it. Core::status_json reports it as mining (null with no session). MiningRun gains last and per_hour(), with the Docked/LoadGame reset moved first so a dock never updates last. The Logbook > Mining page polls the status every 3 s and shows total, t/h and tons by ore, following the existing pattern of a fixture sample outside Tauri. ARCHITECTURE.md and V1-DESIGN.md are updated. Ran cargo test -p astromech-gamestate --test mining, -p astromech-core --test mining_status (hub JSON equals app/src/lib/fixtures/mining.json, so the window sample cannot drift from the core shape) and vitest src/modules/mining: all pass. Did not see the screenshot; full gates taken from the author's report, not rerun. R1 (non-mining limpets start a run) and R2 (last-line rate untested) are deferred, not blocking.
