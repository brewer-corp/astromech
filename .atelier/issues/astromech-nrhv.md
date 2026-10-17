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
updated_at: "2026-10-17T21:31:45.613589882+00:00"
---

## Goal

When the startup catch-up follows an older journal (a sync tool touched it just before we started), the game's current journal is still replayed up to what was there before we started, not applied as live from its first line.

## Done when

A pipeline test that starts with an older journal newest by mtime and then sees the game append to its later-named current journal sends only the appended line (EDDN, callouts), not the history written before startup.

## Context

Found in review of astromech-0d7l (finding R3); predates 0d7l. catch_up opens the newest-by-mtime journal. When the game next writes its current journal, follow_newest sees it as not opened and later by name, so open(n, replay=false) applies all of it as live. The reviewer's probe queued 3 EDDN messages where 1 was expected. Same safety property as 0d7l: replayed lines must never move the route, write the clipboard, speak or be sent.

## Notes

- 2026-10-17 [note] Replaying the real journals (astromech-yin0) with the files' modification times scrambled (as a copy without preserved times, a restore or a sync tool leaves them) shows the other half of this: the catch-up replays whichever journal is newest by mtime, so until the game next writes, the core's state is that old journal's: wrong ship, ship id, system, station, credits and materials (all five differed from a full replay). If the game isn't running, that lasts the whole session. A fix should start from the game's journal, but note journal names carry local time (Journal.2026-10-16T172959 holds 21:29:59Z lines), so ordering by name alone breaks at a DST fall-back (astromech-782v).
