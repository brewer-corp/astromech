---
created_at: "2026-10-15T20:21:44.894393414+00:00"
id: "astromech-niib"
issue_type: "task"
labels:
- "needs-cmdr"
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-pakt"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "British default voice"
updated_at: "2026-10-15T20:21:44.894393414+00:00"
---

## Goal

We ship `kokoro-en-v0_19` with `speaker_id = 0`, which is American. A/B `bf_emma`, `bf_isabella`, `bm_george` and `bm_lewis` with Cockpit on, and make the flattest the default.

## Done when

The sample config and default use the chosen speaker, and Sumeet picked it from the rendered A/B set (note on this issue).

## Context

Check v0.19's speaker order before hard-coding an ID. The speaker walk was planned in V1-DESIGN §6.4 and never done.
