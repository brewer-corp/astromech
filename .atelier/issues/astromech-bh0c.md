---
created_at: "2026-10-17T18:47:41.480795224+00:00"
id: "astromech-bh0c"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-bh0c
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Reopened journal: companion triggers in the replayed half can use up the live half's callout"
updated_at: "2026-10-17T22:18:55.798471058+00:00"
---

## Goal

When a reopened journal is split into a replayed part and a live part (astromech-7uj3), a Market.json read for a Docked in the replayed part is applied with live=false and may mark the market callout done (Engine::market dedupes on market_done), so a live Docked at the same market in the live part gets no callout.

## Done when

A test with a Docked in each half at the same market shows the live one reaches the callouts, or the dedupe is shown not to apply.

## Context

Review finding on the astromech-7uj3 PR (companion triggers can be used up by the replay half).

## Notes

- 2026-10-17 [note] Shown not to apply: Engine::market returns before setting market_done when not live, so a replayed Market.json read cannot use up the dock's callout, and every Docked (replayed or not) resets market_done, so a live dock in the live half of a reopened journal always gets its own chance. Test a_dock_in_the_live_half_of_a_reopened_journal_still_gets_its_market_callout passes on main (a regression guard, not a fail-before test). Mutation check: setting market_done before the live check does NOT fail it, because the Docked reset masks that; it would only matter for a Market with no Docked before it, which is not a new dock. One residual effect is not addressed: a replayed Docked clears market_done, so a live Market handled earlier for the same dock could in principle fire again if a live Market event for it were applied later; no such path exists today.
