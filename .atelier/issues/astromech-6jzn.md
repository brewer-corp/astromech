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
status: "todo"
title: "Fuel: next scoopable star on the route"
updated_at: "2026-10-15T20:36:13.658801674+00:00"
---

## Goal

Say the next scoopable star on the route, and warn when there isn't enough fuel to reach one. The HUD's low-fuel warning and scoop progress aren't repeated.

## Done when

Route fixtures: one with a reachable scoopable star names it; one without enough fuel warns once.

## Context

Status plus NavRoute.
