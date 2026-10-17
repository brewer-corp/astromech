---
created_at: "2026-10-17T21:31:54.140957030+00:00"
id: "astromech-782v"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-782v
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-yin0"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T22:31:27.200876165+00:00"
status: "done"
title: "A journal started in the repeated hour after a DST fall-back is never live"
updated_at: "2026-10-17T22:31:27.200876165+00:00"
---

## Goal

The pipeline treats the journal the game is writing as live (callouts, route following, EDDN, EDSM) even when its name sorts before the previous journal's because the clocks went back.

## Done when

A pipeline test with `Journal.2026-11-01T013000.01.log` followed by a newer-written `Journal.2026-11-01T011000.01.log` (the game restarted at 01:10 after the clocks went back) applies the second file's appended lines as live, and still never treats a touched older journal as live (astromech-0d7l, astromech-nrhv).

## Context

Found replaying real journals (astromech-yin0): journal names carry the PC's local time, while the lines inside are stamped in UTC. `crates/astromech-core/src/pipeline.rs` decides live-ness by name (`latest_journal`, `later`): only the latest journal by name is the game's. When a journal is started in the hour after a fall-back (US 1 Nov, EU 25 Oct 2026) and the previous one was started in the same wall-clock hour before it, the new one sorts earlier, so the whole session is replayed silently. The window is narrow (two game starts within the repeated hour). The `Fileheader` `timestamp` is UTC and could order files instead.

## Close Reason

Already covered by astromech-t16r: a journal that sorts before the previous one but whose header agrees with its name (fall-back ambiguity accepted either way) is gated by line freshness and live. Test a_journal_started_in_the_repeated_hour_after_the_clocks_go_back_is_live (America/New_York, exact shape; fails when only the first ambiguous reading is accepted) and a startup test. A touched first journal (latest by name) staying live is filed as astromech-r3s7. Review recorded.
