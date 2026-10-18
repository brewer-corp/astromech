---
created_at: "2026-10-18T04:05:42.982366386+00:00"
id: "astromech-clju"
issue_type: "task"
labels:
- "post-alpha"
- "voice"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-vkzj"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Voice: let the recogniser use the grammar (n-best rescoring / constrained decoding) instead of matching 1-best text"
updated_at: "2026-10-18T04:05:42.982366386+00:00"
---

## Goal

Let the speech recogniser use the command grammar instead of choosing from one best-guess transcript. Today the matcher fuzzy-matches Parakeet's 1-best text after the fact and has no real confidence signal. Use the decoder's own acoustic evidence to choose the command and to decide between execute, ask and reject.

## Done when

- Command choice uses recogniser evidence: n-best hypotheses from modified beam search rescored against the grammar, and/or a grammar-constrained decoding pass compared with the free pass to give a confidence score.
- Execute/ask/reject thresholds are set from that confidence, not from spelling similarity.
- The synthetic voice eval (`scripts/voice-eval.sh`) keeps 0 wrong executes and 64/64 rejects, and fewer clips need to ask than with the astromech-vkzj rule.
- A fixed sample of the vkzj dictionary-swap probe inputs never executes a key press.
- Latency from release to action start does not regress beyond the measured budget.

## Context

Decided 2026-10-18 by Sumeet during astromech-vkzj. vkzj's alpha fix stops sound-alike (phonetic-key) matches from ever executing. This is the structural follow-up: the matcher was patching around information the recogniser threw away. Prior art: VoiceAttack (Windows speech grammars) and Vosk grammar mode. astromech-y296 already ran Parakeet TDT with modified_beam_search and hotwords; see docs/evidence/astromech-y296/.

## Unknowns

- Whether sherpa-onnx exposes n-best lists or per-hypothesis scores for Parakeet TDT, or whether constrained decoding needs our own beam search over the transducer.
- How to handle open slots (system, station and commodity names): class word lists, a two-pass approach, or leaving slots to text matching.
- Whether the cost of a second decode pass fits the latency budget on the target hardware.
