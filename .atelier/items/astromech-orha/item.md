---
schema: atelier.item
schema_version: 3
id: astromech-orha
title: 'Route parking: skip parking an identical route, and name the route this action replaced'
status: ready
mission: null
labels:
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T07:29:01.588168518Z
trivial: null
caused_by: null
created_at: 2026-11-05T07:29:01.588168518Z
---

## Goal

Following the same route twice never costs the commander their paused trip, and the "Replaced X" note names the route this action replaced.

## Done when

- [ ] - `Desk::follow` (crates/astromech-core/src/route.rs) does not park the replaced route when it equals the new one (same waypoints), so an older parked route survives a repeated Set destination or Follow; a Rust test covers it (finding astromech-n0t1). - The core reports which route a call parked (or the note is worded "Paused: <name>"), so `followedNote` in app/src/lib/route.ts never names a route parked earlier by a galaxy-map detour as "Replaced" (finding astromech-f1fo); fakecore mirrors it; tests cover both. - `cargo test -p astromech-core` and the app checks pass.

## Context

Deferred findings from the rz0f review (reviewer-fable-rz0f). Decision astromech-g8d5 item 3: Set destination parks the replaced route.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-rz0f (Add stop and Set destination on every lookup result)
