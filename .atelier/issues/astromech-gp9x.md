---
created_at: "2026-10-22T22:02:52.474534576+00:00"
id: "astromech-gp9x"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-gp9x
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
status: "review"
title: "Flaky on Windows CI: gear_down did_you_mean_learns_a_key_press_on_the_second_yes gets Denied(Duplicate)"
updated_at: "2026-10-22T22:06:39.961446317+00:00"
---

## Goal

`did_you_mean_learns_a_key_press_on_the_second_yes` (crates/astromech-core/tests/gear_down.rs:392) passes reliably on the Windows CI job.

## Done when

The cause is established (diagnose first) and fixed in the test or the code, the fix explains why it failed, the old test fails deterministically and the fixed one passes when the fake status delay is raised well above the CI margin, a 100-run local loop passes, and the PR's Windows CI job is green. (Revised 22 Oct 2026 from "20 consecutive runs on Windows CI": the fix removes the race rather than widening a timing margin, so repeated Windows runs add little; review finding astromech-lbz8.)

## Context

Failed on main (run 39858023625) and on PR #176 (run 39860833207, job 120110789294) on 22 Oct 2026, passing on other runs. The third 'Dear dawn.' is expected to run as an alias and get Outcome::Already, but got `Ran { outcome: Denied(Duplicate) }`: the policy's 3-second repeat guard refused the press, which suggests the gear-down state from the second press hadn't reached the runner yet (timing on the slower Windows runner).

## Notes

- 2026-10-22 [note] Root cause: the test dropped the first press's verification handle, so the second yes and the alias run raced the fake game's 100 ms Status.json write; until the hub sees gear down, plan_with presses and the policy's in-flight guard refuses (Denied(Duplicate)). Linux margin ~10 ms (gear lands ~138 ms, third plan snapshot ~145 ms after grammar matching); reproduced the CI failure on Linux by raising the fake delay to 200 ms. Fix in the test (join the first verification); product behaviour ("Already on it." while the press is in flight, no second toggle press) is correct. Siblings in gear_down.rs already join or wait_flag. Windows confirmation pending on PR CI.
