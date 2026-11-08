---
created_at: "2026-11-07T21:47:30.701520934+00:00"
id: "astromech-a92t"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "review"
    id: "astromech-86af"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Fleet transfer arrivals read the command layer's clock"
updated_at: "2026-11-07T21:47:30.701520934+00:00"
---

## Goal

In world mode a fleet transfer lands on the world clock, like every other timed thing in the core.

## Done when

- [ ] `JournalState` (crates/astromech-api/src/state.rs:227, :252) judges transfer arrivals with `State::clock()` instead of a hard-coded `SystemClock`, and a world test shows a transfer arriving only after an `advance` past its time. The installed app is unchanged (SystemClock).

## Context

Finding astromech-86af from the astromech-zgoo review; predates zgoo (astromech-6siv). Sibling of astromech-2ebj.
