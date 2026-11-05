---
created_at: "2026-11-04T23:55:31.999191885+00:00"
id: "astromech-b0tn"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-b0tn
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
status: "in_progress"
title: "Journal following: a just-left journal touched by a sync tool"
updated_at: "2026-11-05T00:16:14.116605205+00:00"
---

## Goal

A journal the game has just left can't take the session back, and retire the game's real session, when a tool touches it within two minutes of its last line.

## Done when

In follow_newest, a journal wins by being written only if its last line is stamped after the followed journal's start; a pipeline test touches the just-left journal within 120 s after a rollover and the game's new journal keeps the session and stays Game (R1, astromech-y9qx).

## Context

Deferred review finding R1 (astromech-y9qx) on task/astromech-t34c: crates/astromech-core/src/pipeline.rs:578-614, 765-770. Nothing is wrongly sent; on Windows the game's lines can go unread for 30-60 s and some live lines count as history.
