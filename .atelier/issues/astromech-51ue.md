---
created_at: "2026-10-24T23:07:07.848384897+00:00"
id: "astromech-51ue"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-yn6y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Galaxy rework: route strip, Places finders, one search box"
updated_at: "2026-10-24T23:07:07.848384897+00:00"
---

## Goal

Replace the scattered place-finding pages and the header plot box with the model decided in the galaxy direction decision: a persistent route shown on every page, Places as a family of finders on one linked-place engine (Replace route / Add stop on every result, Find along route, chaining "find near this", recipes), and one "Find or plot" box shared with Ctrl+K and voice.

## Done when

To be broken into children once DESIGN.md and the compound-query spike are done.
