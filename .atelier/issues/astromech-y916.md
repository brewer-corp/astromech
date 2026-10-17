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
closed_at: "2026-10-17T02:01:10.055210005+00:00"
status: "done"
title: "High Grade Emissions callout"
updated_at: "2026-10-17T02:01:10.055210005+00:00"
---

## Goal

On `FSSSignalDiscovered` with a High Grade Emissions USS, announce it and name the likely pool from the system's factions (EDCoPilot's "HGE candidates"). On arrival, say whether this is a good HGE system for the shopping list.

## Done when

Replay tests: an HGE signal in a Famine system names the right pool; non-HGE signals stay quiet.

## Context

The pool logic exists in `crates/astromech-materials/src/hge.rs`.

## Close Reason

HGE signals are announced with the spawning faction's likely pool (Famine system case tested), other signals stay quiet, and arrival says 'Good for <material>' for a billion-plus system whose allegiance pool has a wanted material (evidence astromech-21jd).
