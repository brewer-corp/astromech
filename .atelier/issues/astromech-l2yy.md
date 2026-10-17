---
created_at: "2026-10-17T19:25:24.465705565+00:00"
id: "astromech-l2yy"
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
title: "Learned phrases: surface forget errors, refresh the list, update the control doc"
updated_at: "2026-10-17T19:25:24.465705565+00:00"
---

## Goal

Tidy three small leftovers of the learned-phrases list.

## Done when

(1) Assistant::forget returns the store error so the Voice tester shows it instead of an unchanged list. (2) The list refreshes when a phrase is learned while the page is open (for example on the page's poll). (3) The Request doc comment in crates/astromech-core/src/control.rs lists learned_phrases and learned_forget.

## Context

Review findings R1, R2 and the stale comment on PR #130 (astromech-znvi), all low.
