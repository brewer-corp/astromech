---
schema: atelier.item
schema_version: 3
id: astromech-cgrw
title: 'M3 exit test: eval on the VR clip set'
status: ready
mission: astromech-610m
labels:
- needs-human
- size-s
- validation
priority: critical
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-18T00:06:47.295569535Z
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.656533845Z
---

## Goal

Run `--eval` on Sumeet's VR clip set and check at least 95% execute-or-ask.

## Done when

- [ ] The eval report shows ≥ 95% execute-or-ask, attached as an evidence receipt.

## Context

The harness exists (`save_clips`, `--eval`). Needs the clip set recorded in VR.

## Notes

- 2026-10-17 [note] 17 Oct: since astromech-e0zz the audio kept after push-to-talk release is the configured post_roll_ms (200 ms), down from about 260 ms. Listen for clipped command endings in the clip set; raise post_roll_ms if they clip.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-0474 (M3 exit: voice in the COVAS register)
