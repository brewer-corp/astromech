---
created_at: "2026-10-16T18:54:29.567403634+00:00"
id: "astromech-mcdt"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-mcdt
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-a50f"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T00:00:02.777671921+00:00"
status: "done"
title: "Start speech output when the voice model finishes downloading"
updated_at: "2026-10-17T00:00:02.777671921+00:00"
---

## Goal

When the background speech-model download finishes, speech output starts in the running session, so a user who opens the game before Kokoro v1.0 (about 350 MB) has downloaded is not silent for the whole session.

## Done when

- With the game running and the TTS model missing, finishing the download makes the next reply spoken without restarting astromech or the game (restart the speaker after the download, or start it lazily on first use).
- A test or scripted run shows it.

## Context

Found in review of astromech-a50f: `app::download_models` logs "speech models ready" but nothing restarts the session, and `Session` builds its `TtsSpeaker` once at start (falling back to `ConsoleSpeaker` when `models::find_tts` fails). The v0.19 to v1.0 update starts this download automatically on startup.

## Close Reason

Session wraps TTS in LazySpeaker when the Kokoro model is missing at start; the first line said after the download completes starts Kokoro and is spoken, no restart. Proven by speech::tests::lazy_speaker_starts_once_the_model_is_ready (evidence astromech-za1x, astromech-y4d3).
