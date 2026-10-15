---
created_at: "2026-10-15T20:36:13.651930072+00:00"
id: "astromech-ngyt"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-0zcu"
  - kind: "issue"
    id: "astromech-4zeh"
  - kind: "issue"
    id: "astromech-a63r"
  - kind: "issue"
    id: "astromech-drku"
  - kind: "issue"
    id: "astromech-ew3x"
  - kind: "issue"
    id: "astromech-gpee"
  - kind: "issue"
    id: "astromech-kv9k"
  - kind: "issue"
    id: "astromech-omi1"
  - kind: "issue"
    id: "astromech-rpsb"
  - kind: "issue"
    id: "astromech-wq3f"
  - kind: "issue"
    id: "astromech-ze13"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Event → reaction engine"
updated_at: "2026-10-15T20:36:13.651930072+00:00"
---

## Goal

Rules subscribe to journal and Status events and output speech, a toast in the app or overlay, or a Windows notification. Priority queue, per-rule toggle and cooldown, quiet during docking and combat, silent while replaying the journal.

## Done when

Replay tests show: a rule fires once per event within its cooldown; a disabled rule never fires; nothing is spoken during journal catch-up at startup.

## Context

Today only route progress speaks. Every callout uses this.
