---
created_at: "2026-11-10T05:55:46.190820706+00:00"
id: "astromech-crxa"
issue_type: "validation"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Glance test: an independent read of every page at 2560 x 1440"
updated_at: "2026-11-10T05:55:46.190820706+00:00"
---

## Goal

Mission astromech-xajc's glance-test criterion: on every page a reader can point to the main value, the thing they can change and the thing that needs them, without reading. A fresh agent that built none of the pages answers those three questions per page from screenshots alone.

## Done when

- [ ] Screenshots of every page at 2560 x 1440 from the browser preview are given to an agent with no other context; its answers are recorded as an evidence receipt on astromech-xajc with --criterion glance-test.
- [ ] Every page it could not answer for is fixed or filed.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Runs after the last screen increment merges.
