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
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T15:11:19.673524923+00:00"
status: "done"
title: "Move the Trade and Materials finders into Places"
updated_at: "2026-11-05T15:11:19.673524923+00:00"
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

- Builds on astromech-nxz9. astromech-sllo (Two Tab Rows) starts after this and folds any third tab level Trade still has once Sell, Buy and Sell my cargo have moved.
- DESIGN.md: Places home, Side panel; mockup `#/mockup/ds-sell`.

## Notes

- 2026-11-05 [note] 2026-11-05 (orchestrator): no longer blocked by astromech-sllo (sllo now waits for this). Where moving the Trade modes leaves Trade with a third tab level, leave it for sllo rather than folding it here.
- 2026-11-05 [note] 2026-11-05 (implementer): reversible choices DESIGN.md and the decisions left open, each in the handoff with options: Trade's Commodities page keeps its title and becomes trade routes with a links line (no mode tabs left, so nothing for sllo to fold there); its links open Sell/Buy on Painite, the finders' default, as Trade has no commodity of its own any more; Sell my cargo has no main input (FinderPage's input snippet is optional; its FINDERS input is 'Hold'); Faction states opens on Boom and lists the HGE pools before Famine; Raw sites and Faction states pick from a dropdown; Materials links every site kind under the raw grid and 'Find the nearest systems' in the side panel; Engineering's build plan keeps its embedded Nearby (not in scope).

## Close Reason

Implementer report. Sell, Buy and Sell my cargo (Trade section) and Raw sites and Faction states (Engineering section) are Places finders at #/places/<sell|buy>/<commodity>[?range=anywhere], #/places/sell-cargo, #/places/raw-sites/<kind> and #/places/faction-states/<query>, in FINDERS, each remembering its last answer for its tile. Each keeps its old search-kit lookup id (trade.sell[.anywhere], trade.buy[.anywhere], trade.cargo, sites.<kind>, faction.<id>), so remembered filters and voice queries are unchanged; no Rust changed. Trade's Commodities page (now trade routes plus a links line; its Sell/Buy/My cargo/Route third tab row is gone), Materials (one link per raw site kind under the raw grid; "Find the nearest systems" in the side panel when the source is a faction state) and Exploration (Nearest Universal Cartographics / Vista Genomics via the Services finder) link into the finders with the input filled. None of the moved lookups had its own URL on main (verified by the reviewer), so the old page URLs still open their pages; an App test follows all nine links. lib/ui/Sites.svelte and CommodityInput are removed; nearbyResults moved to lib/galaxy.ts (Nearby stays for the engineering build plan). Guide pages and ARCHITECTURE updated. Proof: app lint, svelte-check 0/0 (460 files), build, 455 tests; check-guide 0 problems; link tests fail with the source pages reverted; screenshots docs/design/places-kwoi-*.png. 2hbw phrase tests ("sell painite", a raw-site phrase) are 2hbw's, since it lands second. Review: reviewer-fable-kwoi approved (astromech-gqpm); abu3 → astromech-vaz9, tc9s → astromech-5s9h, 7r5k and 6q7m folded into astromech-2hbw, m881 records seven decisions (all matching the branch: keep "Commodities", links open Painite, no main input for Sell my cargo, Boom first, dropdown pickers, link every site kind, no extra Ctrl+K entries before 2hbw).
