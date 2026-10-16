---
created_at: "2026-10-16T20:08:08.848475594+00:00"
id: "astromech-550b"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-lxma"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Stop the speech thread promptly when the speaker closes"
updated_at: "2026-10-16T20:08:08.848475594+00:00"
---

## Goal

`TtsSpeaker`'s thread checks its closed flag only once both queues are empty, so after a session ends it keeps rendering queued warm-ups (and any replies) and keeps the Kokoro worker loaded; a quick game restart can load a second Kokoro alongside it.

## Done when

On close the thread clears its warm-up queue and exits; a test with a slow fake synth shows no renders after close.

## Context

`crates/astromech-core/src/speech.rs`. Found in review of astromech-orel (round 4, N1).
