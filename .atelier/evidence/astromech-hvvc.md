---
created_at: "2026-10-16T20:29:55.765290285+00:00"
id: "astromech-hvvc"
evidence_type: "validation"
commit: "bd05a76833eb46050ddd0cd11d67866e2a2d15f2"
result: "pass"
captured_at: "2026-10-16T20:29:55.765231455+00:00"
target:
  kind: "issue"
  id: "astromech-i4gx"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-i4gx"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review fixes at 95ff7b7. R1: failed_migration_rolls_back_whole (store) applies a migration whose last statement fails; the file stays at user_version 1 with neither the ALTER nor the CREATE applied, and the next open with a working migration reaches v2. With the old migrate the same test fails (checked by reverting it temporarily). R3: continuation_part_keeps_the_commander imports a two-part session (part 2 has no Commander/LoadGame) and gets both jumps for F0000003, also after recomputing commanders from the stored lines; it fails with continuation handling disabled (checked). R6: running is set before History::open (inspection; the core history tests pass). R4: doc fixed. fmt, clippy -D warnings, cargo test --workspace, check-deps (18 crates) and the shell --locked check pass after merging main (39cc476)."
updated_at: "2026-10-16T20:29:55.796839173+00:00"
---

Review fixes at 95ff7b7. R1: failed_migration_rolls_back_whole (store) applies a migration whose last statement fails; the file stays at user_version 1 with neither the ALTER nor the CREATE applied, and the next open with a working migration reaches v2. With the old migrate the same test fails (checked by reverting it temporarily). R3: continuation_part_keeps_the_commander imports a two-part session (part 2 has no Commander/LoadGame) and gets both jumps for F0000003, also after recomputing commanders from the stored lines; it fails with continuation handling disabled (checked). R6: running is set before History::open (inspection; the core history tests pass). R4: doc fixed. fmt, clippy -D warnings, cargo test --workspace, check-deps (18 crates) and the shell --locked check pass after merging main (39cc476).
