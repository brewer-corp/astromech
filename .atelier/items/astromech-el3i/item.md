---
schema: atelier.item
schema_version: 3
id: astromech-el3i
title: Trade route as legs
status: building
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A3d492987-933c-450a-9df8-366049ceb011
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:45.472196717Z
---

## Goal

A found trade route reads as the trip it is: a stop, what to buy there, the leg to the next stop, what it sells for and what the leg earns, and for the whole route the credits per hour. Today the result side of Trade > Commodities is a numbered list in which each leg packs both stations, both systems, the cargo, the profit and the trip facts onto one row of prose fragments (`app/src/modules/trade/Route.svelte`, lines 160 to 200).

## Done when

- [ ] The result side of the Workbench (the inputs stay as they are on the left) reads as stops joined by legs, top to bottom. A stop: the station in Name, Options Grey (the first stop, where the trip starts, White), its system with its copy icon, the pad as DESIGN.md's pad letter and the arrival distance in Label. A leg, between two stops: the cargo to buy at the stop above, each commodity as "200 t Biowaste" in Prose Grey with "buy 110 · sell 754" in Label; the leg's profit as its one value in Value L, Options Grey (the legs are in trip order, not ranked), with the profit per tonne in Label; jumps and light years in Label. A leg with no cargo (a loop's way back with nothing worth carrying) says "Fly back empty" in Label Grey. A loop ends back at its first stop, drawn once more.
- [ ] The route's totals lead the result: profit per hour (`per_hour`) as the page's main value in Value XL Orange Lit with "cr an hour, estimated" in Label and the estimate's model in its Tip (`ROUTE_ESTIMATE`); the profit of one round and the minutes a round takes in Label. The source and the market data's age stay one Label line.
- [ ] A stop whose largest pad is too small for the ship shows its pad letter in Red Low, struck through, with the reason as its accessible name, as DESIGN.md's list says; the route's `warning` line stays an orange "!" line above the stops.
- [ ] States: before a search the result side holds the one Label line it has today and nothing else; asking Spansh one Label Grey line ("Asking Spansh… a route can take a minute."); a failed ask the shared Failure line with TRY AGAIN; no profitable route the one prose line with what to relax, as today.
- [ ] The brightest things: the profit per hour (main value), the inputs and Find route (what can be changed), and a pad too small (what needs the commander, else none). Orange only for the main value and the controls; no colour outside DESIGN.md's list.
- [ ] No printed explanations: the estimate's assumptions stay in the Tip.
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded single hop and loop (`app/src/preview/fixtures/trade.json`, `route`): with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the result goes below the inputs (the Workbench's `min`). Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/trade-route/`, single hop and loop. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/trade/{View,Route}.svelte` in `PAGE_SIZING` (width is on today; font size joins it), and the `trade` entry leaves `TOKENS_EXEMPT`.
- [ ] Tests: the eight cases in `trade/Route.browser.test.ts` keep their behaviour on the new markup. No test of layout or copy.
- [ ] `docs/guide/trade.md`'s Route section describes the result.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Trade > Commodities' result side. The inputs, the filters and the Places links above them stay. No core change.

## Context

There is no mockup; the stops and legs follow the route sidebar's timeline (`app/src/modules/route/Timeline.svelte`) in spirit, as a vertical list of stops with what happens between them, but on Void, not the frame's Hull. Replies: `TradeRoute.ts` (`here`, `route`), `TradeRouteResult.ts` (`kind`, `legs`, `profit`, `minutes`, `per_hour`, `warning`, `range`, `fetched_at`), `Leg.ts` (`from`, `to`, `ly`, `jumps`, `cargo`, `profit`, `profit_per_t`, `source`, `seconds`), `Cargo.ts`, `TradeRouteStop.ts` (`pad`, `arrival_ls`).
