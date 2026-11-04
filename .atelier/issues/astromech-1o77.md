---
created_at: "2026-11-04T23:37:08.515637015+00:00"
id: "astromech-1o77"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-1o77
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-9fwe"
  - kind: "issue"
    id: "astromech-u6ai"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:43:56.130294604+00:00"
status: "done"
title: "Toolbox review follow-ups, second November batch"
updated_at: "2026-11-04T23:43:56.130294604+00:00"
---

## Goal

Land the last two small review follow-ups from 2026-11-04 on one branch with one independent review.

## Done when

- Every child is done with its own proof, and one independent review of the branch is approved on this parent.

## Context

Batched per agent-factory orchestrate. Children keep their own Done when.

## Close Reason

Both children done on task/astromech-1o77: astromech-9fwe (9b5943b6: quantity words after 'in' are no system name; unknown-system line capitalises the heard name; guide and ARCHITECTURE updated) and astromech-u6ai (157996b4: tests pin no refusal on a failed refused batch, poisoned drain at 0, 49, 50, 100, 199; no bug). Review of both outstanding.
