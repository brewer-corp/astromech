---
schema: atelier.item
schema_version: 3
id: astromech-278z
title: GPU timing for the Rust voice stack on Sumeet's PC with Elite in VR
status: draft
mission: null
labels:
- needs-human
- size-s
- validation
priority: medium
blocked_by:
- astromech-xec1
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T05:34:24.723280855Z
---

## Goal

Measure whether speech on the RX 9070 XT (burn on wgpu, DX12 or Vulkan) beats the CPU while Elite runs in the headset, and whether it causes frame drops or reprojection.

## Done when

- [ ] Sumeet has run the probe the runtime spike ships, with Elite in VR, and its results JSON is attached as an evidence receipt: CPU and GPU decode medians and p95, first-run kernel compile time, and any reprojection seen.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-v6r3 (Voice stack without sherpa-onnx)
