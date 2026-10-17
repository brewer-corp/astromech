---
created_at: "2026-10-17T16:53:47.902127587+00:00"
id: "astromech-1135"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-5akj"
  - kind: "issue"
    id: "astromech-5sbu"
  - kind: "issue"
    id: "astromech-aare"
  - kind: "issue"
    id: "astromech-x53n"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Alpha: land the finished work"
updated_at: "2026-10-17T16:54:23.348398133+00:00"
---

## Goal

Everything already built but not on main is merged, so the alpha is cut from one tree.

## Done when

Every child issue is done.

## Scope

The route follower from the `route-clipboard` checkout (astromech-5sbu, about 1,290 commits behind main, so effectively a re-land on top of main), the Go button (astromech-5akj), astromech-x53n (done on its branch, not merged) and `task/astromech-3cxd` (per-PR CI trim, one commit, record only on its branch).
