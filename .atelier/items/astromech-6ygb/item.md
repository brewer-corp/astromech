---
schema: atelier.item
schema_version: 3
id: astromech-6ygb
title: 'Lazy speaker: print lines dropped by barge-in, and test the session wiring'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T00:02:49.661084165Z
---

## Goal

Two gaps in the LazySpeaker from astromech-mcdt: a push-to-talk press while Kokoro loads clears queued lines without printing them, and nothing tests the session's choice between starting now and later with the real find_tts check.

## Done when

- [ ] Lines cleared by an interrupt during load are printed, with a test; a session test with a temp models folder (marker and model files) shows the lazy path is chosen when the model is missing and the direct path when present.

## Context

Deferred from the astromech-mcdt review (speech.rs:513, session.rs:200-208).
