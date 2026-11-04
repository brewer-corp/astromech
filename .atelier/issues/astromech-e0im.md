---
created_at: "2026-11-04T21:48:48.614549177+00:00"
id: "astromech-e0im"
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
  - kind: "review"
    id: "astromech-mbgc"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Voice grammar: pin the everyday-question margin and 'how much is this worth'"
updated_at: "2026-11-04T21:48:48.614549177+00:00"
---

## Goal

The everyday-question rule in the voice grammar makes only the choices it means to, pinned by tests.

## Done when

- Whether EVERYDAY counts against every command's margin is decided: either drop the margin term or keep it, and a test pins near misses such as 'how much is my unsold cargo worth' (R2, astromech-mbgc).
- What 'how much is this worth' and 'what is this worth' should do (ask about system value, as before astromech-54k4, or 'Say again') is decided and pinned in a test (R3, astromech-eony).

## Context

Deferred review findings on task/astromech-54k4: crates/astromech-intent/src/lib.rs:972 and :467-470 with the filler words at :510. Read-only answers, so the cost is an extra 'yes' or a 'Say again', never a wrong action.

## Unknowns

- R3 is a judgement on what a player most likely means by 'how much is this worth' in a system (Sumeet).
