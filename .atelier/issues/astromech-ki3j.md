---
created_at: "2026-10-22T20:57:43.083817402+00:00"
id: "astromech-ki3j"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-ki3j
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "At startup, follow the game's current journal, not the one a sync tool touched last"
updated_at: "2026-11-04T20:50:32.031024722+00:00"
---

## Goal

When a backup or sync tool touches an older journal, the core still shows the commander's current ship, system and credits from startup, not the touched journal's, until the game next writes.

## Done when

A pipeline test that starts with an older journal newest by modification time and the game's current journal later by name shows the current journal's state (ship, system, credits) right after the startup catch-up, and the pipeline does not switch back to the touched journal afterwards.

## Context

The other half of astromech-nrhv (see its 2026-10-17 note and the implementer's 2026-10-22 note): the replay-versus-live half is fixed on feature/astromech-g3qt; the pipeline still picks the most recently modified journal to follow, and switching to the latest-named one at startup alone is not enough because it switches straight back.

## Notes

- 2026-11-04 [decision] Order journals by their Fileheader's UTC timestamp. While running, switch to the newest-by-mtime journal only once it has a Fileheader and it did not begin before the followed one (ties switch, so a conflict copy is still followed both ways). At startup, follow the newest by mtime unless the latest by name began after it (only those two headers are read).
  Reason: Fileheader time is UTC, so a DST fall-back (astromech-782v) cannot reorder it, and touching or rewriting an old journal does not change it; name order breaks at fall-back and mtime is what sync tools change. Keeping mtime as the change detector and adding one comparison is the smallest rule that fixes both the startup state and the switch back. Waiting for a Fileheader keeps a tool's just-created empty file from being followed before we can tell when it began. Trade-off: a foreign journal whose header is later than the game's (two game instances writing one synced folder) would keep the session; multi-commander folders are out of scope (astromech-yc7h).
