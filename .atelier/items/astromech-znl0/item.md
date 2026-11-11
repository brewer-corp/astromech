---
schema: atelier.item
schema_version: 3
id: astromech-znl0
title: Lines dropped by the quiet gate are still counted as said
status: draft
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T03:14:02.683998940Z
---

## Goal

Once-per-X rules shouldn't lose a line for good when the engine's quiet gate drops it.

## Done when

- [ ] - A signal (e.g. a conflict zone) found during combat, an interdiction or a docking approach is still spoken once the quiet ends, or the next time it's seen in the same system. - A test covers that case.

## Context

`signals` (crates/astromech-announce/src/signals.rs) records a line as said before `fire()` in lib.rs clears sub-Urgent speech while quiet, so a signal found in combat, an interdiction or a docking approach is never spoken in that system.

Check the other rules with "said" state too (missions thresholds, materials_cap fills, powerplay_arrival). The fix probably belongs in the engine: tell the rule whether its line was actually spoken, or retry dropped lines. This came from the review of astromech-ew3x, R1 (medium).
