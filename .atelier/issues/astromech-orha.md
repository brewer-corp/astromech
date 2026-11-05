---
created_at: "2026-11-05T07:29:01.588168518+00:00"
id: "astromech-orha"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-rz0f"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route parking: skip parking an identical route, and name the route this action replaced"
updated_at: "2026-11-05T07:29:01.588168518+00:00"
---

## Goal

Following the same route twice never costs the commander their paused trip, and the "Replaced X" note names the route this action replaced.

## Done when

- `Desk::follow` (crates/astromech-core/src/route.rs) does not park the replaced route when it equals the new one (same waypoints), so an older parked route survives a repeated Set destination or Follow; a Rust test covers it (finding astromech-n0t1).
- The core reports which route a call parked (or the note is worded "Paused: <name>"), so `followedNote` in app/src/lib/route.ts never names a route parked earlier by a galaxy-map detour as "Replaced" (finding astromech-f1fo); fakecore mirrors it; tests cover both.
- `cargo test -p astromech-core` and the app checks pass.

## Context

Deferred findings from the rz0f review (reviewer-fable-rz0f). Decision astromech-g8d5 item 3: Set destination parks the replaced route.
