---
schema: atelier.item
schema_version: 3
id: astromech-5c5t
title: Derive pressed_actions from the tool enums so a new variant cannot be skipped
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
created_at: 2026-10-17T18:43:12.151095455Z
---

## Goal

Make the health-check coverage test unable to miss a new tool action.

## Done when

- [ ] `astromech_tools::pressed_actions` is derived so that adding a Toggle, Screen or ToolCall variant that presses a key cannot be left out of `Toggle::all()` / `Screen::all()` (for example an exhaustive match in a test, or a length check against the variant count).

## Context

Review finding R1 on PR #114 (astromech-g1ys): the arrays are hand-written and not compiler-checked against the variants.
