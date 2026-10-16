---
created_at: "2026-10-15T20:36:13.655229203+00:00"
id: "astromech-tw2y"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-4su3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Find the nearest service"
updated_at: "2026-10-16T20:16:04.662395080+00:00"
---

## Goal

Find the nearest Interstellar Factors, material trader **by type** (raw, manufactured, encoded), tech broker **by type** (human, Guardian), Universal Cartographics, Vista Genomics, Pioneer Supplies, bartender, Frontline Solutions, Apex, refuel/repair/rearm, black market, and search and rescue.

## Done when

Each service has a recorded-response test, and trader and broker searches return only the asked-for type.

## Context

Ardent `nearest/{service}` for the basic services; Spansh for trader and broker type, which Ardent doesn't distinguish.
