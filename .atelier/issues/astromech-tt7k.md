---
created_at: "2026-10-15T20:30:24.604518257+00:00"
id: "astromech-tt7k"
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
title: "Find a ship near me"
updated_at: "2026-10-15T20:30:24.604518257+00:00"
---

## Goal

Search shipyards for a ship type and list the nearest, with pad size.

## Done when

A recorded-response test returns the nearest shipyards selling the ship, in distance order, with ships too big for a station's pads excluded.

## Context

Same Spansh endpoint as the module search, with a ship filter.
