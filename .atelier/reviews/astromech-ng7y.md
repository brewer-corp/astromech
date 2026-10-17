---
schema: atelier.review
schema_version: 1
id: astromech-ng7y
title: 'Approved at cd662e2. Done-when met: one faction.<id> command per astromech_galaxy::QUERIES entry (test enumerates QUERIES). Dispatches to Galaxy::nearby, the same function as the app''s galaxy_nearby. Filters come from search.filters.faction.<id>, the key Nearby.svelte''s SearchKit uses, and the faction_voice test fails if the key is wrong (updated_at assertion). The answer gives system, distance, jumps and data age. The Spansh call is off the speech thread (spawned; only ''Looking that up.'' is said inline). Copy test covers every query, the long names and the no-answer lines. Docs updated. Checked: cargo test -p astromech-intent -p astromech-tools -p astromech-core passes; check-deps OK; merges cleanly with main (32 commits behind); conflicts with 8joi in assistant.rs (both helpers before fn dispatch), merges cleanly with 3ez9. CI on PR #93 green. Findings: 1 no-action, 2 defer.'
target: astromech-6sf7
identity: review-agent (opus, independent)
commit: cd662e2540a539a7030be7854e9b5e5ba1ea4ab7
created_at: 2026-10-17T08:17:36.723999111Z
event:
  event: approval
---

Approved at cd662e2. Done-when met: one faction.<id> command per astromech_galaxy::QUERIES entry (test enumerates QUERIES). Dispatches to Galaxy::nearby, the same function as the app's galaxy_nearby. Filters come from search.filters.faction.<id>, the key Nearby.svelte's SearchKit uses, and the faction_voice test fails if the key is wrong (updated_at assertion). The answer gives system, distance, jumps and data age. The Spansh call is off the speech thread (spawned; only 'Looking that up.' is said inline). Copy test covers every query, the long names and the no-answer lines. Docs updated. Checked: cargo test -p astromech-intent -p astromech-tools -p astromech-core passes; check-deps OK; merges cleanly with main (32 commits behind); conflicts with 8joi in assistant.rs (both helpers before fn dispatch), merges cleanly with 3ez9. CI on PR #93 green. Findings: 1 no-action, 2 defer.
