---
schema: atelier.item
schema_version: 3
id: astromech-ak4v
title: Missions list page
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T03:12:10.189343365Z
---

## Goal

Show active missions with expiry (and destination/target faction) on a page in the app, e.g. Commander or its own module. GameState already carries `missions` (serialised to the UI).

## Done when

- [ ] A component test renders a missions list with time left from a state fixture.
