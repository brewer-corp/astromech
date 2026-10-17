---
created_at: "2026-10-17T21:09:20.967130842+00:00"
id: "astromech-t16r"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-t16r
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T21:42:19.267185203+00:00"
status: "done"
title: "Game's journal stays quiet when it isn't the latest by name and we only see it late (app restart, sleep)"
updated_at: "2026-10-17T21:42:19.267185203+00:00"
---

## Goal

astromech-36g1 makes the game's journal live when it is the folder's latest by name, stamped within 120 s of local time when we open it, or a later part of a session already judged the game's. Two cases remain silent until a new session starts: (1) the app starts or restarts while the game is running and the folder holds a journal with a later stamp (a clock that went back, or a foreign one); the game's journal is then not latest, not in game_sessions and older than 120 s, and the startup replay opens whichever is newest by mtime. (2) The game's journal is first seen more than 120 s after its start (sleep, resume, a stalled scan) and isn't the latest.

## Done when

A pipeline test starts the pipeline with a foreign later-stamped journal in the folder and a game journal started minutes ago that the game keeps appending to, and its new lines are live, while a sync tool restoring or copying an older journal and writing it late stays a replay (the astromech-0d7l and 36g1 tests keep passing).

## Context

Review findings 1 and 2 on the astromech-36g1 PR. A signal is needed for 'the game is appending to this file now' that a sync tool writing late doesn't give (line timestamps are not enough: the 0d7l tests write fresh ones). Ideas: Status.json or another file the game writes, process or handle checks, or the journal being open for write by the game's process. Daylight saving ends 1 Nov 2026, during the alpha.

## Notes

- 2026-10-17 [note] Rule: a journal that is neither the folder's latest by stamp nor a later part of a session already judged the game's is judged by its Fileheader: the header's UTC time must equal its name's local time (within 120 s, DST ambiguity accepted either way). That is what the game does (checked on 22 real journals, see astromech-36g1), and it separates the game's journal from a copy from a PC with another offset (header and name disagree). A restored copy from this machine agrees too, so its lines count as live only when stamped within 120 s of now: what a tool writes late is old. The 36g1 'stamp within 120 s of local time' window is replaced by this rule. The startup replay stays silent; the game's journal followed after startup replays its stale lines and applies fresh ones live. Not covered: a journal that is neither latest nor header-consistent but is the game's (a machine whose offset changed during a session, or a header written before the clock moved); the game idle for minutes then writing one line is fine (that line is fresh when written). The existing 0d7l fixtures need no change: their restored journals have a fixed old name stamp and a fresh header, so header and name disagree.

## Close Reason

A journal that isn't latest by stamp is the game's if its Fileheader UTC time equals its name's local time; only lines stamped within 120 s are live. Fail-before tests for both cases (app started beside a later foreign journal; journal first seen late); restored journals with old or minutes-old lines, header/name disagreement and conflict copies stay replays; 0d7l tests unchanged and 50 of 50 under load. Review recorded; last fixes not re-reviewed. Windows unverified.
