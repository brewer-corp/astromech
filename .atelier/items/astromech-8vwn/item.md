---
schema: atelier.item
schema_version: 3
id: astromech-8vwn
title: Test the truncated Ardent answer in Where to buy
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T23:17:08.503987161Z
---

## Goal

Ardent sorts exports cheapest first, so a truncated answer drops dearer markets and can understate how many of the list a station stocks; the 'Too many markets' warning is the only signal, and nothing tests it.

## Done when

- [ ] A Rust test covers a truncated answer in market::list and a component test renders the warning.

## Context

Deferred from the astromech-05bi review (crates/astromech-market/src/list.rs:151, app/src/lib/colonisation.ts:140).
