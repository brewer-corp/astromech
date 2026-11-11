---
schema: atelier.item
schema_version: 3
id: astromech-nv75
title: HGE lookup fails when only the population page fails
status: draft
mission: null
labels:
- bug
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T22:42:24.932310422Z
---

## Goal

The HGE lookup fetches a nearest page and a population-filtered page from Spansh. If only the population page fails, the whole lookup errors; it should fall back to the nearest page.

## Done when

- [ ] A test with the population request failing and the nearest request succeeding returns ranked results from the nearest page, with its fetched age.

## Context

Deferred from the review of the astromech-sfys fixes (crates/astromech-galaxy/src/lib.rs:255).
