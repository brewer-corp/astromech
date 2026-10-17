---
created_at: "2026-10-17T01:45:29.110027209+00:00"
id: "astromech-8sx3"
issue_type: "task"
labels:
- "alpha"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Bodies known to EDSM in the arrival briefing"
updated_at: "2026-10-17T17:01:41.827505765+00:00"
---

## Goal

On arrival, say how much of the system EDSM already knows (bodies known against the system's body count), so the commander knows whether it is worth scanning. Split from astromech-nugx, which built the arrival briefing from the journal alone.

## Done when

A replay test with a recorded EDSM bodies answer gives the bodies-known line on a live jump, nothing during catch-up, and nothing (no stall, a logged error) when EDSM is offline or slow. The pipeline thread never waits on the network.

## Scope

Copy within the 8-word callout budget (crates/astromech-core/tests/copy.rs).

## Context

`astromech_data::Client::edsm_bodies` exists (cached, rate limited). The reaction engine's rules are synchronous and see only the event and the game state, so this needs a way to deliver a callout after a lookup finishes: for example a lookup thread that writes the answer into the game state (and a rule that fires on it), or a reaction delivered straight to the callout router with the engine's toggle and quiet rules. That choice is the design part. The in-game undiscovered flag (AutoScan WasDiscovered) is already spoken.
