---
created_at: "2026-10-15T20:21:44.862498011+00:00"
id: "astromech-q11q"
issue_type: "task"
labels:
- "size-l"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-8po8"
  - kind: "issue"
    id: "astromech-9tlx"
  - kind: "issue"
    id: "astromech-e26f"
  - kind: "issue"
    id: "astromech-ffap"
  - kind: "issue"
    id: "astromech-k2bs"
  - kind: "issue"
    id: "astromech-m3lo"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Journal history database and backfill"
updated_at: "2026-10-15T20:21:44.862498011+00:00"
---

## Goal

An `events` table, a resumable import of every `Journal.*.log`, and derived ledgers (visits, earnings, ships, scans).

## Done when

Importing a fixture set, killing it halfway and restarting gives the same ledgers as one clean import; a timing run on Sumeet's real logs is recorded as evidence.

## Context

Unblocks the logbook, "been here before", exploration history and session stats. Measure import time on years of logs.
