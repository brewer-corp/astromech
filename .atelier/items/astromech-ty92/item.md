---
schema: atelier.item
schema_version: 3
id: astromech-ty92
title: 'MVP criteria harness: read the action-start constants from their sources'
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
created_at: 2026-10-17T19:22:54.406839126Z
---

## Goal

The action-start estimate in scripts/mvp_criteria.py uses debounce 150, post-roll 200, mic 10 and STT 104 ms copied from config.rs defaults and S9. Read them from their sources (or check them in a test) so they can't drift.

## Done when

- [ ] A change to the config defaults changes the estimate, or a test fails until the harness is updated.

## Context

Review of astromech-08w8 (R1, deferred).
