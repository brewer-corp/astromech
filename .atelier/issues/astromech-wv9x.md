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
status: "todo"
title: "Mining says the core is not running when it runs but no session has a run"
updated_at: "2026-10-17T22:48:57.973108968+00:00"
---

## Goal

The Mining page tells a running core with no session apart from a core that is not running.

## Done when

loadMining (app/src/lib/mining.ts) returns null only when coreStatus is null; a null status.mining shows the 'No mining yet' text. Tests in lib/mining.test.ts.

## Context

Found walking astromech-x81p; reviewer confirmed against crates/astromech-core/src/app.rs: the core sends mining only when a session exists, so with the core up and no session the page says 'The core isn't running, so there is no mining run to show.'
