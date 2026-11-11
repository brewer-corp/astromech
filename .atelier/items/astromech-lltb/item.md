---
schema: atelier.item
schema_version: 3
id: astromech-lltb
title: Replace the hand-edited Spansh route fixtures with recorded ones
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T23:48:54.231542958Z
---

## Goal

The Spansh route fixtures are recorded raw answers (decision astromech-1zhg), not edited ones.

## Done when

- [ ] `crates/astromech-data/tests/fixtures/spansh-route-*.json` and `spansh-exact-*.json` (all-zero job ids, a 3-stop Sol to Colonia route) are re-recorded with `--record-net` from Spansh's public endpoints (no key, no commander) and checked for redaction; worlds/preview/sol, worlds/every-command and the crate tests that use them pass, and the preview drift test stays green.

## Context

Found by astromech-cwtv's implementer (mission astromech-kjfs), which re-recorded its own route for the journeys.
