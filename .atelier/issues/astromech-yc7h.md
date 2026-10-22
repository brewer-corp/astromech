---
created_at: "2026-10-22T21:47:06.175183217+00:00"
id: "astromech-yc7h"
issue_type: "decision"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Two commanders in one journal folder: old-journal joins and the Powerplay pledge"
updated_at: "2026-10-22T21:47:06.175183217+00:00"
---

## Context

Review findings R3 (astromech-s3kr) and R4 (astromech-sivk) on feature/astromech-fdh3: crates/astromech-history/src/lib.rs:602-626, crates/astromech-powerplay/src/standing.rs:103.

## Question

Two edge cases only matter when one journal folder holds more than one commander. Which behaviour should astromech have?

1. **Old-journal joins (astromech-yrfv):** events from journals too old to carry a commander FID are filed under `name:X` and joined to the one FID commander named X. If a second FID commander named X appears later, the join made during normal imports stays, but a full recompute of commanders would split it again, so the two paths disagree.
2. **Powerplay pledge (astromech-xvov):** the pledge clears when a game run's first login reaches `Missions` without a `Powerplay` event. Logging in first as an unpledged commander, then relogging from the main menu as the pledged one in the same run (which writes no `Powerplay` event), shows "Not pledged" until the next game start. Before this change the page showed the wrong pledge in that case, so it is not a regression.

## Options

- Accept both as they are (rare: two commanders in one folder, same name or same-run switching).
- Make the join reversible, or the recompute keep it, so both paths agree.
- For the pledge, keep the pledge per commander and clear it only for the commander whose first login lacked `Powerplay`.
