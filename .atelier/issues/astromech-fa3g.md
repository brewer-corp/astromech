---
created_at: "2026-10-16T18:12:14.981858462+00:00"
id: "astromech-fa3g"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-snqt"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-12uv"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:55:10.516884478+00:00"
status: "done"
title: "Voice command tester next to Diagnostics"
updated_at: "2026-10-16T18:55:10.516884478+00:00"
---

## Goal

A page next to Diagnostics where the user can try voice commands safely. They speak through push-to-talk or type a phrase, and see the transcript, the matched intent and its confidence, and the keys it would send. Nothing is sent to the game unless they ask for a live run.

## Done when

Typing a known phrase shows its intent and key sequence with nothing pressed (tested). Speaking with push-to-talk shows the transcript and intent, checked once with a recorded clip.

## Context

`crates/astromech-intent`, the dry-run path (`--dry-run`), and the `save_clips`/`--eval` harness.

## Close Reason

The Voice tester page (Settings, next to Diagnostics) dry-runs a typed phrase or a push-to-talk transcript and shows the transcript, intent, confidence, reply and keys; Run live is a separate action through the policy. Typed dry run tested with a recording fake executor (no key events); a synthetic recorded clip went through the real voice worker and Parakeet into the test-mode path (Linux). Real microphone/PTT in the Windows app is split out to astromech-1b43 (needs-human).
