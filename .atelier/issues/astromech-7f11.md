---
created_at: "2026-11-07T05:34:15.373831421+00:00"
id: "astromech-7f11"
issue_type: "feature"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ggcx"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xec1"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Speech-to-text on parakeet-rs, with hotword biasing"
updated_at: "2026-11-07T05:34:15.373831421+00:00"
---

## Goal

Move speech-to-text off sherpa onto parakeet-rs with the model and runtime the two spikes choose, and own the decoding loop so we can bias it towards Elite names (the hotword feature sherpa blocks, V1-DESIGN.md §6.3 item 6).

## Done when

- The worker's STT runs through parakeet-rs; `scripts/voice-eval.sh` scores at least the sherpa baseline.
- Decoding takes a hotword list and the eval shows it helps the names it targets without new wrong executes.

## Unknowns

Which model and runtime: settled by the Unified and runtime spikes.
