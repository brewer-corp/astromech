---
created_at: "2026-10-15T20:30:24.613626269+00:00"
id: "astromech-ns32"
issue_type: "spike"
labels:
- "needs-cmdr"
- "size-m"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-osyk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Spike: SteamVR overlay next to Elite"
updated_at: "2026-10-15T20:30:24.613626269+00:00"
---

## Goal

Spike S2: an overlay in Rust next to Elite: stability over 1 h, GPU/CPU and frame-time cost, raw upload vs texture sharing, Meta headsets via Steam Link and Virtual Desktop.

## Done when

A go/no-go report with measurements (evidence).

## Context

`astromech-overlay` has the toast view only.
