---
created_at: "2026-10-22T23:48:29.163774792+00:00"
id: "astromech-m3xd"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-m3xd
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-e8ux"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Voice queries for the exploration system view and biology"
updated_at: "2026-10-23T00:22:41.349960907+00:00"
---

## Goal

The strategy says a module is not done until its `voice_exposed` queries have phrase templates (ARCHITECTURE §18.7). §18.7 lists the Exploration & exobiology module (M7) voice tools as "Is this system worth mapping?" and "which organics haven't I scanned here?", but the grammar (crates/astromech-intent/src/lib.rs) has no exploration or exobiology query: on 22 Oct "is this system worth mapping", "which organics haven't I scanned here", "what is this system worth", "how much is my unsold data worth" and "what biology is in this system" all came back NotUnderstood through the core Assistant. The System and Biology pages and the biology and sampling callouts exist; only the on-demand voice questions are missing. Add them, answered from the same folds the pages use.

## Done when

- Phrase templates resolve for at least: system value now and if mapped, which bodies are worth mapping, the biology left to sample in this system (genera or species not yet scanned), and unsold data value; tests in astromech-intent show they do not steal existing phrases.
- Core tests answer each from a synthetic journal folder replay (`astromech_exploration::system`, `History::unsold`, the biology fold), with the spoken answer asserted, the way tests/colonisation_voice.rs does.
- The voice tester dry run says what each would do; the Exploration guide page lists the questions.

## Context

- §18.7 M7 row: docs/ARCHITECTURE.md. Strategy S8: docs/strategies/replace-the-toolbox.md.
- Pages: app/src/modules/exploration, app/src/modules/exobiology. Folds: crates/astromech-exploration, crates/astromech-exobiology.
- Pattern to follow: astromech-3ez9 (colonisation voice queries), astromech-8joi.
- Found by validation astromech-e8ux.

## Notes

- 2026-10-22 [note] Synthetic voice eval (scripts/voice-eval.sh, 4 Kokoro voices x 4 conditions, 3200 clips, at e6ba5919): execute-or-ask 3120/3136 (99%), wrong execute 0; exploration family 127/128 (one Miss: bm_george noise heard 'which organics of an eye scan here'). Eval set swaps 8 phrases (2 colonisation.progress, 2 colonisation.buy, 1 each find.service/module/ship, route.next) for 2 per exploration command to stay at 200. Audio and report not committed.
