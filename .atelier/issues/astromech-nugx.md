---
created_at: "2026-10-15T20:36:13.658000951+00:00"
id: "astromech-nugx"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-2myu"
  - kind: "issue"
    id: "astromech-505u"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T01:45:34.689552708+00:00"
status: "done"
title: "Arrival briefing"
updated_at: "2026-10-17T01:45:34.689552708+00:00"
---

## Goal

On `FSDJump`, speak only what isn't on the HUD: notable states for the user's goals (Boom/Famine/War against the shopping list), Powerplay state, bodies known (EDSM), and an "undiscovered" flag. Times visited, bodies landed on and known biology come later from the history and exobiology work.

## Done when

Replay tests: a system matching the shopping list gets its state line; a visited, fully known system with nothing notable gets nothing; star class and security are never spoken.

## Context

The single most-used EDCoPilot feature.

## Close Reason

Arrival rule speaks a shopping-list faction state, Undermining/Acquisition target for the pledged power, and Undiscovered system from the arrival AutoScan; star class, security and allegiance never spoken (evidence astromech-knhn). Bodies known to EDSM split to astromech-8sx3; visit history stays with astromech-2myu.
