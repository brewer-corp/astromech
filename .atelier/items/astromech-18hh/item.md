---
schema: atelier.item
schema_version: 3
id: astromech-18hh
title: First-footfall detection by screen capture
status: draft
mission: null
labels:
- size-l
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.665699675Z
---

## Goal

Capture the screen after `Disembark` and read the first-footfall banner with OCR.

## Done when

- [ ] On test captures, the banner is detected in English with no false positives.

## Context

Not in the journal (manual v35; EDCoPilot's feature list says the same). Fragile with language, VR and HUD mods; the strategy rules it out for now.
