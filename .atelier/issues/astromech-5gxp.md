---
created_at: "2026-10-17T20:17:47.040811672+00:00"
id: "astromech-5gxp"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-22T21:17:26.115383636+00:00"
status: "done"
title: "A new-scheme journal the game starts after an old-scheme one opens as a replay"
updated_at: "2026-10-22T21:17:26.115383636+00:00"
---

## Goal

follow_newest decides whether a journal is the game's new one by its parsed name stamp, the same way as astromech_journal::latest_journal, not by comparing raw names with later().

## Done when

A pipeline test that follows an old-scheme journal (Journal.YYMMDDHHMMSS.01.log) and then sees the game start a Journal.YYYY-MM-DDTHHMMSS.01.log opens the new one live from its Fileheader, so its opening lines are sent.

## Context

Found in review of astromech-0d7l (finding F2); predates 0d7l. later() compares raw names, and by bytes "Journal.22..." sorts after "Journal.2026-...". So for a returning player whose newest file is old-scheme, or after a backup touched one, the game's new journal opens as a replay, and the lines already in it at open (LoadGame, Location) are not live.

## Close Reason

Done and independently reviewed on feature/astromech-g3qt (approval astromech-yewr).
