---
schema: atelier.item
schema_version: 3
id: astromech-fq4o
title: Inara profile sync
status: draft
mission: astromech-7q0z
labels:
- size-m
priority: high
blocked_by:
- astromech-3zoz
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.660858408Z
---

## Goal

Batched events to Inara: location, ship and loadout, ranks, reputation, credits, engineers, materials, missions and CG progress. At most one send per ~35 s; send on start and end, `Docked` and `FSDJump`.

## Done when

- [ ] Fixture events produce batches within the rate limit, and a live send is accepted (evidence).

## Context

**Blocker:** Inara's author must whitelist the app name. Ask early, since it has lead time. Per-user key in the OS keyring. Inara has no galaxy or market reads for apps.
