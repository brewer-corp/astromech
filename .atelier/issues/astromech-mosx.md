---
created_at: "2026-10-17T17:35:54.889632866+00:00"
id: "astromech-mosx"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-mosx
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Pipeline: read the journal on elapsed time, not only after 250 ms of silence"
updated_at: "2026-10-17T18:40:12.158546668+00:00"
---

## Goal

Tailer::on_idle only runs when the notification channel is quiet for 250 ms, so a steady stream of other events (companion files, unrelated files, watcher errors) can starve the journal read. Read when enough time has passed since the last journal read, whatever woke the loop.

## Done when

A test sends a continuous stream of non-journal notifications and still sees an appended journal line applied within 1 s.

## Context

Found in review of the astromech-mh3m PR (finding M4).
