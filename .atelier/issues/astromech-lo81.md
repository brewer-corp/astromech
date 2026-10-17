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
status: "in_progress"
title: "Route follower: track the current system even with route_follow turned off"
updated_at: "2026-10-17T18:58:25.100580858+00:00"
---

## Goal

The route's current system (the Route page's 'you are here' marker, and Desk::set starting after the first stop when already there) stays current when `[callouts.route_follow] enabled = false`.

## Done when

With the rule disabled, a jump updates the desk's current system, with a test.

## Context

Review finding astromech-8os1 on PR #112 (astromech-1135). The engine asks only enabled rules, for both react() and replayed(), so with the toggle off the current system goes stale.
