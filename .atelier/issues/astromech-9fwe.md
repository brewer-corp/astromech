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
closed_at: "2026-11-04T23:41:41.409877084+00:00"
status: "done"
title: "Voice lookups: 'in bulk' is not a system name"
updated_at: "2026-11-04T23:41:41.409877084+00:00"
---

## Goal

An ordinary phrase after 'in' on a voice lookup ('in bulk', 'in large quantities') is not read as a system name.

## Done when

'where can I sell gold in bulk' and 'in large quantities' search from the current system (or are treated as no origin), and the unknown-system line capitalises the name it heard; with tests (R9, astromech-wcp0).

## Context

Deferred review finding R9 (astromech-wcp0) on task/astromech-1n9b: crates/astromech-intent/src/lib.rs:504. Today it answers 'bulk. Spansh does not know that system.' and costs one Spansh request; it never searches from a wrong system.

## Close Reason

Done at 9b5943b6. split_origin: 'bulk', 'large', 'quantity', 'quantities' added to NOT_A_NAME (first words of a place that names no system), so 'where can I sell gold in bulk', 'in large quantities' and 'in quantity' search from the current system; a system whose name starts with one of these words can no longer be named after near/from/in/around (none known). lookup::unknown_system uppercases the first character of the heard name, rest as heard ('nowhere prime' -> 'Nowhere prime.'). Tests: intent case list extended (failed before the fix: origin Some("bulk")); new tools test an_unknown_system_line_starts_with_a_capital (not watched failing; old code returns the name unchanged). Docs: guide lookups.md 'From another system' adds 'in bulk'; ARCHITECTURE search-origin paragraph updated. check-guide.py, fmt, clippy -D warnings clean. Other quantity phrasings ('in big amounts') still read as a system name and get the plain unknown-system line. Review outstanding (orchestrator).
