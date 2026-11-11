---
schema: atelier.item
schema_version: 3
id: astromech-l2yy
title: 'Learned phrases: surface forget errors, refresh the list, update the control doc'
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
created_at: 2026-10-17T19:25:24.465705565Z
---

## Goal

Tidy three small leftovers of the learned-phrases list.

## Done when

- [ ] (1) Assistant::forget returns the store error so the Voice tester shows it instead of an unchanged list. (2) The list refreshes when a phrase is learned while the page is open (for example on the page's poll). (3) The Request doc comment in crates/astromech-api/src/control.rs (moved from astromech-core by astromech-8s6k) lists learned_phrases and learned_forget.

## Context

Review findings R1, R2 and the stale comment on PR #130 (astromech-znvi), all low.
