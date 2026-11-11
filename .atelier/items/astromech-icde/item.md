---
schema: atelier.item
schema_version: 3
id: astromech-icde
title: Kokoro ported to burn
status: draft
mission: null
labels:
- size-l
- feature
priority: medium
blocked_by:
- astromech-7yb5
- astromech-xec1
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T05:34:15.555625317Z
---

## Goal

Run Kokoro v1.0 natively in burn from its PyTorch weights (no ONNX, so none of Loop/If/sequence ops), on CPU and wgpu, so TTS leaves sherpa and can use the GPU.

## Done when

- [ ] - A burn Kokoro renders from the misaki-rs phonemes with output matching today's Kokoro ONNX output within a tolerance set by golden tests (waveform and mel distance), for all English voices we offer. - TTS first-audio on Sumeet's PC is no worse than S9's 446 ms median.

## Context

References: any-tts and rgbkrk/voice (candle ports). Parts: ALBERT text encoder, prosody predictor (LSTMs, AdaIN), text encoder, iSTFTNet decoder.

## Unknowns

Whether wgpu helps or hurts with Elite in VR on the same GPU: from the runtime probe and Sumeet's Windows run.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-v6r3 (Voice stack without sherpa-onnx)
