---
created_at: "2026-10-22T23:48:36.936226438+00:00"
id: "astromech-j75v"
issue_type: "task"
labels: []
priority: "P2"
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
status: "todo"
title: "Voice lookups: a named system ('near Sol') turns into a different commodity or module"
updated_at: "2026-10-22T23:48:36.936226438+00:00"
---

## Goal

Voice lookups search only from the current system. Saying a system as well is not understood and is misread as a slot value: on 22 Oct through the core Assistant, "where can I sell gold near Sol" asked "where can i sell Polymers" and "where is the nearest 5A frame shift drive near Sol" asked "where is the nearest Frame Shift Drive (SCO)". Either answer from the named system (the grammar already has a `{system}` slot for plot_route) or recognise the trailing "near <system>" and say lookups by voice are from where you are, instead of offering a wrong commodity or module.

## Done when

- Tests in astromech-intent/astromech-core: "where can I sell gold near Sol" and "nearest 5A frame shift drive near Sol" no longer resolve or ask about another commodity or module; they either search from Sol or say voice searches from the current system.

## Context

- Strategy preamble: "Every lookup works from the current system or a typed one, in the app and as a voice query" (docs/strategies/replace-the-toolbox.md). The app's search kit takes a typed origin; voice does not.
- Grammar: crates/astromech-intent/src/lib.rs; answers: crates/astromech-core/src/lookups.rs.
- Found by validation astromech-e8ux.

## Unknowns

- Whether the preamble means voice must search from a spoken system too (Sumeet). The validation read "a typed one" as the app's typed origin.
