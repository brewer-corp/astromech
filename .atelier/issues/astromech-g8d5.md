---
created_at: "2026-11-05T06:28:07.757784366+00:00"
id: "astromech-g8d5"
issue_type: "decision"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
  - kind: "issue"
    id: "astromech-os70"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T06:28:11.440762247+00:00"
status: "done"
title: "Galaxy rework: ten open design choices"
updated_at: "2026-11-05T06:28:11.440762247+00:00"
---

## Question

Planning astromech-51ue left ten design choices that DESIGN.md, PRODUCT.md and decision astromech-bi94 do not settle. Each shapes a child's Done when, so each needs an answer before the children start.

## Options

1. Branching: keep the engine, recipes, Find along route and the local station file as children of astromech-51ue (one branch that merges when all are done), or split them into a second increment.
2. Where Add stop inserts: as the next stop, at the end of the route, or where it adds the least detour.
3. Set destination and the old route: park it (Resume restores it) or discard it.
4. Galaxy's page tabs: Places, Route and System as in the mockups, or Places and Route only.
5. Pages that embed a lookup today (Trade, Materials, Exploration): keep the embedded lists, or replace them with links into the finder.
6. Places home tile figures: none, the last answer this session, or fetched when the home page opens.
7. Search box and voice: one shared phrase parser, a shared phrase file, or separate matchers.
8. Single-search finders on the engine: move all of them now, or only when "Find near this" needs one.
9. Recipes and live request volume: block the recipes on the Spansh maintainer's answer (astromech-5rm2), build the local station file (astromech-ibmy) first, or ship within the spike's budget.
10. Route strip for a route without jump counts (imports, lookups): show stops left, or hide the count.

## Decision

Decided by the independent reviewer reviewer-fable-51ue-plan (Fable 5.1), under Sumeet's direction that agents settle design choices DESIGN.md leaves open and take the reversible option:

1. Split the engine work into a second increment (astromech-os70). The route strip, actions, Places and Find or plot then reach main without waiting on someday work or on a reply from outside the project, and stay clear of the parallel design-system rework of the same files.
2. Add stop inserts the place as the next stop; Find along route (astromech-wla4) adds leg-aware insertion later. A commander who finds a place mid-route usually wants it now, and that is the simplest rule to undo.
3. Set destination parks the old route, and Resume restores it. "Never a silent replace" (astromech-bi94), using the parking the follower already has.
4. Galaxy's tabs are Places and Route; System stays in Exploration. It is already a page there, and moving it is outside this rework.
5. Embedded lookups become links into the finder with the input filled in. Places stays the one home for "where is X", without two copies of each list.
6. Home tiles show the finder's last answer this session; opening the home page makes no request. That keeps within the request budget and the rate limits.
7. The search box and voice keep separate matchers, both targeting the same finder IDs and URLs. That avoids coupling the TypeScript box to the Rust voice grammar while both still reach the same finders.
8. Single-search finders move onto the engine only when "Find near this" needs one. No refactor without a user-visible gain, and it is reversible.
9. The recipes are not blocked on astromech-5rm2, and astromech-ibmy is not built first: ship within the spike's budget (at most 2 live legs, about 4 requests per query) and keep astromech-5rm2 open in parallel. The spike showed the live cost is workable, and the local file stays available if the answer or measured volume calls for it.
10. A route strip without jump counts shows "N stops left". It stays honest about what is known and keeps the strip on one line.

## Close Reason

Ten open galaxy-rework design choices settled by reviewer-fable-51ue-plan (Fable 5.1) under Sumeet's direction, taking the reversible option each time; see the Decision section.
