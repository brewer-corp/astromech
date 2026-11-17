---
schema: atelier.item
schema_version: 3
id: astromech-m07p
title: Colonisation as progress per commodity
status: in_review
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

- [x] Trade > Colonisation is a List with detail page on the type roles: the list holds the site's progress and then Where to buy, whose station rows fill the side panel when selected, as a Places finder's do (SearchKit inside a ListDetail, `useDetail`). The site's head: its station name as the Title, White (`station_name`, else the market id as today's `projectName` gives it), the system with its copy icon, and the progress (`progress`) as the page's main value in Value XL Orange Lit ("62 % built"), with a meter under it, Prose Grey on Mark Off. Complete or failed says so in its place.
- [x] Each commodity the site requires (`resources`) is one row: its name in Name, Options Grey; a meter of provided against required (`provided`, `required`), Prose Grey on Mark Off, with the aboard tonnes (`shopping.lines[].in_cargo`) shown as an Orange Mid part of the meter after the delivered part; then "still to buy" (`to_buy`) as the row's ranked value in Value L Orange, and "provided of required t" and "aboard" in Label. A commodity complete is Grey Dim with a full meter. Rows order as the reply's shopping lines (largest still to buy first), complete ones after.
- [x] The aboard part of the meter is a new entry on DESIGN.md's list of colours that carry a meaning, added by this item: "A colonisation delivery: aboard and not yet delivered, Orange Mid; also shown by the 'aboard' figure in words". It is the one new colour use.
- [x] Under the rows, the loads left for the ship flown (`shopping.loads`, against `shopping.capacity`) in Label; no capacity known says so.
- [x] **Where to buy** keeps today's SearchKit (`app/src/lib/ui/SearchKit.svelte`, unchanged by this item): stations stocking the most of the list first, searched from the site's system. The paragraph explaining the order and the From field moves into the SearchKit title's Tip.
- [x] A coloniser with several sites keeps the site Dropdown above the head, each option with its progress. The candidates (`Colonies.svelte`, systems still open to claim near you) stay under the site, as today.
- [x] States: no sites in the journals, reading the journals, a failed read (the Stale line, or the words with no action: the page re-reads on its own), everything aboard ("deliver it") and complete or failed keep today's words; a finished project asks no search, as today.
- [x] The brightest things: the site's progress (main value), the Dropdown, the search's controls and its row actions (what can be changed), and the commodities still short with nothing aboard (their still-to-buy figures in Orange: what needs the commander). No other colour than the list's.
- [x] No printed explanations remain on the page.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded `colonisation` and `colonies` fixtures: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/colonisation/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/colonisation/View.svelte` in `PAGE_SIZING`, and the `colonisation` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: the nine cases in `colonisation/View.browser.test.ts` keep their behaviour on the new markup. No test of layout or copy.
- [x] `docs/guide/colonisation.md`'s Projects section describes the progress rows.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Trade > Colonisation. `SearchKit.svelte` and `Colonies.svelte` are shared and not changed here. No core change.

## Context

There is no mockup. Replies: `colonisation_projects` (`ProjectView.ts`: `progress`, `complete`, `failed`, `resources: Resource[]` with `required`, `provided`, `payment`; `shopping: ShoppingList` with `lines` (`remaining`, `in_cargo`, `to_buy`), `to_buy_total`, `capacity`, `loads`). The fixtures are `app/src/preview/fixtures/colonisation.json` and `colonies.json`.

## Report

Trade > Colonisation now reads as progress toward building the site. The page is a List with
detail: the site's head with "42 % built" as the main value and a meter, one row per required
commodity, the loads left, then Where to buy, whose station rows fill the side panel. Code:
`app/src/modules/colonisation/View.svelte`, with the row join in `progressOf`
(`app/src/lib/colonisation.ts`). Commit f3e0ec88, with a separate tool commit 96a7c2ec.

### One test changed in kind

Test 2 ("keeps each station's stock, trip and report age…") asserted SearchKit's folded third
line. SearchKit folds only outside a ListDetail (`folded = compact && !detail`). Inside one, a
narrow row goes to one line and the side panel holds the rest, as in every Places finder. The
test now selects Efremov Plant at 1100 x 700 at 145 percent and finds the full stock, the trip
and the report's age in the panel. That is the same outcome (nothing is lost when the row is too
narrow), reached the way the layout does it. The other eight keep their behaviour. Tests 1 and 4
changed only where they read the old copy ("42% built", "… t in all · …") or the old list.

### Calls I made where the item left room

- **Still-to-buy colour.** The figure is Orange whenever any is still to buy, including a row
  with some aboard (Insulating Membrane: 90 t aboard, 310 t still to buy). That is the plain
  reading of the second Done-when line, and those tonnes still need the commander. A row whose
  remainder is all aboard shows "0" in Value L's default grey. A complete row is Grey Dim.
- **"Complete" and "Failed" in the head** are Value XL in Options Grey, not Orange Lit and not
  Alarm Red. They are facts about the project, not an app failure (The Narrow Red Rule).
- **Colonies sits outside the ListDetail.** Its own SearchKit would write the same detail
  context (`SearchKit.svelte:351-357`), and the last effect to run would win the panel. Below the
  layout it keeps its own rows and stays shown in every state, as test 9 needs.
- **The aboard part is a `coming` prop on `Meter.svelte`.** It is drawn after the fill in Orange
  Mid, so the page sets no sizes. DESIGN.md gets the new row on the list of colours that carry a
  meaning, and the Meter bullet says how a colonisation commodity uses it. That bullet also says
  "a shopping list's fill turns Orange Lit when complete". I followed this item instead (a
  complete commodity is Grey Dim with a full Prose Grey meter), because that sentence is about
  Suits' shopping list.
- **Where to buy's Tip** sits on a new Section heading "Where to buy" above the kit, since
  SearchKit has no visible title of its own and was not to change. The heading is inside the
  branch with things to buy, so a finished project shows neither heading nor search.

### Outside this item's scope (not changed, for the lead)

- `Colonies.svelte` still prints a paragraph ("Unclaimed systems within 15 ly of a populated
  one…"). "No printed explanations remain on the page" collides with Scope, which keeps
  Colonies unchanged. It wants its own item, moving the paragraph to its heading's Tip.

### Browser check

`docs/design/colonisation/plan.json` captures the site, a station selected, the complete site
(the second site chosen in the Dropdown), no sites, reading and a failed read. The site states
are at 1100 x 700 and 1920 x 1080 at 100 percent and 2560 x 1440 at 145; the others at 1100 and
2560. The before captures, plan and report are in `docs/design/colonisation/before/`. With the
route followed, the sidebar is docked at 1920 and 2560 and is the strip at 1100. No sites runs
on `core=empty`, which has no route, so the frame is the rail there.

Every capture lists nothing under `sideways`, `overlaps`, `small`, `contrast` or `far`. The only
`clipped` entries (9 or 10 per site capture) are SearchKit's Stock text, ellipsised on the row
(`div.row.result > span.g.own > span.l1 > span.text`). Before this change it was cut the same
way, 11 at 1100 and 9 at 2560. SearchKit is not changed here, and the side panel shows the
stock whole (`site-station-selected-*.png`), so it is not a fault.

`bright` ranks by luminance, so the White title and the Options Grey names lead it, then Prose
Grey figures. The first colour on the page is the main value "42" (Orange Lit), alone in its
step. The Orange step below the cut-off holds the Dropdown, CHANGE, the row actions and the
still-to-buy figures. On the complete site the head says "Complete" in grey, so the brightest
orange is the candidates' chosen option. The page has one fill, Set destination in the panel
(the top tab is frame).

The aboard part of the meter appears in the fixture only on Insulating Membrane. Three states
have no fixture and are not captured: everything aboard, a failed project and no cargo capacity.
Their words are unchanged from before ("Everything still needed is aboard: deliver it.") or new
("no cargo capacity known for the ship flown, so no loads").

### Tool change

`scripts/browser/audit.mjs` gains a `select` step (choose a dropdown option by its starting
text), so a plan can capture the second site. It is its own commit, 96a7c2ec.

### Checks

`pnpm check` (0 errors, 0 warnings), `pnpm lint`, `pnpm test` (92 files, 788 tests) and the
journeys (7 of 7) pass. The journeys ran against a core built incrementally from this branch.
The worktree had no `target/`, so I built `astromech-core` into the main checkout's target dir
and pointed `ASTROMECH_CORE` at it.


### Responses

1-aj8y.1: fixed in cb35a7d9: `needsOf` asks for nothing once a project is complete or has failed, so the page shows "Nothing left to deliver." and mounts no search; covered at the lowest layer that failed, `colonisation.test.ts` ("asks for nothing once a project has failed").
1-aj8y.2: fixed in cb35a7d9: a row's name falls back to the shopping line's symbol, else the resource's journal name normalised as the core's `commodity_id` does; covered in `colonisation.test.ts` ("names a commodity by its market symbol").
