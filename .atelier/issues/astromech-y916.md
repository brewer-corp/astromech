---
created_at: "2026-10-15T20:36:13.650423373+00:00"
id: "astromech-y916"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-505u"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "High Grade Emissions callout"
updated_at: "2026-10-15T20:36:13.650423373+00:00"
---

## Goal

On `FSSSignalDiscovered` with a High Grade Emissions USS, announce it and name the likely pool from the system's factions (EDCoPilot's "HGE candidates"). On arrival, say whether this is a good HGE system for the shopping list.

## Done when

Replay tests: an HGE signal in a Famine system names the right pool; non-HGE signals stay quiet.

## Context

The pool logic exists in `crates/astromech-materials/src/hge.rs`.
