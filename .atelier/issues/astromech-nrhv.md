---
created_at: "2026-10-17T20:12:38.721718941+00:00"
id: "astromech-nrhv"
issue_type: "bug"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "The game's journal is fully live after a startup catch-up on a touched older journal"
updated_at: "2026-10-17T20:12:38.721718941+00:00"
---

## Goal

When the startup catch-up follows an older journal (a sync tool touched it just before we started), the game's current journal is still replayed up to what was there before we started, not applied as live from its first line.

## Done when

A pipeline test that starts with an older journal newest by mtime and then sees the game append to its later-named current journal sends only the appended line (EDDN, callouts), not the history written before startup.

## Context

Found in review of astromech-0d7l (finding R3); predates 0d7l. catch_up opens the newest-by-mtime journal. When the game next writes its current journal, follow_newest sees it as not opened and later by name, so open(n, replay=false) applies all of it as live. The reviewer's probe queued 3 EDDN messages where 1 was expected. Same safety property as 0d7l: replayed lines must never move the route, write the clipboard, speak or be sent.
