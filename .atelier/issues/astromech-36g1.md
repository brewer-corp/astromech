---
created_at: "2026-10-17T20:12:38.600227989+00:00"
id: "astromech-36g1"
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
status: "todo"
title: "A new game session whose journal name sorts earlier is never live"
updated_at: "2026-10-17T20:12:38.600227989+00:00"
---

## Goal

A journal the game starts while we run is live even when its name sorts before the previous session's (the clock went back for daylight saving, or the PC moved to a timezone further west), assuming the game names journals in local time (to check).

## Done when

A pipeline test with a new session journal named earlier than the one followed, created after we started, gets its lines delivered as live (EDDN, callouts), while an older journal a sync tool touches or restores stays a replay (astromech-0d7l tests still pass).

## Context

Found in review of astromech-0d7l (finding R2). Since 0d7l, Tailer::open treats only the folder's latest journal by name stamp as live, so such a session is entirely non-live until a later-named file appears. Before 0d7l, only the lines already in the file at open were replayed. One option is "later by name, or created after we started". Rare: needs a session start within the repeated DST hour, or travel west.
