---
schema: atelier.item
schema_version: 3
id: astromech-a92t
title: Fleet transfer arrivals read the command layer's clock
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T21:47:30.701520934Z
---

## Goal

In world mode a fleet transfer lands on the world clock, like every other timed thing in the core.

## Done when

- [ ] `JournalState` (crates/astromech-api/src/state.rs:227, :252) judges transfer arrivals with `State::clock()` instead of a hard-coded `SystemClock`, and a world test shows a transfer arriving only after an `advance` past its time. The installed app is unchanged (SystemClock).

## Context

Finding astromech-86af from the astromech-zgoo review; predates zgoo (astromech-6siv). Sibling of astromech-2ebj.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: astromech-86af
