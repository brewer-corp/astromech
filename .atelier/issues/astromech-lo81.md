---
created_at: "2026-10-17T17:40:28.379811892+00:00"
id: "astromech-lo81"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-lo81
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:13:37.920867910+00:00"
status: "done"
title: "Route follower: track the current system even with route_follow turned off"
updated_at: "2026-10-17T19:13:37.920867910+00:00"
---

## Goal

The route's current system (the Route page's 'you are here' marker, and Desk::set starting after the first stop when already there) stays current when `[callouts.route_follow] enabled = false`.

## Done when

With the rule disabled, a jump updates the desk's current system, with a test.

## Context

Review finding astromech-8os1 on PR #112 (astromech-1135). The engine asks only enabled rules, for both react() and replayed(), so with the toggle off the current system goes stale.

## Close Reason

With route_follow disabled, jumps (live and replayed) still update the desk's current system through the engine's new while_off hook; tests in announce and core. Reviewed in PR #126, no open FIX NOW.
