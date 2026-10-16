---
created_at: "2026-10-16T22:18:23.023963321+00:00"
id: "astromech-usck"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Outfitting search: nearest stations first, not newest rows"
updated_at: "2026-10-16T22:18:23.023963321+00:00"
---

## Goal

outfitting_search applies LIMIT 100 to module rows before sorting by distance, so for broad words the nearest station can be cut and one station's many matches crowd others out.

## Done when

A test with more than 100 matching rows returns the nearest station first, limited by station.

## Context

Deferred from the astromech-zig3 review.
