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
updated_at: "2026-10-18T01:00:06.408193405+00:00"
---

## Goal

Answering yes to a 'Did you mean ...?' question saves the heard phrase as a personal phrase that executes next time. Since astromech-y296 and astromech-8fey, near-misses made of everyday words ('I'll stop', 'Pays') deliberately ask instead of executing; one yes would then make 'I'll stop' in conversation zero the throttle. A phrase listed as ask-only (the near-miss table in crates/astromech-intent) must never be learned as an executing personal phrase, and learning for T1 key-pressing commands should need more than one confirmation, or not happen for phrases made only of common words.

## Done when

Tests show: after answering yes to 'I'll stop' -> all stop, saying 'I'll stop' again asks again (does not execute); the same for 'Pays' -> paste; learning still works for a genuine personal phrase for a non-ask-only mishearing; the voice-tester's learned-phrase list (astromech-znvi) shows nothing for the ask-only cases. docs/guide/voice.md describes the rule.

## Notes

- 2026-10-17 [note] Decision (review R5): T1 key-pressing phrases need two yes answers, rather than never learning phrases made only of common words. Why: (1) 'common word' needs a word-frequency list the app does not have, and any cut-off would be arbitrary for game speech (most mishearings are common words: 'Dear dawn', 'Clear down'), so it would either block almost all genuine learning or let through what it should stop; (2) a second, separate yes to the same heard phrase is evidence the mishearing recurs for this commander, which is what a personal phrase is for; (3) the known everyday-word near-misses are already handled exactly by the ask-only table, which is never learned, and since review R1 an ask-only transcript also stays capped against learned neighbours. Spoken answers (status, lookups) still learn on one yes: they press nothing. Product consequence: 'You're down' (the original real-game example) is no longer learned; it asks every time.
