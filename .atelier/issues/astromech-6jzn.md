---
created_at: "2026-10-15T20:36:13.658801674+00:00"
id: "astromech-6jzn"
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
closed_at: "2026-10-17T01:54:04.387335730+00:00"
status: "done"
title: "Fuel: next scoopable star on the route"
updated_at: "2026-10-17T01:54:04.387335730+00:00"
---

## Goal

Say the next scoopable star on the route, and warn when there isn't enough fuel to reach one. The HUD's low-fuel warning and scoop progress aren't repeated.

## Done when

Route fixtures: one with a reachable scoopable star names it; one without enough fuel warns once.

## Context

Status plus NavRoute.

## Close Reason

On a route, arrival at a non-scoopable star says the jumps to the next scoopable one, and fuel_range warns once (10 min cooldown) when the main tank won't reach it (evidence in .atelier/evidence for 6jzn). Lines give the jump count, not the system name, to keep within the 8-word copy budget.
