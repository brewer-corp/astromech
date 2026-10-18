---
created_at: "2026-10-18T00:16:54.087396505+00:00"
id: "astromech-8997"
issue_type: "task"
labels:
- "alpha"
- "voice"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Voice: 'Pace.' executes panel.close through a phonetic-key collision with 'back'"
updated_at: "2026-10-18T00:20:46.314950329+00:00"
---

## Goal

Stop "Pace." from executing panel.close ("back"). "pace" and "back" share the crude phonetic key "pk", so word_sim gives 0.85, which executes with a clear margin; "pace" is also a plausible mishearing of "paste", so a commander saying "paste" in the galaxy map could close the map instead.

## Done when

"Pace." asks (or is rejected) instead of executing panel.close; "back" and "go back" still execute; a unit test pins it; the voice eval keeps 0 wrong executes.

## Context

Found by the independent reviewer of astromech-8fey (pre-existing on main, not caused by that change). Single-word phrases ("back", "paste", "fuel", "hull") get no margin help from other words, so one phonetic-key collision is enough to execute.
