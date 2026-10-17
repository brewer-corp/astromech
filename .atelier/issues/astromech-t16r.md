---
created_at: "2026-10-17T21:09:20.967130842+00:00"
id: "astromech-t16r"
issue_type: "task"
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
title: "Game's journal stays quiet when it isn't the latest by name and we only see it late (app restart, sleep)"
updated_at: "2026-10-17T21:09:20.967130842+00:00"
---

## Goal

astromech-36g1 makes the game's journal live when it is the folder's latest by name, stamped within 120 s of local time when we open it, or a later part of a session already judged the game's. Two cases remain silent until a new session starts: (1) the app starts or restarts while the game is running and the folder holds a journal with a later stamp (a clock that went back, or a foreign one); the game's journal is then not latest, not in game_sessions and older than 120 s, and the startup replay opens whichever is newest by mtime. (2) The game's journal is first seen more than 120 s after its start (sleep, resume, a stalled scan) and isn't the latest.

## Done when

A pipeline test starts the pipeline with a foreign later-stamped journal in the folder and a game journal started minutes ago that the game keeps appending to, and its new lines are live, while a sync tool restoring or copying an older journal and writing it late stays a replay (the astromech-0d7l and 36g1 tests keep passing).

## Context

Review findings 1 and 2 on the astromech-36g1 PR. A signal is needed for 'the game is appending to this file now' that a sync tool writing late doesn't give (line timestamps are not enough: the 0d7l tests write fresh ones). Ideas: Status.json or another file the game writes, process or handle checks, or the journal being open for write by the game's process. Daylight saving ends 1 Nov 2026, during the alpha.
