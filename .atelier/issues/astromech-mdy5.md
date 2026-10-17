---
created_at: "2026-10-17T18:57:48.943731490+00:00"
id: "astromech-mdy5"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Copy test backstop: handle constants, other crates and raw literals"
updated_at: "2026-10-17T18:57:48.943731490+00:00"
---

## Goal

Make the copy test's backstop (tests/copy.rs, say_literals) sturdier, or replace it with a typed registry of fixed spoken lines.

## Done when

Constants and format! values passed to say(), lines in other crates' sources, raw strings and char literals are handled, or the lines live in one registry the test enumerates.

## Context

Review finding on PR #120 (astromech-yerd): the scan reads only astromech-core/src, sees string literals inside say(...) only, cuts a file at the first cfg(test), and does not parse char or raw literals.
