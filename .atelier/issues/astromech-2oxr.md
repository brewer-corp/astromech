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
closed_at: "2026-10-17T01:58:01.809924613+00:00"
status: "done"
title: "Route hazards from NavRoute"
updated_at: "2026-10-17T01:58:01.809924613+00:00"
---

## Goal

From `NavRoute.json`: non-scoopable stars and fuel risk, neutron and white dwarf stars, jumps remaining and total distance.

## Done when

A route fixture with a white dwarf and a fuel gap gives one hazard line for each.

## Context

Star class is in NavRoute.

## Close Reason

A plotted route gives one line per hazard (fuel risk, summary, white dwarfs, neutron stars) from NavRoute.json, fed to the engine as a deduplicated route event (evidence astromech-u4nf).
