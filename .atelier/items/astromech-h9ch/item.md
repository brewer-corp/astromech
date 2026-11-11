---
schema: atelier.item
schema_version: 3
id: astromech-h9ch
title: Check the Odyssey game data against the game
status: ready
mission: astromech-7q0z
labels:
- needs-human
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-16T22:33:31.666781527Z
trivial: null
caused_by: null
created_at: 2026-10-16T22:14:30.771392296Z
---

## Goal

Confirm in game the values `data/catalog/odyssey.json` is unsure of: its `uncertain` list (weapon upgrade prices the wiki leaves blank, sale prices for a few data and goods items, engineer/mod pairs the wiki contradicts itself on such as Hero Ferrari and Increased Sprint Duration, Rosa Dayette and Greater Range/Scope/Stability), and that the wiki infobox "value" is what a bartender pays.

## Done when

- [ ] Each uncertain value is confirmed or corrected (in scripts/gen-odyssey.py as an override, or upstream on the wiki and regenerated), and the list in the file is shorter.

## Context

Follow-up of astromech-j4c9.

## Notes

- 2026-10-16 [note] Review of astromech-j4c9 (R4, astromech-ra7z): two suspicious wiki weapon upgrade prices to check in game. Manticore Intimidator grade 2 at 60,000 CR (other Manticore G2 steps are in the hundreds of thousands), and Manticore Tormentor grade 2 at 500,000 CR against 750,000 CR at grade 3 (G2 vs G3 ratio unlike the other weapons). Both are in data/catalog/odyssey.json as the wiki gives them and are not yet in its uncertain list.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-j4c9 (Odyssey micro-resources)
