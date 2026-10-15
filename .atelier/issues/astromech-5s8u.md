---
created_at: "2026-10-15T20:30:24.600286177+00:00"
id: "astromech-5s8u"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-hw3c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Material cap alerts and trade-down suggestions"
updated_at: "2026-10-15T20:30:24.600286177+00:00"
---

## Goal

Alert when a material hits its cap ("Arsenic at 300/300") and suggest what to trade down at a trader.

## Done when

Replaying a fixture that fills a material raises one alert; docking at a trader suggests trades that free that slot.

## Context

A reaction rule over the materials ledger.
