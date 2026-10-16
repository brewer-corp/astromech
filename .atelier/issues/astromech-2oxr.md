---
created_at: "2026-10-15T20:36:13.654316291+00:00"
id: "astromech-2oxr"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
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
title: "Route hazards from NavRoute"
updated_at: "2026-10-15T20:36:13.654316291+00:00"
---

## Goal

From `NavRoute.json`: non-scoopable stars and fuel risk, neutron and white dwarf stars, jumps remaining and total distance.

## Done when

A route fixture with a white dwarf and a fuel gap gives one hazard line for each.

## Context

Star class is in NavRoute.
