---
created_at: "2026-10-15T20:36:13.651536478+00:00"
id: "astromech-aonb"
issue_type: "task"
labels:
- "size-l"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-2myu"
  - kind: "issue"
    id: "astromech-bg0x"
  - kind: "issue"
    id: "astromech-ce4s"
  - kind: "issue"
    id: "astromech-fxp0"
  - kind: "issue"
    id: "astromech-j2mb"
  - kind: "issue"
    id: "astromech-qows"
  - kind: "issue"
    id: "astromech-tqdh"
  - kind: "issue"
    id: "astromech-xxyp"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Journal history database and backfill"
updated_at: "2026-10-16T19:32:23.008325494+00:00"
---

## Goal

An `events` table, a resumable import of every `Journal.*.log`, and derived ledgers (visits, earnings, ships, scans).

## Done when

Importing a fixture set, killing it halfway and restarting gives the same ledgers as one clean import, and an import-time measurement on a large synthetic log set is recorded. The timing run on Sumeet's real logs is astromech-tqdh.

## Context

Unblocks the logbook, "been here before", exploration history and session stats. Measure import time on years of logs.
