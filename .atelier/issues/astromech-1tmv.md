---
created_at: "2026-10-15T20:21:44.863665649+00:00"
id: "astromech-1tmv"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-1v3m"
  - kind: "issue"
    id: "astromech-2uji"
  - kind: "issue"
    id: "astromech-6y47"
  - kind: "issue"
    id: "astromech-ag7v"
  - kind: "issue"
    id: "astromech-dp15"
  - kind: "issue"
    id: "astromech-ggej"
  - kind: "issue"
    id: "astromech-gm88"
  - kind: "issue"
    id: "astromech-mzlr"
  - kind: "issue"
    id: "astromech-uh7d"
  - kind: "issue"
    id: "astromech-y3bw"
  - kind: "issue"
    id: "astromech-zmy2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Event → reaction engine"
updated_at: "2026-10-15T20:21:44.863665649+00:00"
---

## Goal

Rules subscribe to journal and Status events and output speech, a toast in the app or overlay, or a Windows notification. Priority queue, per-rule toggle and cooldown, quiet during docking and combat, silent while replaying the journal.

## Done when

Replay tests show: a rule fires once per event within its cooldown; a disabled rule never fires; nothing is spoken during journal catch-up at startup.

## Context

Today only route progress speaks. Every callout uses this.
