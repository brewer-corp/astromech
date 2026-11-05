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
closed_at: "2026-11-05T06:31:08.637944334+00:00"
status: "done"
title: "Flaky on CI: a_dock_in_the_live_half_of_a_reopened_journal_still_gets_its_market_callout"
updated_at: "2026-11-05T06:31:08.637944334+00:00"
---

## Goal

Main stays green: the bh0c pipeline test passes on the CI runner every time, not only on a fast host.

## Done when

- The cause of the CI failure (run 41948812450, main @ 4212d3e4: `left: []`, `right: ["Market."]` at pipeline.rs:2912) is established and written in the close reason.
- The test or the code it covers is fixed at that cause (no retries or longer sleeps hiding it), and the fix is shown to hold under load that reproduced the failure, if a reproduction was found.
- `cargo test -p astromech-core` passes; CI on the PR passes.

## Context

Passes 15/15 locally (`cargo test -p astromech-core --lib`). The test stamps journals with `ts(n)` relative to now and `touch`es mtimes, so it may depend on wall-clock time between steps on a slow 2-core runner.

## Close Reason

Implementer report. Cause: the bh0c test wrote the live dock's Market.json and its journal Market event with two separate ts(0) clock reads. When a second boundary fell between them, the file was stamped a second before its event; parse_fresh correctly rejects a companion file older than its event, so the live event stayed pending and "Market." never fired (main CI run 41948812450: left: []). The product was right: the game stamps both with the same time. Reproduced deterministically by a 1.1 s sleep between the file write and the journal append (same assertion; tailer logged "file is older than its event"); the same sleep after the appends passes. Fix: the test takes one `now` for Market.json, Docked and Market (e5138f36); no pipeline code changed. With the sleep injected the fixed test passed 10/10; 50/50 on one core under a CPU burner; cargo test -p astromech-core, clippy -D warnings and fmt pass; PR #206 CI green (Linux, Windows, installer, MVP criteria). Every other market_file write in pipeline.rs shares one stamp with its event or stamps the file after it. Deviation: load alone never reproduced the failure (microsecond window), so the reproduction used an injected sleep. Review: approved by reviewer-sonnet-aiwv (astromech-sn9z), no findings.
