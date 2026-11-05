---
created_at: "2026-11-05T06:19:45.033104685+00:00"
id: "astromech-kwoi"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-nxz9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-sllo"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Move the Trade and Materials finders into Places"
updated_at: "2026-11-05T06:19:45.033104685+00:00"
---

## Goal

Move the place-finding lookups that live on other pages today into Places as finders, so "where is X" has one home (decision astromech-bi94): Sell, Buy and Sell my cargo from Trade's Commodities page, and Raw sites and Faction states (the HGE and Boom/Famine/War lookups) from Materials. Colonisation stays where it is.

## Done when

- Places home shows the Trade tiles (Sell, Buy, Sell my cargo) and the Engineering tiles Raw sites and Faction states; each is a finder page at `#/places/<finder>[/<input>]` with its main input (commodity, site kind, state or HGE pool).
- The pages they came from link into the finder with its input filled in instead of embedding the list (decision astromech-g8d5, item 5): on Materials, the raw-sites lookup under the raw grid (`<Sites />`, `modules/materials/View.svelte` around line 155) and the faction-state lookup in the side panel for the selected material's source query (`<Nearby query={lookup} />`, around line 174, `faction.<query>`); on Trade, the Sell, Buy and Sell my cargo modes; on Exploration's System page, the embedded nearest-services lookup (`modules/exploration/View.svelte` around line 165, `stations.service`) becomes a link into the Services finder. There is no trader control on Materials today; material traders are astromech-nxz9's finder. Trade keeps trade routes and Colonisation. Old URLs and remembered filters (`search.filters.<lookup>`) still work.
- The moved finders are in the finder list the Find or plot box matches; if astromech-2hbw has landed, its phrase tests gain "sell painite" and a raw-site phrase.
- Component tests for the moved finders and the links; `pnpm lint && pnpm check && pnpm build && pnpm test` pass.
- `scripts/ui` screenshots of Places home, the Sell finder and the Materials page, before and after.
- docs/guide pages for Trade, Materials and Places updated; `python3 scripts/check-guide.py` passes.

## Scope

`app/src/modules/{trade,materials,exploration}`, `lib/ui/{Nearby,Sites}.svelte`, the places module. Voice queries are unchanged.

## Context

- Builds on astromech-nxz9; astromech-sllo folds Trade's mode tabs, so this starts after it.
- DESIGN.md: Places home, Side panel; mockup `#/mockup/ds-sell`.
