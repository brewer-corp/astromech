---
created_at: "2026-10-17T20:12:38.600227989+00:00"
id: "astromech-36g1"
issue_type: "bug"
labels:
- "alpha"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "The game's journal is never live when a journal name's stamp disagrees with real time"
updated_at: "2026-10-17T20:20:34.481956351+00:00"
---

## Goal

The game's own journal is live even when the folder holds a journal whose name stamp sorts later than it, or when the game's new session has an earlier stamp than the previous one. Examples: the clock went back for daylight saving; the player travelled west; another PC in a later timezone, or one with a skewed clock, syncs the same folder. This assumes the game names journals in local time, which still needs checking.

## Done when

- Pipeline tests cover both shapes and deliver the game's lines as live (EDDN, callouts):
  - a new session journal named earlier than the one followed, created after we started;
  - a foreign journal stamped later than the game's current one.
- A sync tool touching or restoring an older journal still stays a replay, and the astromech-0d7l tests still pass.
- When the followed journal isn't the folder's latest, the pipeline logs it once.

## Context

Found in review of astromech-0d7l (findings R2 and F1). Since 0d7l, Tailer::open treats only the folder's latest journal by name stamp (astromech_journal::latest_journal) as live. In these cases EDDN, EDSM and callouts go quiet with no log line until real time passes the stamp. One option is "latest by stamp, or created after we started".
