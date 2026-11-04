---
created_at: "2026-11-04T20:43:35.986453459+00:00"
id: "astromech-7c7c"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-7c7c
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-9kp2"
  - kind: "issue"
    id: "astromech-aznm"
  - kind: "issue"
    id: "astromech-ksb3"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Toolbox review follow-ups, November batch"
updated_at: "2026-11-04T21:05:07.259267126+00:00"
---

## Goal

Land the low-priority review follow-ups for the toolbox mission on one branch with one independent review, instead of a branch and review each.

## Done when

- Every child is done with its own proof, and one independent review of the branch is approved on this parent.

## Context

Batched on 2026-11-04 per agent-factory orchestrate (low-priority follow-ups share a parent). Children keep their own Done when.
