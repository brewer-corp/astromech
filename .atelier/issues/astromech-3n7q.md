---
created_at: "2026-11-04T23:51:08.425564470+00:00"
id: "astromech-3n7q"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-3n7q
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
title: "Voice origins: real systems beside NOT_A_NAME, and lowercase procedural names"
updated_at: "2026-11-05T00:21:54.718975732+00:00"
---

## Goal

A spoken origin place is read as a system only when it is one, and procedural names read right whatever case speech recognition returns.

## Done when

- A place containing 'bulk', 'quantity' or 'quantities' anywhere ('in small quantities') is no system name, with a test (R1, astromech-ud7x).
- A place is checked against the known-system slot values before NOT_A_NAME applies, so 'near Any Na', 'A Bootis' and 'MY Apodis' search from those systems, with a test (R2, astromech-hn1c).
- The TTS reading rules handle a lowercase procedural name ('col 285 sector ab-c d14-123', 'hip 12345'), checked against real Parakeet transcripts first (R3, astromech-7b7k).

## Context

Deferred review findings on task/astromech-1o77: crates/astromech-intent/src/lib.rs:495 and :535, crates/astromech-tools/src/lookup.rs:24, crates/astromech-voice/src/speakable.rs:31 and :216-223.
