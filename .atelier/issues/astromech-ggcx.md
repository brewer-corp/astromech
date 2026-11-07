---
created_at: "2026-11-07T05:34:14.816472616+00:00"
id: "astromech-ggcx"
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
title: "Parakeet Unified against TDT v2 on the synthetic voice eval"
updated_at: "2026-11-07T05:34:14.816472616+00:00"
---

## Goal

Decide whether to move speech-to-text to Parakeet Unified (nvidia/parakeet-unified-en-0.6b) by measuring it against today's TDT 0.6B v2 int8 on our own phrases, not the leaderboard.

## Done when

- A report under docs/evidence/<this id>/ gives, for TDT v2 (sherpa, today's baseline) and Unified (parakeet-rs on ort, CPU): execute-or-ask and wrong executes by family and condition from the `voice-eval` scoring, raw word error rate of the transcripts against the phrase text, and decode-time median and p95, on the same rendered clips.
- The report says which ONNX export of Unified was used and its licence, and whether the NVIDIA Open Model License allows shipping the weights the way we ship Parakeet today (first-run download from Hugging Face).
- A recommendation: switch, stay, or wait for real clips (astromech-cgrw).

## Scope

Whole-clip (offline) decoding only. Streaming while the button is held is a later question, noted in the report if the export supports it.

## Context

- The synthetic eval is near its ceiling for v2 (3123/3136), so command scoring may not separate the models; raw WER and the hard phrases in astromech-xi2i (notoriety, rebuy, handbrake off) are the more sensitive signals.
- Rendered audio is cached in results/voice-eval/audio (git-ignored); never commit audio or models.
