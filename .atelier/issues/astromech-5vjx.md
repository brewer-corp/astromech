---
created_at: "2026-10-17T16:54:24.179378366+00:00"
id: "astromech-5vjx"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-18T13:42:56.572754914+00:00"
status: "done"
title: "Release notes and the in-game test script for the alpha"
updated_at: "2026-10-18T13:42:56.572754914+00:00"
---

## Goal

Write the v1.0.0-alpha.1 notes: what's new since v0.1.5, and what has not been checked in the game, grouped into about four play sessions (trading, outfitting and engineering; mining and callouts; exploration, Powerplay and colonisation; VR and voice) that cover every open needs-human validation issue.

## Done when

The notes are in the release, and every open needs-human validation issue under both missions appears in one session.

## Close Reason

Release notes docs/releases/v1.0.0-alpha.1.md (what's new, 4 in-game test sessions covering the 37 needs-human issues, known limits), check-release-notes.py, release.yml builds the body from the notes and fails on pending markers; no pending markers remain.
