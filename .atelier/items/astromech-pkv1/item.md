---
schema: atelier.item
schema_version: 3
id: astromech-pkv1
title: 'Market comparison: handle truncated nearby answers for carried goods'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T02:45:47.141212496Z
---

## Goal

Make the Station market comparison handle a truncated nearby answer for carried goods.

## Done when

- [ ] - `compare_here` (crates/astromech-market/src/here.rs) handles `Prices.truncated` for carried lines, the way the trade cargo plan does with its second request, so a market that can take the whole load is never cut before ranking. - A test covers a truncated answer. - From the review of astromech-qw2m, finding R3 (low).
