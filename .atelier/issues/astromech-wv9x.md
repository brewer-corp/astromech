---
created_at: "2026-10-17T22:48:57.973108968+00:00"
id: "astromech-wv9x"
issue_type: "task"
labels:
- "alpha"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Mining says the core is not running when it runs but no session has a run"
updated_at: "2026-10-22T21:25:56.027746020+00:00"
---

## Goal

The Mining page tells a running core with no session apart from a core that is not running.

## Done when

- With the core up and no session (status.mining null) the page says "No mining run yet." and nothing about the core not running.
- With the core not answering (core_status null) the page says "Could not read the mining run: the core did not answer."
- loadMining rejects when coreStatus is null and is null when status.mining is null (lib/mining.test.ts); a production-path page test (lib/ui/staleTauri.test.ts) covers both texts.

## Context

Found walking astromech-x81p; reviewer confirmed against crates/astromech-core/src/app.rs: the core sends mining only when a session exists, so with the core up and no session the page said 'The core isn't running, so there is no mining run to show.'

The earlier Done when (loadMining null only when coreStatus is null) was overtaken by astromech-spxx, which made loadMining reject when the core does not answer and changed the empty text (5be1fd9d). This record now asks for the page-level proof of both cases.
