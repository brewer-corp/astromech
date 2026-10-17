---
created_at: "2026-10-17T07:38:18.657274444+00:00"
id: "astromech-8joi"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-8joi
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
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T08:01:28.724360873+00:00"
status: "done"
title: "Voice query for nearby Powerplay targets"
updated_at: "2026-10-17T08:01:28.724360873+00:00"
---

## Goal

The commander can ask by voice for nearby systems for their power to work in (acquire, reinforce, undermine), e.g. "where can I undermine near me", "nearest system to reinforce". The lookup exists in the app (Powerplay > Where to work, shell command `powerplay_targets`, astromech-7ix7) but astromech-7ix7 notes it has no voice query, so S4 of the toolbox strategy ("in the app and as a voice query"; ARCHITECTURE §18.7) is not met.

## Done when

Phrase templates per target kind, dispatched to the same Spansh search with the remembered filters (`powerplay.<kind>`) and the pledged power from the journal; a spoken answer with system, distance and state; an unpledged commander hears that plainly. Tests: phrase-to-command tests and an answer test against recorded Spansh responses. Independent review.

## Context

Found by the S1-S8 validation (astromech-e8ux).

## Close Reason

Powerplay targets (acquire, reinforce, undermine) answer by voice via Galaxy::powerplay_targets for the journal's pledged power with the remembered powerplay.<kind> filters; unpledged commanders are told so; gates green at 63a82c4. Independent review not yet done.
