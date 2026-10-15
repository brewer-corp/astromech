---
created_at: "2026-10-15T20:30:24.601402528+00:00"
id: "astromech-s21p"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-2fq8"
  - kind: "issue"
    id: "astromech-56w8"
  - kind: "issue"
    id: "astromech-5s8u"
  - kind: "issue"
    id: "astromech-631g"
  - kind: "issue"
    id: "astromech-drod"
  - kind: "issue"
    id: "astromech-e0iv"
  - kind: "issue"
    id: "astromech-f3z8"
  - kind: "issue"
    id: "astromech-lbv0"
  - kind: "issue"
    id: "astromech-m4d5"
  - kind: "issue"
    id: "astromech-n6b5"
  - kind: "issue"
    id: "astromech-ssba"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Event → reaction engine"
updated_at: "2026-10-15T20:30:24.601402528+00:00"
---

## Goal

Rules subscribe to journal and Status events and output speech, a toast in the app or overlay, or a Windows notification. Priority queue, per-rule toggle and cooldown, quiet during docking and combat, silent while replaying the journal.

## Done when

Replay tests show: a rule fires once per event within its cooldown; a disabled rule never fires; nothing is spoken during journal catch-up at startup.

## Context

Today only route progress speaks. Every callout uses this.
