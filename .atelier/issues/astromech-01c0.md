---
created_at: "2026-10-17T22:21:16.128540383+00:00"
id: "astromech-01c0"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Pending Market retry uses the current live flag, so it can fire a stale callout for a replayed dock"
updated_at: "2026-10-22T20:57:08.221213686+00:00"
---

## Goal

retry_pending (crates/astromech-core/src/pipeline.rs) retries a pending companion trigger with the Tailer's current live flag, not the flag of the batch that made the trigger. A Market trigger from a replayed half whose Market.json was missing or stale goes pending; after the live half runs, a later Market.json read is handled live and can fire a callout for the replayed (old) dock.

## Done when

A trigger remembers whether it came from a live batch and is retried with that; a test shows a replayed Market trigger retried after the live half stays silent, and a live one still speaks.

## Context

Review finding on the astromech-bh0c PR.
