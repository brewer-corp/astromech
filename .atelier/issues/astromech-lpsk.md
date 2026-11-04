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
status: "todo"
title: "Journal following after a backwards clock correction and a restart"
updated_at: "2026-11-04T21:17:40.916015916+00:00"
---

## Goal

After this PC's clock is corrected back by more than two minutes, a restart of astromech while the game writes a new journal still follows the game's journal, not the older one whose header was stamped by the fast clock.

## Done when

A pipeline test: journal J1 headed an hour ahead, J2 (the game's) started after the clock was corrected back, astromech starting once real time has passed J1's header while J2 is written: after catch-up and the game's next line, J2 is followed and J1 isn't. A likely fix is to let the newest journal that gets fresh lines take the session regardless of header order (which would also cover a foreign journal from a clock less than 120 s ahead), as long as a touched or appended older journal is still never followed.

## Context

Review finding R4 (astromech-of26) on bug/astromech-ki3j: crates/astromech-core/src/pipeline.rs:95-106 and 669-673. Fails closed (nothing is wrongly live) but the game's journal is not read for that session.
