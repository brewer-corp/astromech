---
created_at: "2026-10-18T00:18:06.352011755+00:00"
id: "astromech-1c66"
issue_type: "bug"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-1c66
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-y296"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "A phrase confirmed once after a near-miss question must not become a personal phrase for a key-pressing command"
updated_at: "2026-10-18T00:33:42.934543319+00:00"
---

## Goal

Answering yes to a 'Did you mean ...?' question saves the heard phrase as a personal phrase that executes next time. Since astromech-y296 and astromech-8fey, near-misses made of everyday words ('I'll stop', 'Pays') deliberately ask instead of executing; one yes would then make 'I'll stop' in conversation zero the throttle. A phrase listed as ask-only (the near-miss table in crates/astromech-intent) must never be learned as an executing personal phrase, and learning for T1 key-pressing commands should need more than one confirmation, or not happen for phrases made only of common words.

## Done when

Tests show: after answering yes to 'I'll stop' -> all stop, saying 'I'll stop' again asks again (does not execute); the same for 'Pays' -> paste; learning still works for a genuine personal phrase for a non-ask-only mishearing; the voice-tester's learned-phrase list (astromech-znvi) shows nothing for the ask-only cases. docs/guide/voice.md describes the rule.
