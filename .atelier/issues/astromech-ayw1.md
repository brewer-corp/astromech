---
created_at: "2026-10-23T01:00:07.556556377+00:00"
id: "astromech-ayw1"
issue_type: "feature"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T01:23:59.868271597+00:00"
status: "done"
title: "Follow routes plotted in the galaxy map"
updated_at: "2026-10-23T01:23:59.868271597+00:00"
---

## Goal

When the commander plots a route in the game, astromech follows it too, without breaking a Spansh route whose leg the game plot is.

## Done when

- A live `NavRoute` whose destination is the followed route's next or a later waypoint is a leg: the route is kept.
- Any other live `NavRoute` becomes the followed route (source `game`, the jumps after the current system); a followed non-game route is parked and can be resumed, and a later in-game plot to one of the parked route's waypoints resumes it.
- Routes from the game move on as you jump but say nothing and never write the clipboard.
- `NavRouteClear` clears only a route from the game. Replayed journals change nothing.
- Rust tests cover each case; docs/PLOTTING.md describes the rules.

## Close Reason

Galaxy-map routes followed: legs kept, detours park our route, Resume, silent and no clipboard, NavRouteClear drops only its own; reviewed and approved. Ordering race deferred to astromech-k1fw
