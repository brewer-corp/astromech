---
schema: atelier.review
schema_version: 1
id: astromech-2v9y
title: Trade merge 954961f approved. cargo_stolen is now derived in GameState::apply_cargo as the sum of max(0, stolen) over main's CargoSnap.inventory, equal to the old per-line sum, set on every Cargo.json apply (the only writer; journal Cargo events never fed state). Mission cargo is counted as before (Stolen is 0 there in practice). Main's sellable()/trade My cargo path is untouched, and the merged tree has no lost or weakened tests (the journal parse test now asserts inventory[0].stolen). fmt, clippy -D warnings, workspace tests, check-deps and the tauri metadata check all pass.
target: astromech-cmhv
identity: review-agent (cmhv merge review 2, sonnet)
commit: 03ce6b9583668d078ef82ff29c28d78a0d16b066
created_at: 2026-10-16T22:51:07.170610878Z
event:
  event: approval
---

Trade merge 954961f approved. cargo_stolen is now derived in GameState::apply_cargo as the sum of max(0, stolen) over main's CargoSnap.inventory, equal to the old per-line sum, set on every Cargo.json apply (the only writer; journal Cargo events never fed state). Mission cargo is counted as before (Stolen is 0 there in practice). Main's sellable()/trade My cargo path is untouched, and the merged tree has no lost or weakened tests (the journal parse test now asserts inventory[0].stolen). fmt, clippy -D warnings, workspace tests, check-deps and the tauri metadata check all pass.
