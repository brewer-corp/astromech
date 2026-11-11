---
schema: atelier.item
schema_version: 3
id: astromech-vy9z
title: 'Ships page: accessible name for unnamed ships; link-open error test'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T20:09:16.942762236Z
---

## Goal

An unnamed ship's card link shows "Unnamed" but is labelled "CU-01, Imperial Cutter", so voice control saying "click Unnamed" misses it; and no test shows "Couldn't open the link" when opening a Coriolis/EDSY/Spansh/Inara link fails.

## Done when

- [ ] The label includes the visible text ("Unnamed CU-01, Imperial Cutter"); a component test covers the open error.

## Context

`app/src/modules/engineering/View.svelte:84`, `Build.svelte` `open()`. Found in review of astromech-z4jb (round 5, N1, N2).

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-z4jb (Ships page with live build stats)
