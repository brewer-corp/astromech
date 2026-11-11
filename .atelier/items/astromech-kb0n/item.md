---
schema: atelier.item
schema_version: 3
id: astromech-kb0n
title: 'Voice: count phrase confirmations by normalised text'
status: draft
mission: null
labels:
- voice
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-18T00:59:54.360818931Z
---

## Goal

Count "Did you mean" confirmations per normalised phrase, not per exact transcript string, so "Dear dawn." and "dear dawn" add up.

## Done when

- [ ] Confirmations of transcripts that normalise the same count together toward the two-yes rule for key presses; the learned list still shows what was heard; tests cover it.

## Context

Review finding R4 of astromech-1c66. Errs on the safe side today: variation only delays learning.
