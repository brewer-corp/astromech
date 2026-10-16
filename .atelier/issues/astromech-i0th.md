---
created_at: "2026-10-15T20:36:13.662496483+00:00"
id: "astromech-i0th"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-sor5"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Render the British-voice A/B set"
updated_at: "2026-10-16T04:33:15.221232690+00:00"
---

## Goal

We ship `kokoro-en-v0_19` with `speaker_id = 0`, which is American. Render the same callout lines with `bf_emma`, `bf_isabella`, `bm_george` and `bm_lewis`, Cockpit on, so Sumeet can pick the flattest.

## Done when

The A/B set is rendered from the real TTS path and attached as evidence, with each file named by speaker, and v0.19's speaker order is confirmed so each name maps to the right ID.

## Context

The speaker walk was planned in V1-DESIGN §6.4 and never done.
