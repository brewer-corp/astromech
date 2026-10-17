---
created_at: "2026-10-17T18:19:56.459972261+00:00"
id: "astromech-dpoc"
issue_type: "task"
labels:
- "alpha"
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-dpoc
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-94j4"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:28:58.666256002+00:00"
status: "done"
title: "Guide: the route follower, paste and plotters"
updated_at: "2026-10-17T19:28:58.666256002+00:00"
---

## Goal

Replace the placeholder `docs/guide/routes.md` with a real page now that the route follower has landed (astromech-1135, PR #112): the Route page, following a route, the next waypoint on the clipboard, the paste button (galaxy map only, what it presses, the Enter re-check), Go on search results and Follow on engineer trips, the route voice commands and replies, the route_follow callout and its setting, and what happens at startup and on replayed journals. Add the Spansh plotters (astromech-94j4) when they land.

## Done when

The page matches the code on main (checked by someone other than the writer), `scripts/check-guide.py` passes, and voice-commands.md and callouts.md list the route phrases and the route_follow callout.

## Close Reason

routes.md is the real page (follower, paste, Go/Follow, voice commands, route_follow, startup and replays, plotters); voice-commands.md and callouts.md list the route phrases and the route_follow callout; check-guide passes; checked against the code by an independent reviewer (2 rounds). Stacked on #122: merge after it.
