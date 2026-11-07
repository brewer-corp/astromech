---
created_at: "2026-11-07T05:34:14.998986555+00:00"
id: "astromech-xec1"
issue_type: "spike"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Runtime probe: Parakeet on burn (CPU, wgpu) and rten"
updated_at: "2026-11-07T05:34:14.998986555+00:00"
---

## Goal

Find out which Rust runtime can carry Parakeet: burn (CPU and wgpu, from fp32 or fp16 weights) or rten (pure-Rust CPU, today's int8 files), against ort as the reference.

## Done when

- A report under docs/evidence/<this id>/ gives, on this Linux box, for each runtime that loads the model: whether it loads (and the unsupported ops if not), transcript parity with ort on the eval clips, encoder time median and p95 on 1.3 s and 3 s clips, peak RSS, and the on-disk model size.
- It covers rten on today's int8 encoder, decoder and joiner, and says whether Microsoft's DynamicQuantizeLSTM is the only blocker if one fails.
- It names the Windows probe Sumeet would run for the GPU numbers (astromech-278z), or says why one isn't worth it.

## Context

- burn's ONNX importer lacks DynamicQuantizeLinear, ConvInteger and DynamicQuantizeLSTM (ARCHITECTURE.md §17.5 item d); parakeet-rs 0.4 runs burn on fp32 exports only.
- The box has no AMD GPU; wgpu here measures only that the path works, not its speed on an RX 9070 XT.
