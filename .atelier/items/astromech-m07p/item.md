---
schema: atelier.item
schema_version: 3
id: astromech-m07p
title: Colonisation as progress per commodity
status: building
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A4c0ff1c6-a31d-43a7-995a-48c4914ed293
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:43.303344530Z
---

## Goal

A construction site reads as progress toward building it: how far along it is, each commodity as delivered against required with what is aboard marked, the loads left for the ship flown, and where to buy what is left. Today it is a list of tonnes still to buy, a total line and a paragraph that explains the search (`app/src/modules/colonisation/View.svelte`); the site's progress shows only as a percentage in the dropdown.

## Done when

- [ ] Trade > Colonisation is a List with detail page on the type roles: the list holds the site's progress and then Where to buy, whose station rows fill the side panel when selected, as a Places finder's do (SearchKit inside a ListDetail, `useDetail`). The site's head: its station name as the Title, White (`station_name`, else the market id as today's `projectName` gives it), the system with its copy icon, and the progress (`progress`) as the page's main value in Value XL Orange Lit ("62 % built"), with a meter under it, Prose Grey on Mark Off. Complete or failed says so in its place.
- [ ] Each commodity the site requires (`resources`) is one row: its name in Name, Options Grey; a meter of provided against required (`provided`, `required`), Prose Grey on Mark Off, with the aboard tonnes (`shopping.lines[].in_cargo`) shown as an Orange Mid part of the meter after the delivered part; then "still to buy" (`to_buy`) as the row's ranked value in Value L Orange, and "provided of required t" and "aboard" in Label. A commodity complete is Grey Dim with a full meter. Rows order as the reply's shopping lines (largest still to buy first), complete ones after.
- [ ] The aboard part of the meter is a new entry on DESIGN.md's list of colours that carry a meaning, added by this item: "A colonisation delivery: aboard and not yet delivered, Orange Mid; also shown by the 'aboard' figure in words". It is the one new colour use.
- [ ] Under the rows, the loads left for the ship flown (`shopping.loads`, against `shopping.capacity`) in Label; no capacity known says so.
- [ ] **Where to buy** keeps today's SearchKit (`app/src/lib/ui/SearchKit.svelte`, unchanged by this item): stations stocking the most of the list first, searched from the site's system. The paragraph explaining the order and the From field moves into the SearchKit title's Tip.
- [ ] A coloniser with several sites keeps the site Dropdown above the head, each option with its progress. The candidates (`Colonies.svelte`, systems still open to claim near you) stay under the site, as today.
- [ ] States: no sites in the journals, reading the journals, a failed read (the Stale line, or the words with no action: the page re-reads on its own), everything aboard ("deliver it") and complete or failed keep today's words; a finished project asks no search, as today.
- [ ] The brightest things: the site's progress (main value), the Dropdown, the search's controls and its row actions (what can be changed), and the commodities still short with nothing aboard (their still-to-buy figures in Orange: what needs the commander). No other colour than the list's.
- [ ] No printed explanations remain on the page.
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded `colonisation` and `colonies` fixtures: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/colonisation/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/colonisation/View.svelte` in `PAGE_SIZING`, and the `colonisation` entry leaves `TOKENS_EXEMPT`.
- [ ] Tests: the nine cases in `colonisation/View.browser.test.ts` keep their behaviour on the new markup. No test of layout or copy.
- [ ] `docs/guide/colonisation.md`'s Projects section describes the progress rows.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Trade > Colonisation. `SearchKit.svelte` and `Colonies.svelte` are shared and not changed here. No core change.

## Context

There is no mockup. Replies: `colonisation_projects` (`ProjectView.ts`: `progress`, `complete`, `failed`, `resources: Resource[]` with `required`, `provided`, `payment`; `shopping: ShoppingList` with `lines` (`remaining`, `in_cargo`, `to_buy`), `to_buy_total`, `capacity`, `loads`). The fixtures are `app/src/preview/fixtures/colonisation.json` and `colonies.json`.
