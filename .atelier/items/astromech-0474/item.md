---
schema: atelier.item
schema_version: 3
id: astromech-0474
title: 'M3 exit: voice in the COVAS register'
status: ready
mission: astromech-610m
labels:
- feature
priority: critical
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-16T18:26:25.247824052Z
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.647520981Z
---

## Goal

Voice replies use the speaker the user picks in Settings (astromech-orel), through the Cockpit chain tuned in the headset to sit in the same register as in-game COVAS, and recognition passes the 95% execute-or-ask bar on the VR clip set.

## Done when

- [ ] The eval run's report, and Sumeet's verdict from the headset, recorded as evidence. Delivers v1.0 S1.

## Scope

Already built: Kokoro TTS with the Cockpit colour chain (band-limit, presence, compression, saturation, comb, pip), Natural/Cockpit/Harsh presets, the coloured confirmation cache and a cuttable mixer (`astromech_voice::colour`, `mixer`; V1-DESIGN §6.4, §7.12). The speaker is already a per-request `sid` and part of the TTS cache key, so switching voices needs no restart. Which speaker ships as the default is not a v1.0 question.

## Design

[V1-DESIGN §6.4](docs/V1-DESIGN.md).

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-a50f (Switch text-to-speech to Kokoro v1.0)
- related: astromech-orel (Settings → Voice responses)
