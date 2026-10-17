---
created_at: "2026-10-17T18:47:41.480795224+00:00"
id: "astromech-bh0c"
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
status: "todo"
title: "Reopened journal: companion triggers in the replayed half can use up the live half's callout"
updated_at: "2026-10-17T18:47:41.480795224+00:00"
---

## Goal

When a reopened journal is split into a replayed part and a live part (astromech-7uj3), a Market.json read for a Docked in the replayed part is applied with live=false and may mark the market callout done (Engine::market dedupes on market_done), so a live Docked at the same market in the live part gets no callout.

## Done when

A test with a Docked in each half at the same market shows the live one reaches the callouts, or the dedupe is shown not to apply.

## Context

Review finding on the astromech-7uj3 PR (companion triggers can be used up by the replay half).
