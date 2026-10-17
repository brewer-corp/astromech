---
created_at: "2026-10-16T19:41:46.292368742+00:00"
id: "astromech-0x68"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: wt-base-0x68
  work_branch: task/astromech-0x68
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-k3wp"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Colonisation tracker: wire into the live journal (per-commander keys, load fallback, records table)"
updated_at: "2026-10-17T17:13:45.073520717+00:00"
---

## Goal

Settle three things when the colonisation tracker (astromech-colonisation) is wired into the live journal: (R5) key projects, claims and beacons by commander (ARCHITECTURE section 18.5 cmdr_fid); (R7) decide the caller's fallback when one corrupt record makes Tracker::load fail; (R6) fold the generic records table into the events table design when astromech-aonb lands, or note it in section 18.5.

## Done when

The tracker's state is keyed per commander when fed from the live journal, the load-failure fallback is decided and tested, and the records-table question is resolved or noted in ARCHITECTURE section 18.5.

## Notes

- 2026-10-17 [note] Decision (R7): Tracker::load skips and reports an unreadable record (returned in Loaded::skipped for the caller to log) instead of failing the load. The record stays in the store untouched, the rest loads, and a journal replay refills what is missing, so no data is lost and the app keeps working. R6: the generic records table is kept in astromech.sqlite, not folded into the history events table; noted in ARCHITECTURE 18.5. R5: state keyed <commander>|<id> using the history's commander key (moved to astromech-journal::commander). Persistence is not yet driven by a live consumer: the app and voice read through colonisation::from_journal_dir, which now shows only the newest line's commander.
