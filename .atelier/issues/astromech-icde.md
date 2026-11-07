---
created_at: "2026-11-07T05:34:15.555625317+00:00"
id: "astromech-icde"
issue_type: "feature"
labels:
- "size-l"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7yb5"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xec1"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Kokoro ported to burn"
updated_at: "2026-11-07T05:34:15.555625317+00:00"
---

## Goal

Run Kokoro v1.0 natively in burn from its PyTorch weights (no ONNX, so none of Loop/If/sequence ops), on CPU and wgpu, so TTS leaves sherpa and can use the GPU.

## Done when

- A burn Kokoro renders from the misaki-rs phonemes with output matching today's Kokoro ONNX output within a tolerance set by golden tests (waveform and mel distance), for all English voices we offer.
- TTS first-audio on Sumeet's PC is no worse than S9's 446 ms median.

## Context

References: any-tts and rgbkrk/voice (candle ports). Parts: ALBERT text encoder, prosody predictor (LSTMs, AdaIN), text encoder, iSTFTNet decoder.

## Unknowns

Whether wgpu helps or hurts with Elite in VR on the same GPU: from the runtime probe and Sumeet's Windows run.
