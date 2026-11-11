---
schema: atelier.item
schema_version: 3
id: astromech-y14z
title: Run the voice eval with the Mining hotspots phrases before the next release
status: ready
mission: null
labels:
- size-s
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-05T09:13:45.458444355Z
trivial: null
caused_by: null
created_at: 2026-11-05T09:13:45.458444355Z
---

## Goal

The new `mining.hotspots` phrases and `{mineral}` slot words (painite, LTDs, low temp diamonds, void opals and the rest) are measured by speech recognition before they ship.

## Done when

- [ ] - `scripts/voice-eval.sh` runs on a release sherpa build (on boat, per the boat skill; it is too heavy for the live host) with the current 200-phrase set, and its results are recorded as evidence on this issue, including the two hotspot phrases and the trade.sell / find.service phrases they replaced. - Any phrase below the eval's bar gets an issue.

## Context

Deferred finding astromech-geeg from the f7i0 review: the eval set swap is within CONTRIBUTING's rules and the phrase-coverage test passes, but the audio eval was not rerun.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-51ue (Galaxy rework: route strip, Places finders, one search box)
