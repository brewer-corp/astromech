---
created_at: "2026-10-17T23:26:17.106327862+00:00"
id: "astromech-cvxf"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Two polls still fail silently: the trip planner's engineer list and the header status"
updated_at: "2026-10-17T23:30:24.661394485+00:00"
---

## Goal

The two remaining polls that swallow a failure show it, or are recorded as fine as they are.

## Done when

- engineering/Build.svelte polls loadEngineers every second and ignores a failure (.catch(() => {})): the trip plan keeps using the last engineer progress with no sign. Show a Could not refresh line near the trip plan, or decide it is not worth one.
- modules/settings/View.svelte and lib/onboarding/Onboarding.svelte poll coreStatus(), which turns a failed call into null (shown as not running). Decide whether a failed poll after a good one should say the status is stale rather than 'not running'.

## Context

Found while doing astromech-spxx, which covers every page whose data comes from a poll.

## Notes

- 2026-10-17 [note] Also check trade/Route.svelte:43: the loadDefaults catch falls back to FALLBACK_HOP_LY with no sign that the ship's range was not read. Not a poll; decide whether it needs a line.
