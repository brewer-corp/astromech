---
created_at: "2026-10-16T18:20:15.335077037+00:00"
id: "astromech-a50f"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-a50f
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-gl7q"
  - kind: "issue"
    id: "astromech-orel"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-0474"
    type: "related"
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Switch text-to-speech to Kokoro v1.0"
updated_at: "2026-10-16T18:27:57.077247515+00:00"
---

## Goal

Replace Kokoro v0.19 (11 voices, 4 British) with Kokoro v1.0 from sherpa-onnx (`kokoro-multi-lang-v1_0`, about 53 voices, about 8 British, generally better quality), so the Settings voice picker offers more and better voices.

## Done when

- The model catalog downloads v1.0 (and an int8 version if one exists), with pinned SHA-256s, and the worker loads it.
- The real TTS path renders the same line with every British speaker, attached as evidence.
- The download size and the speaker list (index to name) are recorded in the catalog.
- The config stores the voice by name, not by numeric ID. A test shows an existing `speaker_id` config migrates to the matching v1.0 voice name, or to the default if there is no match.
- Users with v0.19 installed get v1.0 on update, and the old model folder is removed after the switch.

## Scope

Voice blending is out of scope.

## Context

`crates/astromech-voice/src/catalog.rs` (`TTS_KOKORO_EN*`), `models.rs`, `crates/astromech-core/src/config.rs` (`speaker_id`) and `speech.rs`. The speech cache key includes the speaker, so add the model version to it, or clear the cache on the switch. Unverified: the package size, and whether the bundled Chinese lexicon matters. Decided by Sumeet on 2026-10-16 (astromech-rnwk).
