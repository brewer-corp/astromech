---
created_at: "2026-10-17T02:44:29.456970978+00:00"
id: "astromech-6g4x"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Tighten powerplay_arrival sentence budget and off-by-default test"
updated_at: "2026-10-17T02:44:29.456970978+00:00"
---

## Goal

Tidy up two leftovers from the review of astromech-gtza.

## Done when

- `sentences()` in crates/astromech-announce/src/powerplay_arrival.rs can never produce a sentence of more than 8 words. Today a long sentence with no comma passes through unsplit. Either split it further, or make the copy test cover every `arrival_line` shape so an over-budget line would fail it.
- `is_off_by_default` in tests/powerplay_arrival.rs no longer has `.all()` assertions that pass on empty output.
