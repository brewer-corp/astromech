---
created_at: "2026-10-17T21:37:15.558189795+00:00"
id: "astromech-dr5j"
issue_type: "bug"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-dr5j
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Powerplay week and session merits count awards the game wrote twice"
updated_at: "2026-10-17T21:37:20.352270972+00:00"
---

## Goal

The Powerplay page's merits this week and this session match the game's totals when the journal repeats a `PowerplayMerits` award the game didn't count.

## Done when

A standing test from an anonymised excerpt: two identical `PowerplayMerits` lines (same second, `MeritsGained: 41`, `TotalMerits` 1272 then 1313) followed by a login `Powerplay` with `Merits: 1272` counts the 41 once this week and gives lifetime 1272; a following `PowerplayMerits` whose `TotalMerits - MeritsGained` is below the running total also takes the excess back. Replaying the real journals (astromech-yin0 harness) shows the week total the journal's totals give.

## Context

Found replaying real journals (astromech-yin0). The game wrote a `PowerplayMerits` line twice, on two days in two weeks (same timestamp and gain, `TotalMerits` counting both); the next `PowerplayMerits` (its `TotalMerits - MeritsGained`) and the next login's `Powerplay` `Merits` show only one was counted. `Standing` (`crates/astromech-powerplay/src/standing.rs`) adds every `MeritsGained` to the week and session, so the page said 113 this week where the game's totals give 72. Lifetime merits were right after the next login.
