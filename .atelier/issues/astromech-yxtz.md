---
created_at: "2026-10-24T23:27:52.556049922+00:00"
id: "astromech-yxtz"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Caps and orange per DESIGN.md: fix the tab hierarchy and take orange off values"
updated_at: "2026-10-24T23:27:52.556049922+00:00"
---

## Goal

Make caps and orange mean something again: caps only for the wordmark, top tabs, section heads and action words; orange only for selected, action, needs-you and focus (DESIGN.md: The Caps Are Wayfinding Rule, The One Signal Rule).

## Done when

- Active top tab is orange; active page tab is lit with a 2 px orange underline; inactive tabs are `--text-lo`.
- Uppercase + `--caps` appear only in the four allowed roles; column heads, labels, badges, status and sub-tabs are mixed case.
- No orange on values, numbers, units, dropdown text, chips, badges or status that needs no action; warnings and errors are an orange "!" plus plain words, never a filled block.
- Screenshots of Engineers, Trade, Keybinds and the route error before and after.

## Context

About 140 `text-transform: uppercase` rules (Engineers has three caps lines per card, two orange). The active top tab is white while the deepest tabs are orange; dropdown text, numbers, chips and the header status are orange; "Could not read the route" is a solid orange block.
