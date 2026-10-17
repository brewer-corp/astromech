---
created_at: "2026-10-17T17:33:44.425564290+00:00"
id: "astromech-2za5"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-2za5
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:25:45.828685992+00:00"
status: "done"
title: "Binds health check: warn when Key_V or Key_Enter is bound in the galaxy map"
updated_at: "2026-10-17T19:25:45.828685992+00:00"
---

## Goal

The binds health check warns when `Key_V` or `Key_Enter` is bound to a galaxy-map action, since paste (`Runner::paste`, docs/PLOTTING.md) sends Ctrl+V and Enter whenever the galaxy map is open, and nothing tells astromech whether its search box has focus. With such a binding, a paste with the box unfocused fires that map action.

## Done when

The health check reports a galaxy-map action bound to `Key_V` or `Key_Enter` (with or without modifiers), with a test on a synthetic .binds file.

## Context

Review finding R5 on PR #112 (astromech-1135, route follower). Deferred from the alpha landing of the route follower.

## Close Reason

Done-when met: The health check warns on V or Enter on a galaxy map action; test on a synthetic .binds file; PR 116; see evidence
