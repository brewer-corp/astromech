---
created_at: "2026-10-23T00:28:58.350802420+00:00"
id: "astromech-54k4"
issue_type: "task"
labels:
- "size-s"
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
status: "todo"
title: "Exploration voice answers: near-miss phrases, one credits formatter, logged history errors"
updated_at: "2026-10-23T00:28:58.350802420+00:00"
---

## Goal

The exploration voice questions don't pull unrelated speech into did-you-mean prompts, every spoken answer says credits the same way, and a history failure on the unsold-data answer is logged with its cause.

## Done when

- 'what should I do / say / buy', 'what is in this system', 'what is here' and 'how much is my ship/cargo worth' get no exploration did-you-mean (drop or narrow 'what should I map'), with near-miss tests.
- One spoken credits formatter (in astromech-tools, with billions and trimming) is used by the exploration, session summary and discovery answers; '1.0 million credits' and '1460.0 million credits' no longer occur, with tests.
- crates/astromech-core/src/exploration.rs logs the history error before answering 'Journal history is unreadable right now.'

## Context

Review findings e34o, cg58 and 0vov on task/astromech-m3xd (crates/astromech-intent/src/lib.rs:385, crates/astromech-tools/src/exploration.rs:73, crates/astromech-core/src/exploration.rs:53).
