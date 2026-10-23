---
created_at: "2026-10-23T01:00:07.222048616+00:00"
id: "astromech-g0hm"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-5k55"
  - kind: "issue"
    id: "astromech-ayw1"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Plot from the top bar, and follow routes plotted in the game"
updated_at: "2026-10-23T01:00:07.557251943+00:00"
---

## Goal

Plotting is a first-class action: a system search in the window's top bar plots a route from the current system, and a route plotted in the game's galaxy map shows up in astromech without breaking a Spansh neutron route being followed.

## Done when

- The children are done.

## Context

Sumeet, 22 Oct 2026: the Plot sub-panel in the route rail keeps closing (its `<details open={!route}>` is re-set on every 2 s poll), it should be much more prominent, the top bar should search systems directly, and `From` goes: plotting is always from the current system. Decisions taken: an in-game detour off a followed route is followed, and the old route is parked and resumable; routes mirrored from the game are silent and leave the clipboard alone.
