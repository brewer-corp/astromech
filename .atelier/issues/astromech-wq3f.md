---
created_at: "2026-10-15T20:36:13.663651636+00:00"
id: "astromech-wq3f"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-6920"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:20:04.261165293+00:00"
status: "done"
title: "Callout copy guide and pass"
updated_at: "2026-10-16T20:20:04.261165293+00:00"
---

## Goal

Write the copy guide (official, short, present tense, no contractions, no jokes, little "I") and apply it to every callout, with a test that no callout runs over its word budget.

## Done when

The word-budget test covers every reaction's lines and passes.

## Context

Done for the gear and status lines only (partly built).

## Notes

- 2026-10-16 [note] Guide in V1-DESIGN §6.5; test crates/astromech-core/tests/copy.rs. Help lines (setup problems, e.g. the 'I can't switch X' binding guidance and 'isn't available yet') are outside the budget by the guide and were not rewritten; they still contain contractions and 'I'.

## Close Reason

The word-budget test covers every reaction's lines (callout rules, fixed phrases, replies, status answers) and passes.
