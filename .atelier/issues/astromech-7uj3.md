---
created_at: "2026-10-17T17:40:28.295541245+00:00"
id: "astromech-7uj3"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-7uj3
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T18:48:01.367493678+00:00"
status: "done"
title: "Pipeline: a journal reopened this run replays only up to its previous read point"
updated_at: "2026-10-17T18:48:01.367493678+00:00"
---

## Goal

When the pipeline reopens a journal it already read this run (it became newest again after a backup or sync tool touched an older journal), it replays only up to where it had read before and treats later lines as live.

## Done when

After an older journal is touched and the game then writes the current journal again, the first new line is handled as live (EDDN, callouts, the route follower), with a pipeline test.

## Context

Review finding astromech-76q0 on PR #112 (astromech-1135). Today the whole reopened journal is a replay, so the next real jump after such a touch is silent: it affects EDDN and every callout, not only the route follower.

## Close Reason

A reopened journal replays up to its last committed read point and later lines are live. Test a_reopened_journal_replays_only_what_was_read_before failed before; left-twice test added. Stacked on astromech-p4um (JournalTail::read_to). Deferred: astromech-bh0c (companion triggers in the replay half). Windows unverified.
