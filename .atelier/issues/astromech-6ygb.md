---
created_at: "2026-10-17T00:02:49.661084165+00:00"
id: "astromech-6ygb"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Lazy speaker: print lines dropped by barge-in, and test the session wiring"
updated_at: "2026-10-17T00:02:49.661084165+00:00"
---

## Goal

Two gaps in the LazySpeaker from astromech-mcdt: a push-to-talk press while Kokoro loads clears queued lines without printing them, and nothing tests the session's choice between starting now and later with the real find_tts check.

## Done when

Lines cleared by an interrupt during load are printed, with a test; a session test with a temp models folder (marker and model files) shows the lazy path is chosen when the model is missing and the direct path when present.

## Context

Deferred from the astromech-mcdt review (speech.rs:513, session.rs:200-208).
