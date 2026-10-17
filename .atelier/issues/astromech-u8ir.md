---
created_at: "2026-10-16T20:30:43.805407809+00:00"
id: "astromech-u8ir"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-u8ir
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-cmhv"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:25:55.212585835+00:00"
status: "done"
title: "Callout loose ends: repeat prospects, threshold storage on a fresh machine, help-line copy"
updated_at: "2026-10-17T19:25:55.212585835+00:00"
---

## Goal

Three small gaps from the review of the callout slices are closed:

- **Prospector repeats (R2):** a second prospector limpet on the same rock repeats the line; prospecting the same rock again is said once.
- **Threshold storage (R3):** `prospector_thresholds` creates the data folder before opening the database (today Settings shows "opening the database" on a fresh machine), and a session whose store will not open logs that it is using default thresholds instead of falling back silently.
- **Help-line copy (R9):** the help lines (health.rs "I can't switch …", policy `UnknownKey`, session "Voice isn't loaded.") are reworded to the copy guide (V1-DESIGN §6.5), or the guide states the exemption for help lines explicitly, including contractions and "I".

## Done when

A replay test shows a repeated prospect of the same rock gives one line and a different rock still gives its own; a test opens `prospector_thresholds` on a missing data folder and gets the defaults; the help lines either pass the copy test (and are added to it) or §6.5 names the exemption.

## Context

Review findings R2, R3 and R9 on feature/astromech-cmhv (prospector astromech-omi1, copy guide astromech-wq3f). How to tell "the same rock" needs a check: `ProspectedAsteroid` has no rock id, so the material list plus a short time window is the likely key.

## Close Reason

Done-when met: Repeat prospects said once, thresholds open on a fresh machine, help-line copy done in PR 120; PR 123; see evidence
