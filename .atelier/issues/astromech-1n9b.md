---
created_at: "2026-11-04T21:25:48.498950217+00:00"
id: "astromech-1n9b"
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
    id: "astromech-c4nj"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Voice lookups from a named system: more phrasings, colonisation, copy checks"
updated_at: "2026-11-04T21:25:48.498950217+00:00"
---

## Goal

A spoken origin system is understood in more phrasings and checked against the copy rules, and colonisation buy questions can take one too.

## Done when

- 'in <system>', 'around <system>' and 'near <system> system' search from that system (or say plainly what was heard), and non-system words after 'from' or 'near' ('from a station') are not read as a system name; with tests (R4, astromech-c4nj).
- Colonisation 'where can I buy <commodity>' accepts a named system, or the strategy's 'every lookup' is narrowed with Sumeet (R5, astromech-1sl7).
- crates/astromech-core/tests/copy.rs runs the named-system wording ('From X.', the unknown-system line) through the §6.5 checks (R6, astromech-53q1).

## Context

Deferred review findings R4–R6 on task/astromech-j75v: crates/astromech-intent/src/lib.rs:472 and :462, crates/astromech-core/tests/copy.rs:696.
