---
created_at: "2026-10-17T05:03:49.168907951+00:00"
id: "astromech-44lg"
evidence_type: "test"
commit: "bb5c01a034fa683f5bbac1c53f0d3763e9de513c"
result: "pass"
captured_at: "2026-10-17T05:03:49.168838561+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-tzc2"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-tzc2"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At bb5c01a: cargo test -p astromech-exploration --test unsold (3 tests): replaying scans (a repeat scan counted once), a DSS map and two organic samples gives cartographic = star + icy + mapped water world and organic = 20,010,800; SellExplorationData of one system takes off only its bodies, MultiSellExplorationData and SellOrganicData then reset the total to 0, and scanning on starts the next total; Died loses everything; 240 M cr of samples is flagged risky. cargo test -p astromech-history exploration (2 tests): History::unsold over an imported journal counts only lines since the commander's last death, less the sold system, per commander. App: vitest src/modules/exploration (2 tests) passes; UI check scripts/ui against pnpm dev, screenshot /tmp/p4ls/exploration-unsold.png (unsold tiles, risk reminder, nearest UC on the search kit)."
updated_at: "2026-10-17T05:03:49.169401002+00:00"
---

At bb5c01a: cargo test -p astromech-exploration --test unsold (3 tests): replaying scans (a repeat scan counted once), a DSS map and two organic samples gives cartographic = star + icy + mapped water world and organic = 20,010,800; SellExplorationData of one system takes off only its bodies, MultiSellExplorationData and SellOrganicData then reset the total to 0, and scanning on starts the next total; Died loses everything; 240 M cr of samples is flagged risky. cargo test -p astromech-history exploration (2 tests): History::unsold over an imported journal counts only lines since the commander's last death, less the sold system, per commander. App: vitest src/modules/exploration (2 tests) passes; UI check scripts/ui against pnpm dev, screenshot /tmp/p4ls/exploration-unsold.png (unsold tiles, risk reminder, nearest UC on the search kit).
