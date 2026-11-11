---
schema: atelier.item
schema_version: 3
id: astromech-toqc
title: 'Prospector dedupe: unparsable timestamps, reuse open_store in app.rs'
status: draft
mission: null
labels:
- alpha
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T19:15:07.381630854Z
---

## Goal

Two small leftovers from the prospector dedupe review: (1) when a journal timestamp does not parse, the rock memory uses time 0, so a rock is silenced for good; speak instead or use the wall clock. (2) app.rs prospector_set and shopping_set repeat the create-folder-then-open steps that callouts::open_store now holds; use it.

## Done when

- [ ] Both are tidied with a test for the unparsable timestamp.

## Context

Review findings 4 and 5 on PR #123 (astromech-u8ir), both low.
