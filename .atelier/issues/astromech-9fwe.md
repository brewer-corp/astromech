---
created_at: "2026-11-04T23:27:02.851266362+00:00"
id: "astromech-9fwe"
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
title: "Voice lookups: 'in bulk' is not a system name"
updated_at: "2026-11-04T23:37:08.975758106+00:00"
---

## Goal

An ordinary phrase after 'in' on a voice lookup ('in bulk', 'in large quantities') is not read as a system name.

## Done when

'where can I sell gold in bulk' and 'in large quantities' search from the current system (or are treated as no origin), and the unknown-system line capitalises the name it heard; with tests (R9, astromech-wcp0).

## Context

Deferred review finding R9 (astromech-wcp0) on task/astromech-1n9b: crates/astromech-intent/src/lib.rs:504. Today it answers 'bulk. Spansh does not know that system.' and costs one Spansh request; it never searches from a wrong system.
