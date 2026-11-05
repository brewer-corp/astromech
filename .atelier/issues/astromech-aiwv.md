---
created_at: "2026-11-05T06:06:20.721136457+00:00"
id: "astromech-aiwv"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-aiwv
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-bh0c"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Flaky on CI: a_dock_in_the_live_half_of_a_reopened_journal_still_gets_its_market_callout"
updated_at: "2026-11-05T06:06:24.361299383+00:00"
---

## Goal

Main stays green: the bh0c pipeline test passes on the CI runner every time, not only on a fast host.

## Done when

- The cause of the CI failure (run 41948812450, main @ 4212d3e4: `left: []`, `right: ["Market."]` at pipeline.rs:2912) is established and written in the close reason.
- The test or the code it covers is fixed at that cause (no retries or longer sleeps hiding it), and the fix is shown to hold under load that reproduced the failure, if a reproduction was found.
- `cargo test -p astromech-core` passes; CI on the PR passes.

## Context

Passes 15/15 locally (`cargo test -p astromech-core --lib`). The test stamps journals with `ts(n)` relative to now and `touch`es mtimes, so it may depend on wall-clock time between steps on a slow 2-core runner.
