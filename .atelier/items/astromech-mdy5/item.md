---
schema: atelier.item
schema_version: 3
id: astromech-mdy5
title: 'Copy test backstop: handle constants, other crates and raw literals'
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
created_at: 2026-10-17T18:57:48.943731490Z
---

## Goal

Make the copy test's backstop (tests/copy.rs, say_literals) sturdier, or replace it with a typed registry of fixed spoken lines.

## Done when

- [ ] Constants and format! values passed to say(), lines in other crates' sources, raw strings and char literals are handled, or the lines live in one registry the test enumerates.

## Context

Review finding on PR #120 (astromech-yerd): the scan reads only astromech-core/src, sees string literals inside say(...) only, cuts a file at the first cfg(test), and does not parse char or raw literals.
