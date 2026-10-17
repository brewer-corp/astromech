---
created_at: "2026-10-17T00:11:18.156254244+00:00"
id: "astromech-8ozh"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-8ozh
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T04:47:38.895463828+00:00"
status: "done"
title: "Outbox row ids are reused after a purge, so the claim in deliver_due can match a new row"
updated_at: "2026-10-17T04:47:38.895463828+00:00"
---

## Goal

The outbox `id` is a plain INTEGER PRIMARY KEY, so after discard_all the next enqueue reuses ids. A pass that already read due rows can then claim a new row with a purged row's id (UPDATE ... WHERE id) and post the purged body. Use AUTOINCREMENT (migration) or claim with a created/attempts match.

## Done when

A test enqueues new messages mid-pass after a purge and the purged bodies are not posted.

## Close Reason

Outbox ids are AUTOINCREMENT via a v2 migration that keeps rows; test of a purge and requeue mid-pass shows purged bodies are not posted
