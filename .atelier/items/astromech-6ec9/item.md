---
schema: atelier.item
schema_version: 3
id: astromech-6ec9
title: 'Spike: int8 Parakeet Unified that parakeet-rs loads'
status: ready
mission: null
labels:
- spike
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-08T18:51:31.769799110Z
trivial: null
caused_by: null
created_at: 2026-11-08T18:51:14.729776005Z
---

## Goal

Make an int8 Parakeet Unified that parakeet-rs 0.4.4 loads on ONNX Runtime, and check it matches the sherpa int8 export, so Unified can be tried without adding anything to sherpa.

## Done when

- [ ] `encoder.int8.onnx` and `decoder_joint.int8.onnx` made from the parakeet-rs export (`altunenes/parakeet-rs`, `parakeet-unified-en-0.6b`, fp32) with `onnxruntime.quantization.quantize_dynamic`, int8 MatMul weights; the onnxruntime version, source revision and SHA-256s recorded.
- [ ] `spikes/v6-stt-compare --backend unified` on the int8 folder scores within the noise of the sherpa int8 export on the 3200-clip set (3112/3136, WER 5.64%, decode median 110 ms) and on the cloned-voice set (2209/2224).
- [ ] Findings in `docs/evidence/astromech-ggcx/` next to the earlier runs, with a go or no-go for the shadow decode.

## Context

- parakeet-rs checks `encoder.onnx` and `decoder_joint.onnx` before the `.int8.` names (`model_unified.rs:108-129`), so the int8 folder holds no fp32 files.
- The sherpa int8 export (csukuangfj2) splits decoder and joiner, so parakeet-rs cannot load it. altunenes publishes Unified as fp32 only (2.5 GB).
- Quantising needs about 10 GB of RAM: run it on boat (`large`), not here.
- Unified is under the NVIDIA Open Model License. A file we host or point users at carries its notice.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-v6r3 (Voice stack without sherpa-onnx)
