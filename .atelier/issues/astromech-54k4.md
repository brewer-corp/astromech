---
created_at: "2026-10-23T00:28:58.350802420+00:00"
id: "astromech-54k4"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-54k4
priority: "P3"
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
status: "in_progress"
title: "Exploration voice answers: near-miss phrases, one credits formatter, logged history errors"
updated_at: "2026-11-04T21:29:17.915464552+00:00"
---

## Goal

The exploration voice questions don't pull unrelated speech into did-you-mean prompts, every spoken answer says credits the same way, and a history failure on the unsold-data answer is logged with its cause.

## Done when

- 'what should I do / say / buy', 'what is in this system', 'what is here' and 'how much is my ship/cargo worth' get no exploration did-you-mean (drop or narrow 'what should I map'), with near-miss tests.
- One spoken credits formatter (in astromech-tools, with billions and trimming) is used by the exploration, session summary and discovery answers; '1.0 million credits' and '1460.0 million credits' no longer occur, with tests.
- crates/astromech-core/src/exploration.rs logs the history error before answering 'Journal history is unreadable right now.'

## Context

Review findings e34o, cg58 and 0vov on task/astromech-m3xd (crates/astromech-intent/src/lib.rs:385, crates/astromech-tools/src/exploration.rs:73, crates/astromech-core/src/exploration.rs:53).

## Notes

- 2026-11-04 [decision] The shared spoken credits formatter is astromech_announce::credits (credits() and amount(), the number without the word), not astromech-tools; session_summary::credits and discovery::credits are removed and the exploration answers, lookups, session summary and discovery callout call it.
  Reason: astromech-tools depends on astromech-announce (scripts/check-deps.py), so a formatter in tools could not be used by the session summary and discovery callout. Discovery keeps its sentence shape ('4.4 million mapped') through amount().
- 2026-11-04 [decision] Dropped the 'what should i map' phrase and added astromech_intent::EVERYDAY, a short list of everyday questions ('what is in this system', 'what is here', 'how much is my ship/cargo worth'...) that Grammar::mvp scores as a rival: a transcript scoring at least EXECUTE against one, and no better against a command, is NoMatch; it also counts against every command's margin.
  Reason: Dropping 'what should i map' fixed 'what should I do/say/buy', but 'what is in this system' and 'how much is my cargo worth' are one word from phrases kept on purpose ('what biology is in this system' was asked in the 22 Oct validation; 'how much is my data worth'), and the edit-distance scorer has no notion of a key word, so no phrase edit could push them under ASK. The EXECUTE floor keeps loose speech near an everyday phrase ('what is left' beside 'what is here') scoring as before.
