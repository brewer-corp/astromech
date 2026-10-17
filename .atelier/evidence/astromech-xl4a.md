---
created_at: "2026-10-17T04:57:44.527839386+00:00"
id: "astromech-xl4a"
evidence_type: "test"
commit: "4a39a8441ff21ab251c4fea9055e8c0a9dd567a6"
result: "pass"
captured_at: "2026-10-17T04:57:44.527713541+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-wu30"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-wu30"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 4a39a84: cargo test -p astromech-exploration passes (12 tests): tests/values.rs matches EDSM's published estimates for 20 valuable bodies (mapped value, within 1 cr) and 14 system totals (scan value, within 2 cr) covering every star and planet value class, plus ED Scout's first-discovered-and-mapped worked example; tests/system.rs replays a synthetic system and shows all 5 bodies the honk counted with values, worth-mapping and signals. App: vitest src/modules/exploration passes; UI check scripts/ui against pnpm dev, screenshot /tmp/p4ls/exploration-system.png. Mutation check: altering the ammonia-world and neutron-star k values fails the EDSM tests."
updated_at: "2026-10-17T04:57:44.528457749+00:00"
---

At 4a39a84: cargo test -p astromech-exploration passes (12 tests): tests/values.rs matches EDSM's published estimates for 20 valuable bodies (mapped value, within 1 cr) and 14 system totals (scan value, within 2 cr) covering every star and planet value class, plus ED Scout's first-discovered-and-mapped worked example; tests/system.rs replays a synthetic system and shows all 5 bodies the honk counted with values, worth-mapping and signals. App: vitest src/modules/exploration passes; UI check scripts/ui against pnpm dev, screenshot /tmp/p4ls/exploration-system.png. Mutation check: altering the ammonia-world and neutron-star k values fails the EDSM tests.
