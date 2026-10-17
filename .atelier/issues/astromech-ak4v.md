---
created_at: "2026-10-17T03:12:10.189343365+00:00"
id: "astromech-ak4v"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Missions list page"
updated_at: "2026-10-17T03:12:10.189343365+00:00"
---

## Goal

Show active missions with expiry (and destination/target faction) on a page in the app, e.g. Commander or its own module. GameState already carries `missions` (serialised to the UI).

## Done when

A component test renders a missions list with time left from a state fixture.
