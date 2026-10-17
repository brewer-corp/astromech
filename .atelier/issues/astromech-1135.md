---
created_at: "2026-10-17T16:53:47.902127587+00:00"
id: "astromech-1135"
issue_type: "feature"
labels:
- "alpha"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-5akj"
  - kind: "issue"
    id: "astromech-5sbu"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Alpha: land the route follower and the Go button"
updated_at: "2026-10-17T17:01:44.545907856+00:00"
---

## Goal

Everything already built but not on main is merged, so the alpha is cut from one tree.

## Done when

Every child issue is done.

## Scope

The route follower from the `route-clipboard` checkout (astromech-5sbu, about 1,290 commits behind main, so effectively a re-land on top of main), the Go button (astromech-5akj).
