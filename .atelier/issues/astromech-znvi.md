---
created_at: "2026-10-17T17:29:23.461289517+00:00"
id: "astromech-znvi"
issue_type: "task"
labels:
- "alpha"
- "docs-found"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-znvi
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:25:59.898354350+00:00"
status: "done"
title: "Learned voice phrases cannot be reviewed or removed"
updated_at: "2026-10-17T19:25:59.898354350+00:00"
---

## Goal

Answering yes to "Did you mean ...?" stores the heard phrase as a personal alias (astromech-store), which then executes at once. There is no page to list, edit or delete them, and a wrong yes teaches a wrong alias for good short of deleting the database.

## Done when

Settings (or the Voice tester) lists the learned phrases with a way to remove each. The guide (docs/guide/voice.md) says there is no such page; update it. Found while writing the user guide.

## Close Reason

Done-when met: Learned phrases listed with Remove in the Voice tester; PR 130; see evidence
