---
schema: atelier.item
schema_version: 3
id: astromech-v6r3
title: Voice stack without sherpa-onnx
status: ready
mission: null
labels:
- size-l
- feature
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-08T18:51:23.938826148Z
trivial: null
caused_by: null
created_at: 2026-11-07T05:34:14.631474764Z
---

## Goal

Replace sherpa-onnx (heavy C++, GPL-3.0 through espeak-ng, MSVC-only prebuilts) with a Rust stack, and move speech work onto the GPU where it pays, since Elite tends to be CPU-bound. Direction from the 7 Nov 2026 survey: Parakeet through parakeet-rs (ort, or burn on wgpu), Kokoro ported to burn by hand, misaki-rs plus `speakable.rs` as the phonemiser, earshot for VAD. The children gate each step on a measurement.

## Done when

- The voice worker builds and runs without the sherpa-onnx crate, and `cargo deny` no longer needs the GPL-3.0 allowance for it.
- `scripts/voice-eval.sh` on the new stack scores at least as well as the sherpa baseline (docs/evidence/astromech-xi2i/report.md: 3123/3136 execute-or-ask, 0 wrong executes).
- [ ] After merge (needs Sumeet's PC): release-to-transcript and TTS first-audio latency are no worse than S9 (ARCHITECTURE.md §17.6: 85 ms median, 446 ms median). Stand-in: the same latencies on this box, no worse than sherpa's.

## Context

- Today's use of sherpa: Silero VAD trim, Parakeet TDT 0.6B v2 int8 (`OfflineRecognizer`), Kokoro v1.0 with espeak-ng (crates/astromech-voice, ARCHITECTURE.md §17.5).
- Options surveyed: parakeet-rs 0.4 (ort default; burn backend on fp32 files only), rten (pure-Rust CPU ONNX runtime with DynamicQuantizeLinear/MatMulInteger/ConvInteger), burn-onnx (no dynamic-quant ops, no Loop/If/sequence ops for Kokoro), candle (no AMD GPU on Windows), parakeet.cpp on ggml (C++, Vulkan/HIP), any-tts and rgbkrk/voice (candle Kokoro ports, reference only).
- Parakeet Unified (nvidia/parakeet-unified-en-0.6b, 7 Apr 2026): RNNT, English only, 5.91 vs 6.04 offline WER against TDT v2 on the Open ASR set, streams at 0.56 s chunks with ~6.5 WER. NVIDIA Open Model License, not CC-BY-4.0.
