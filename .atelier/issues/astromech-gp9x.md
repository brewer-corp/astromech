---
created_at: "2026-10-22T22:02:52.474534576+00:00"
id: "astromech-gp9x"
issue_type: "bug"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Flaky on Windows CI: gear_down did_you_mean_learns_a_key_press_on_the_second_yes gets Denied(Duplicate)"
updated_at: "2026-10-22T22:02:52.474534576+00:00"
---

## Goal

`did_you_mean_learns_a_key_press_on_the_second_yes` (crates/astromech-core/tests/gear_down.rs:392) passes reliably on the Windows CI job.

## Done when

The cause is established (diagnose first) and fixed in the test or the code; the test passes in 20 consecutive runs on Windows CI (or a loop on the Windows job), and the fix explains why it failed.

## Context

Failed on main (run 39858023625) and on PR #176 (run 39860833207, job 120110789294) on 22 Oct 2026, passing on other runs. The third 'Dear dawn.' is expected to run as an alias and get Outcome::Already, but got `Ran { outcome: Denied(Duplicate) }`: the policy's 3-second repeat guard refused the press, which suggests the gear-down state from the second press hadn't reached the runner yet (timing on the slower Windows runner).
