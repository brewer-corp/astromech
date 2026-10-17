---
created_at: "2026-10-17T16:54:23.622988405+00:00"
id: "astromech-yin0"
issue_type: "task"
labels:
- "alpha"
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Replay real journals through every module"
updated_at: "2026-10-17T17:01:43.101183343+00:00"
---

## Goal

Run Sumeet's real journal history (years of events the fixtures don't cover) through the core and every module, and fix crashes, panics, parse failures and implausible results.

## Done when

A replay of the full folder runs clean (no panics, every unknown event counted and reviewed), each fixed problem has a fixture test, and the import time is recorded on astromech-tqdh.

## Context

Needs a copy of Sumeet's `Saved Games\Frontier Developments\Elite Dangerous` folder on the dev machine, kept outside the repo (it holds his commander name and history); only small, anonymised excerpts become fixtures. Also gives astromech-tqdh its import timing.
