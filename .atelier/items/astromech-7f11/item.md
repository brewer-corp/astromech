---
schema: atelier.item
schema_version: 3
id: astromech-7f11
title: Speech-to-text on parakeet-rs, with hotword biasing
status: draft
mission: null
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-ggcx
- astromech-xec1
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T05:34:15.373831421Z
---

## Goal

Move speech-to-text off sherpa onto parakeet-rs with the model and runtime the two spikes choose, and own the decoding loop so we can bias it towards Elite names (the hotword feature sherpa blocks, V1-DESIGN.md §6.3 item 6).

## Done when

- [ ] - The worker's STT runs through parakeet-rs; `scripts/voice-eval.sh` scores at least the sherpa baseline. - Decoding takes a hotword list and the eval shows it helps the names it targets without new wrong executes.

## Unknowns

Which model and runtime: settled by the Unified and runtime spikes.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-v6r3 (Voice stack without sherpa-onnx)
