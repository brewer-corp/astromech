---
created_at: "2026-10-15T20:30:24.601030274+00:00"
id: "astromech-hrag"
issue_type: "task"
labels:
- "size-l"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-2o5q"
  - kind: "issue"
    id: "astromech-aown"
  - kind: "issue"
    id: "astromech-av6w"
  - kind: "issue"
    id: "astromech-da98"
  - kind: "issue"
    id: "astromech-iwf3"
  - kind: "issue"
    id: "astromech-qkbv"
  - kind: "issue"
    id: "astromech-xs43"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Journal history database and backfill"
updated_at: "2026-10-15T20:30:24.601030274+00:00"
---

## Goal

An `events` table, a resumable import of every `Journal.*.log`, and derived ledgers (visits, earnings, ships, scans).

## Done when

Importing a fixture set, killing it halfway and restarting gives the same ledgers as one clean import; a timing run on Sumeet's real logs is recorded as evidence.

## Context

Unblocks the logbook, "been here before", exploration history and session stats. Measure import time on years of logs.
