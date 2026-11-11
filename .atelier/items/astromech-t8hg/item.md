---
schema: atelier.item
schema_version: 3
id: astromech-t8hg
title: Check which Odyssey events rewrite Backpack.json and ShipLocker.json
status: ready
mission: null
labels:
- needs-human
- size-s
- validation
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T17:17:34.412349339Z
trivial: null
caused_by: null
created_at: 2026-10-17T17:17:34.412349339Z
---

## Goal

Find out in the game which on-foot events rewrite Backpack.json or ShipLocker.json, so the companion-file triggers (`crates/astromech-journal/src/companion.rs`) re-read them at the right moments.

## Done when

- [ ] A note lists, for each of those events, which file changed (watch the files' modified times while doing each action once), and an issue is filed for any trigger that is missing.

## Context

astromech-ogq4 found that the game rewrites Backpack.json with each BackpackChange, with no Backpack event. CollectItems, DropItems, UseConsumable, and micro-resource buy, sell, trade and transfer events may also rewrite one of the files; that can't be checked without the game.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-ogq4 (Companion files: stale Backpack snapshot and lost watcher notifications)
