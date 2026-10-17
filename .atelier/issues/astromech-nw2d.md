---
created_at: "2026-10-16T18:49:57.061032597+00:00"
id: "astromech-nw2d"
issue_type: "bug"
labels:
- "alpha"
workflow_branch:
  branch_base: wt-base-nw2d
  work_branch: bug/astromech-nw2d
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Undocked leaves the station set"
updated_at: "2026-10-17T17:03:51.886935288+00:00"
---

## Goal

After undocking, "where am I" still answers "Docked at X": `Undocked` clears `station_market_id` but not `station` (`crates/astromech-gamestate/src/lib.rs`, used by `Query::Location` in `crates/astromech-tools/src/lib.rs`). A Location that is not docked already clears both, so the two exits from a station disagree.

## Done when

A game-state test shows `Undocked` clears the station, and the Location query no longer says docked after undocking.

## Context

Found in review of astromech-tpdb (round 2, finding N1); predates that branch.
