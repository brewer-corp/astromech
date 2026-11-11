---
schema: atelier.item
schema_version: 3
id: astromech-d4p9
title: Prototype the SteamVR overlay with cost logging
status: draft
mission: null
labels:
- size-m
- spike
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.664211831Z
---

## Goal

Spike S2, the build half: an overlay in Rust that logs GPU/CPU and frame-time cost, with both raw upload and texture sharing behind a switch.

## Done when

- [ ] The prototype builds and runs against SteamVR's null driver or a headset, logging its costs (evidence).

## Context

`astromech-overlay` has the toast view only.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-nf25 (VR overlay)
