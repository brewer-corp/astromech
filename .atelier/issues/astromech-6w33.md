---
created_at: "2026-10-15T20:30:24.599925154+00:00"
id: "astromech-6w33"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "High Grade Emissions callout"
updated_at: "2026-10-15T20:30:24.599925154+00:00"
---

## Goal

On `FSSSignalDiscovered` with a High Grade Emissions USS, announce it and name the likely pool from the system's factions (EDCoPilot's "HGE candidates"). On arrival, say whether this is a good HGE system for the shopping list.

## Done when

Replay tests: an HGE signal in a Famine system names the right pool; non-HGE signals stay quiet.

## Context

The pool logic exists in `crates/astromech-materials/src/hge.rs`.
