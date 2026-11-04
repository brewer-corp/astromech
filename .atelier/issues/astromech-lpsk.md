---
created_at: "2026-11-04T21:17:40.916015916+00:00"
id: "astromech-lpsk"
issue_type: "bug"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "review"
    id: "astromech-of26"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Journal following after a backwards clock correction and a restart"
updated_at: "2026-11-04T23:31:27.293271434+00:00"
---

## Goal

After this PC's clock is corrected back by more than two minutes, a restart of astromech while the game writes a new journal still follows the game's journal, not the older one whose header was stamped by the fast clock.

## Done when

A pipeline test: journal J1 headed an hour ahead, J2 (the game's) started after the clock was corrected back, astromech starting once real time has passed J1's header while J2 is written: after catch-up and the game's next line, J2 is followed and J1 isn't. A likely fix is to let the newest journal that gets fresh lines take the session regardless of header order (which would also cover a foreign journal from a clock less than 120 s ahead), as long as a touched or appended older journal is still never followed.

## Context

Review finding R4 (astromech-of26) on bug/astromech-ki3j: crates/astromech-core/src/pipeline.rs:95-106 and 669-673. Fails closed (nothing is wrongly live) but the game's journal is not read for that session.

## Notes

- 2026-11-04 [decision] Follow the newest journal when the game is writing it now (being_written: the game's by classify's rules without taking the session, i.e. same session stamp, header agrees with name, or latest by name of a session not left; and its last complete line stamped within FRESH_LINE_SECS), whatever the header order. Otherwise switch by header order only if the followed journal isn't being written. catch_up keeps the newest by mtime when it is being written. A 'not being written at length L' memo avoids re-reading the tail of a refused journal until it grows.
  Reason: Freshness of the last line is what a touched or appended older journal can't have unless something writes lines stamped now; the game's-journal condition excludes conflict copies and copies whose header disagrees with their name, the same boundary classify already uses for live Gated lines. The second condition stops a touched journal that began later (the fast-clock J1) from taking the session from the one the game writes. Trade-offs: a journal of this machine that a tool appends lines stamped now to is followed (and its fresh lines are live, as Gated already allowed); while the game is idle for over 120 s, a touched journal that began later can still take the session until the game's next line; a journal the game starts whose header disagrees with its name and isn't latest by name waits up to 120 s for the old one to go stale.
