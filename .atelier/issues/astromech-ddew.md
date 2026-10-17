---
created_at: "2026-10-16T19:40:10.037540786+00:00"
id: "astromech-ddew"
issue_type: "validation"
labels:
- "needs-human"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Listen to the TTS name reading rules"
updated_at: "2026-10-16T19:40:10.037540786+00:00"
---

## Goal

The name reading rules (astromech-w8ai) choose TTS input text, but nobody has heard Kokoro/espeak read it: spelled sector letters (does a lone "A" read as the letter or the article "uh"?), "D 12 dash 39", the ore respellings (Grandidierite, Serendibite, Rhodplumsite, Musgravite, Benitoite), "Gleeza", "Collinder".

## Done when

Sumeet has listened to a rendered set of the table examples and each reads right, or the misreads are filed.

## Context

Render with `crates/astromech-voice/examples/render-voices.rs` or the speaking path. Found in review of astromech-w8ai (R1, R2).
