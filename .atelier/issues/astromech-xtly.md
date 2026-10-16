---
created_at: "2026-10-16T18:56:53.876435294+00:00"
id: "astromech-xtly"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-q318"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "System chip: announce copies and show failures; chips in prose"
updated_at: "2026-10-16T18:56:53.876435294+00:00"
---

## Goal

The shared SystemChip has a fixed aria-label, so screen readers miss "Copied", and a clipboard failure is swallowed. System names inside data prose (Materials source hints such as "Nearest encoded trader: Ray Gateway, Diaguandri") are plain text, not chips.

## Done when

The chip has a polite live status and an error state (component test); source hints carry structured system names rendered as chips.

## Context

`app/src/lib/ui/SystemChip.svelte`, Materials data. Found in review of astromech-q318 (round 1, R4, R5).
