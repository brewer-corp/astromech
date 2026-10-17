---
created_at: "2026-10-17T05:13:31.406734979+00:00"
id: "astromech-wzol"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-wzol
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
closed_at: "2026-10-17T05:25:35.069687129+00:00"
status: "done"
title: "Session summary voice query: say when the history couldn't be read"
updated_at: "2026-10-17T05:25:35.069687129+00:00"
---

## Goal

The session summary voice query (astromech-oo8y) says "No session history yet." whenever the read fails, including a lost lock or a busy database, which is wrong when the history exists but was briefly busy. Say the history couldn't be read, or retry briefly on the voice path.

## Done when

A test with a locked history database gets a can't-read answer (or the retried answer), not "No session history yet."

## Context

Review finding astromech-no3f on astromech-oo8y.

## Close Reason

Voice query says the history is unreadable on a lost lock, busy database or read error; tests added
