---
schema: atelier.item
schema_version: 3
id: astromech-6g4x
title: Tighten powerplay_arrival sentence budget and off-by-default test
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T02:44:29.456970978Z
---

## Goal

Tidy up two leftovers from the review of astromech-gtza.

## Done when

- [ ] - `sentences()` in crates/astromech-announce/src/powerplay_arrival.rs can never produce a sentence of more than 8 words. Today a long sentence with no comma passes through unsplit. Either split it further, or make the copy test cover every `arrival_line` shape so an over-budget line would fail it. - `is_off_by_default` in tests/powerplay_arrival.rs no longer has `.all()` assertions that pass on empty output.
