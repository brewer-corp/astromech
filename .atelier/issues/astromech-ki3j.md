---
created_at: "2026-10-22T20:57:43.083817402+00:00"
id: "astromech-ki3j"
issue_type: "bug"
labels:
- "size-s"
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
status: "todo"
title: "At startup, follow the game's current journal, not the one a sync tool touched last"
updated_at: "2026-10-22T20:57:48.172109815+00:00"
---

## Goal

When a backup or sync tool touches an older journal, the core still shows the commander's current ship, system and credits from startup, not the touched journal's, until the game next writes.

## Done when

A pipeline test that starts with an older journal newest by modification time and the game's current journal later by name shows the current journal's state (ship, system, credits) right after the startup catch-up, and the pipeline does not switch back to the touched journal afterwards.

## Context

The other half of astromech-nrhv (see its 2026-10-17 note and the implementer's 2026-10-22 note): the replay-versus-live half is fixed on feature/astromech-g3qt; the pipeline still picks the most recently modified journal to follow, and switching to the latest-named one at startup alone is not enough because it switches straight back.
