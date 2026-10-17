---
created_at: "2026-10-17T00:31:20.202543644+00:00"
id: "astromech-azr6"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Trade: show how many fetched markets passed the filters"
updated_at: "2026-10-17T00:31:20.202543644+00:00"
---

## Goal

Ardent returns a fixed 100 rows for galaxy-wide prices and ignores the pad; the pad (default: the ship's) is applied locally, so a large ship can see a handful of rows or none with no explanation. Prices.truncated is computed before filtering, so it is always true and never shown.

## Done when

The Anywhere result footer or empty state says how many of the fetched markets passed the filters (e.g. '12 of 100 pass your pad filter'), truncated means something accurate, and a component test covers it.

## Context

Deferred from the astromech-d1lo review (View.svelte:94-113, trade.rs:299-321, SearchKit.svelte:114).
