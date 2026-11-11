---
schema: atelier.item
schema_version: 3
id: astromech-t71i
title: Check the bartender's barter rules against the game
status: ready
mission: astromech-7q0z
labels:
- needs-human
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T05:19:02.900011860Z
trivial: null
caused_by: null
created_at: 2026-10-17T05:19:02.900011860Z
---

## Goal

Check in game how the bartender's barter works, which the On foot page's trade plan (astromech-ha4m) assumes without having seen: that goods and data cannot be bartered (the wiki gives barter values for assets only; `data/catalog/odyssey.json` has none for goods or data), that a swap takes equal barter value both ways (no margin or fee), that the bartender will hand over whatever is asked for (no stock limit), and what happens to a trade that overshoots.

## Done when

- [ ] Each assumption is confirmed or corrected. If goods or data can be bartered, their barter values are added to `scripts/gen-odyssey.py` and the catalogue regenerated so the plan covers them; if the rate differs, `crates/astromech-odyssey/src/barter.rs` follows it.

## Context

Follow-up of astromech-ha4m. Like astromech-h9ch, it needs the game.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-ha4m (On foot: bartender trade plan for the shopping list)
