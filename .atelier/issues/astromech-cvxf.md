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
closed_at: "2026-10-22T21:44:44.653140613+00:00"
status: "done"
title: "Two polls still fail silently: the trip planner's engineer list and the header status"
updated_at: "2026-10-22T21:44:44.653140613+00:00"
---

## Goal

The two remaining polls that swallow a failure show it, or are recorded as fine as they are.

## Done when

- engineering/Build.svelte: a failed engineer-progress poll shows a Could not refresh line under the Engineer trip heading (Could not read engineer progress before any good read), cleared by the next good read. Tests in Build.test.ts.
- modules/settings/View.svelte and lib/onboarding/Onboarding.svelte keep the last status when a later status read fails, with a Could not refresh: the core did not answer. line, instead of the not-running page (Settings) or Not set for the buttons (setup). Before any good read Settings keeps its not-running text. Tests in settings/View.test.ts and Onboarding.test.ts.
- trade/Route.svelte: a failed tradeRouteDefaults says Could not read your ship and credits, with the hop at its 20 ly default. Test in trade/Route.test.ts.

## Context

Found while doing astromech-spxx, which covers every page whose data comes from a poll.

## Notes

- 2026-10-17 [note] Also check trade/Route.svelte:43: the loadDefaults catch falls back to FALLBACK_HOP_LY with no sign that the ship's range was not read. Not a poll; decide whether it needs a line.
- 2026-10-22 [note] 2026-10-22 decisions (implementer, from the astromech-spxx convention that a polling page keeps its data and says the refresh failed): (1) Trip planner: show the line; a silent failure leaves the trip planned from old engineer progress. (2) Settings and setup: keep the last status and say it is stale. coreStatus is null for any failed status call (the shell's core_status is call("status").ok()), so the window cannot tell a stopped core from one that did not answer, and 'not running' or 'Not set' was a claim it could not make. (3) The App header is left as it is: it is the liveness indicator, and 'Not running' there is how the window sees a core that does not answer; the next good read (2 s) corrects it. (4) The trade Route defaults (2026-10-17 note) get a line too, since the empty fields and 20 ly hop otherwise look like real values. Sumeet: say if the header should read something like 'Not answering' instead.

## Close Reason

Done and independently reviewed on feature/astromech-jo3l (approval astromech-15yv).
