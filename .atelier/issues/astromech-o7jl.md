---
created_at: "2026-10-17T00:46:21.886579582+00:00"
id: "astromech-o7jl"
issue_type: "bug"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Flaky Windows test: notifications_run_in_the_background_at_most_one_per_gap"
updated_at: "2026-10-17T00:46:21.886579582+00:00"
---

## Goal

The callout notifier test in crates/astromech-core/src/callouts.rs uses fixed sleeps (700 ms, 600 ms, 400 ms) against a 200 ms slow notifier and a 300 ms gap. On a loaded Windows CI runner (PR #42, run 2026-10-17) the background thread hadn't delivered 'A.' or 'B.' within 1 s, so the assertion saw [] and the notify thread then panicked on a poisoned mutex.

## Done when

The test waits on conditions with a generous deadline instead of fixed sleeps, still proves notify doesn't block, at most one notification per gap, and that the next goes after the gap; it passes repeatedly under CPU load (e.g. 20 runs with stress), and fails if the gap rule is removed.

## Context

Windows CI log from PR #42. Same family as astromech-2hml (gear_down latency floor).
