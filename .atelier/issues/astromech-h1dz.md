---
created_at: "2026-10-15T20:36:13.655765593+00:00"
id: "astromech-h1dz"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-nugx"
  - kind: "issue"
    id: "astromech-w1q2"
  - kind: "issue"
    id: "astromech-w62j"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:43:24.428794400+00:00"
status: "done"
title: "System Powerplay on arrival"
updated_at: "2026-10-16T20:43:24.428794400+00:00"
---

## Goal

Show the controlling power, state (exploited, fortified, stronghold, acquisition), control progress, reinforcement vs undermining and contesting powers; build the line spoken on arrival.

## Done when

An FSDJump fixture with Powerplay fields produces the expected summary and arrival line. Speaking it when enabled, and nothing when the setting is off, moved to astromech-gtza (needs the reaction engine, astromech-cmhv).

## Context

FSDJump `Powerplay*` fields. Journal only.

## Notes

- 2026-10-16 [note] Done in 9b67a35. Journal: SystemPowerplay on FSDJump/CarrierJump/Location (field names community-sourced: kayahr/ed-journal + quoted lines; ControlProgress semantics = fraction of the state band, per EliteMeritTracker's handling). astromech-powerplay::SystemPower (summary, work_for, spoken) and arrival_line (FSDJump/CarrierJump only, None when no activity). GameState.system_power; voice status.system_power. App: This system panel. Speech on arrival filed as astromech-gtza (blocked by astromech-cmhv); Done when updated accordingly.

## Close Reason

FSDJump fixtures with Powerplay fields give the expected summary and arrival line (evidence astromech-9pw5, astromech-d3xn); speaking it moved to astromech-gtza, blocked by astromech-cmhv.
