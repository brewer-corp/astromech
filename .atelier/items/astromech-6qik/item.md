---
schema: atelier.item
schema_version: 3
id: astromech-6qik
title: Armour lookups match only the current ship's armour
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T00:34:23.648352558Z
---

## Goal

Each armour grade in the outfitting catalogue carries ~47 ships' symbols, so a voice or picker search for 'military grade composite' in a Krait MkII can name a station selling only Adder armour. The app's palette already declines armour for this reason.

## Done when

- [ ] Armour searches (voice and picker) match only the current ship's armour symbol (GameState.ship or Spansh's Module.ship), with a test where another ship's armour isn't offered.

## Context

Deferred from the astromech-970a review (crates/astromech-galaxy/src/outfitting.rs:360, app/src/lib/outfitting.ts:55).
