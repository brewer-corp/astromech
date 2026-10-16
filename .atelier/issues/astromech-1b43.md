---
created_at: "2026-10-16T18:55:05.673876676+00:00"
id: "astromech-1b43"
issue_type: "validation"
labels:
- "needs-human"
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Voice tester with real push-to-talk on Windows"
updated_at: "2026-10-16T18:55:05.673876676+00:00"
---

## Goal

Check the Voice tester page (Settings → Voice tester) with the real microphone and push-to-talk in the Windows app, which the Linux checks for astromech-fa3g could not reach.

## Done when

With Elite running and the page open: holding push-to-talk and saying "gear down" shows the transcript, the landing_gear.down intent and its key within a couple of seconds, and the gear does not move. Clicking Run live then lowers it. After leaving the page, push-to-talk lowers or raises the gear again as normal within 3 s. Each result is recorded as evidence.

## Context

astromech-fa3g built the page. Its dry run, the test-mode routing and a recorded clip through the voice worker and Parakeet were checked on Linux; the Windows microphone capture, the PTT thread change in session.rs and the shell's core_call change were not compiled or run there (Windows CI builds them).
