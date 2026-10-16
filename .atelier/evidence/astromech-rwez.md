---
created_at: "2026-10-16T19:45:09.529752046+00:00"
id: "astromech-rwez"
evidence_type: "validation"
commit: "95d54565100f3d4a6cc438b8c5d93ba7ca70427e"
result: "pass"
captured_at: "2026-10-16T19:45:09.529718484+00:00"
target:
  kind: "issue"
  id: "astromech-aonb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-aonb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Import timing on a large synthetic journal set (release build of crates/astromech-history/examples/import-history.rs at c11b2a2, Linux 7.0, Ryzen 5 PRO 6650H, CARGO_BUILD_JOBS=3). Set: 80 synthetic Journal.*.log files generated under /tmp (heavy sessions ~5 MB each: Loadout with 40 engineered modules, FSDJump with 7 factions, 2-9 detailed Scans per jump, dockings with trade/bounty), 400.3 MB, 888,839 lines. Clean import into an empty history.sqlite: 7.45 s and 8.06 s on two runs (~50-54 MB/s, ~110-120k lines/s), 888,839 events, 0 malformed. Ledgers: visits 74,690, earnings 25,252, ships 12, scans 341,423. Database 638.0 MB after WAL checkpoint (1.59x the journals): events 506.8 MB, events_event_ts 37.2 MB, events_ts 28.2 MB, scans 24.4 MB, unique (file_id, offset) 13.5 MB, the rest under 11 MB each. Re-running the import with nothing new: 0 files read, 0.00 s. Synthetic set and database deleted afterwards. The run on real logs is astromech-tqdh."
updated_at: "2026-10-16T19:45:09.529956698+00:00"
---

Import timing on a large synthetic journal set (release build of crates/astromech-history/examples/import-history.rs at c11b2a2, Linux 7.0, Ryzen 5 PRO 6650H, CARGO_BUILD_JOBS=3). Set: 80 synthetic Journal.*.log files generated under /tmp (heavy sessions ~5 MB each: Loadout with 40 engineered modules, FSDJump with 7 factions, 2-9 detailed Scans per jump, dockings with trade/bounty), 400.3 MB, 888,839 lines. Clean import into an empty history.sqlite: 7.45 s and 8.06 s on two runs (~50-54 MB/s, ~110-120k lines/s), 888,839 events, 0 malformed. Ledgers: visits 74,690, earnings 25,252, ships 12, scans 341,423. Database 638.0 MB after WAL checkpoint (1.59x the journals): events 506.8 MB, events_event_ts 37.2 MB, events_ts 28.2 MB, scans 24.4 MB, unique (file_id, offset) 13.5 MB, the rest under 11 MB each. Re-running the import with nothing new: 0 files read, 0.00 s. Synthetic set and database deleted afterwards. The run on real logs is astromech-tqdh.
