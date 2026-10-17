---
created_at: "2026-10-17T05:19:07.451460138+00:00"
id: "astromech-627q"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-627q
priority: "P3"
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
closed_at: "2026-10-17T05:29:38.867796128+00:00"
status: "done"
title: "Session summary when astromech starts mid-session"
updated_at: "2026-10-17T05:29:38.867796128+00:00"
---

## Goal

The session summary arms only on a LoadGame seen live, so if astromech starts after the game is already logged in, that session's real logout says nothing (review finding astromech-bnic on astromech-oo8y). Arm from game state when the catch-up ends inside a logged-in session, keeping the fresh-launch case (astromech-d2e4) silent.

## Done when

Replay: the app starts mid-session (LoadGame only in the catch-up), a live Shutdown speaks the summary once; a fresh launch's menu still says nothing.

## Close Reason

The summary arms from the logged-in game state when the catch-up ends mid-session; fresh launch stays silent; tests added
