---
schema: atelier.item
schema_version: 3
id: astromech-el3i
title: Trade route as legs
status: changes_requested
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

- [x] The result side of the Workbench (the inputs stay as they are on the left) reads as stops joined by legs, top to bottom. A stop: the station in Name, Options Grey (the first stop, where the trip starts, White), its system with its copy icon, the pad as DESIGN.md's pad letter and the arrival distance in Label. A leg, between two stops: the cargo to buy at the stop above, each commodity as "200 t Biowaste" in Prose Grey with "buy 110 · sell 754" in Label; the leg's profit as its one value in Value L, Options Grey (the legs are in trip order, not ranked), with the profit per tonne in Label; jumps and light years in Label. A leg with no cargo (a loop's way back with nothing worth carrying) says "Fly back empty" in Label Grey. A loop ends back at its first stop, drawn once more.
- [x] The route's totals lead the result: profit per hour (`per_hour`) as the page's main value in Value XL Orange Lit with "cr an hour, estimated" in Label and the estimate's model in its Tip (`ROUTE_ESTIMATE`); the profit of one round and the minutes a round takes in Label. The source and the market data's age stay one Label line.
- [x] A stop whose largest pad is too small for the ship shows its pad letter in Red Low, struck through, with the reason as its accessible name, as DESIGN.md's list says; the route's `warning` line stays an orange "!" line above the stops.
- [x] States: before a search the result side holds the one Label line it has today and nothing else; asking Spansh one Label Grey line ("Asking Spansh… a route can take a minute."); a failed ask the shared Failure line with TRY AGAIN; no profitable route the one prose line with what to relax, as today.
- [x] The brightest things: the profit per hour (main value), the inputs and Find route (what can be changed), and a pad too small (what needs the commander, else none). Orange only for the main value and the controls; no colour outside DESIGN.md's list.
- [x] No printed explanations: the estimate's assumptions stay in the Tip.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded single hop and loop (`app/src/preview/fixtures/trade.json`, `route`): with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the result goes below the inputs (the Workbench's `min`). Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/trade-route/`, single hop and loop. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/trade/{View,Route}.svelte` in `PAGE_SIZING` (width is on today; font size joins it), and the `trade` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: the eight cases in `trade/Route.browser.test.ts` keep their behaviour on the new markup. No test of layout or copy.
- [x] `docs/guide/trade.md`'s Route section describes the result.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Trade > Commodities' result side. The inputs, the filters and the Places links above them stay. No core change.

## Context

There is no mockup; the stops and legs follow the route sidebar's timeline (`app/src/modules/route/Timeline.svelte`) in spirit, as a vertical list of stops with what happens between them, but on Void, not the frame's Hull. Replies: `TradeRoute.ts` (`here`, `route`), `TradeRouteResult.ts` (`kind`, `legs`, `profit`, `minutes`, `per_hour`, `warning`, `range`, `fetched_at`), `Leg.ts` (`from`, `to`, `ly`, `jumps`, `cargo`, `profit`, `profit_per_t`, `source`, `seconds`), `Cargo.ts`, `TradeRouteStop.ts` (`pad`, `arrival_ls`).

## Report

A found trade route now reads as the trip: the estimated profit per hour leads, then the stops top
to bottom with each leg between them. Before, each leg packed both stations, both systems, the
cargo and the figures onto one row of prose fragments.

**What changed** (`app/src/modules/trade/Route.svelte`, `View.svelte`; the inputs are as they were):

- **Totals first.** `per_hour` is the page's main value: Value XL with `.main` (Orange Lit), then
  "cr an hour, estimated" in Label. Its rich Tip holds the model: "Estimated: Spansh gives no profit
  per hour", `ROUTE_ESTIMATE` with the range per jump, and, on a single hop, that a round counts the
  flight back empty. Under it are two Label lines. The first is "128,800 cr a hop · 7 min there and
  back", or "347,400 cr a round · 7 min a round" for a loop. The second is the source and the age:
  "Spansh and Ardent Insight, 32 d ago", where each leg's `source` is named once. The `h4` ("Loop
  from Sol") is gone, because the kind is the input on the left.
- **The warning** (`route.warning`) is the orange "!" line between the totals and the stops, `role="alert"` as before.
- **Stops and legs** form one subgrid list (`ol.trip`). The first column is the spine, a Seam line
  with a Label Grey ring at each stop. The stops are `[legs[0].from, ...legs.map(l => l.to)]`, so a
  loop draws its first stop once more at the end. A stop shows its station in Name (White for the
  first, Options Grey after that). Under it, in Label, come the system with its copy icon, the pad
  letter and the arrival distance. A part that is unknown is left out (the single hop's first stop
  has no pad in the recording). A leg shows each commodity as "200 t Biowaste" (Figures, Prose
  Grey) with "buy 110 · sell 754" in Label. Its figures column is right-aligned (The Tabular Rule):
  the profit in Value L, Options Grey, with "cr" as `.type-unit`, then the profit per tonne, then
  jumps and light years, both in Label. A leg with no cargo says "Fly back empty" in Label and
  shows no profit. The cargo column is capped at 24rem (`minmax(0, 24rem)`), so the leg's profit
  sits next to its cargo and not 1,000 px away at 2560.
- **Pad too small.** This is the new shared `app/src/lib/ui/PadLetter.svelte` (its own commit).
  It is DESIGN.md's outlined letter. When it is too small, it is Red Low and struck through, and its
  accessible name says why ("Small pad: too small for your ship"). "The ship" is the pad the core
  measured its `warning` against. The core resolves the current ship's pad on every search unless
  the commander set one (`filters::resolve`). So the page takes it from the reload of the filters
  that each search already makes (`followShip`), falls back to the filters it asked with, and keeps
  it with the answer (`need`). A later edit or ship swap then doesn't re-mark an old answer. My
  first version used the pad the page loaded with, which was wrong after a ship swap. A ninth test,
  written failing first, covers that ("measures a stop's pad against the ship flown at the search"). SearchKit and Now's `Sale.svelte`
  still have their own copies of the badge. Moving them would touch SearchKit's busy-state greying,
  so I left them for a follow-up item (to file from a backlog branch: "Pad letter: SearchKit and
  Now's Sale use PadLetter.svelte").
- **States.** Before a search, it shows today's one Label line. Asking Spansh is one Label line.
  A failure is `Failure` with TRY AGAIN. No route is the one Body line saying what to relax.
- **Lint.** `font-size` joins `width` for `trade/{View,Route}.svelte` in `PAGE_SIZING`, and the
  trade entry is off `TOKENS_EXEMPT`. For that, the inputs' labels and the Places line in
  `View.svelte` name `.type-label` in place of their own font sizes. The label column is
  `calc(5.5 * var(--fs-s))`, the same 5.5em at Label size as OriginInput's, so the cells line up
  with From as before. Two things did move. The Places line takes Label's 1.3 line height, so the
  inputs under it sit 2 px higher. The `.tools` gap is on the scale now (`--sp-3 --sp-4`, from 10
  and 18 px).

**Tests.** The eight cases in `Route.browser.test.ts` keep their behaviour, and there is the ninth above. Only the strings they
read changed to the new markup, and the pad is asserted through its accessible name. The loop
case also checks that the first station is drawn twice. The age is matched as `/^Spansh, .* ago$/`,
so the test doesn't depend on the wall clock. `pnpm check`, `pnpm lint` and `pnpm test` (898 tests)
pass, and so does `python3 scripts/check-guide.py`. The 7 journeys pass with
`ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core`, the main
checkout's debug core, as Station did. This worktree has no core build, and there is no core change
(`git diff` against the main checkout's head for `crates app/src-tauri Cargo.lock` is empty).

**Browser check.** `docs/design/trade-route/plan.json` runs at 1100 x 700 and 1920 x 1080 at 100
percent and 2560 x 1440 at 145 percent, on a fixed clock. It covers before a search, the recorded
single hop and loop, each scrolled to its stops at 1100, asking (`slow=never:trade_route`),
failed (`fail=trade_route`) and no route (`core=empty`, with the station, cargo and credits typed).
`report.json` is measured on the committed head `1fb62535`, and all 20 captures list nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or
`contrast`. With the route followed, the sidebar is docked at 1920 and 2560 and is the strip at
1100, where the result goes below the inputs. No route runs on `core=empty`, which has no route,
so its 1920 and 2560 captures show the rail. The before captures are in `before/`
(`plan-before.json`): single hop and loop at 1100 and 2560, and each scrolled to the result at
1100, where the old result was below the inputs. I took them with the Start commit's
`Route.svelte` and `View.svelte` restored for the run, so `before/report.json`'s `commit` names a
later commit than the code it measured. The per-hour Tip was checked in the preview by focusing
the figure. It shows the title, the model with "at 31.5 ly a jump", and the single hop's note.

**`bright`, the sidebar apart** (loop, 1100, scrolled to the stops). It is sorted by luminance, so
it leads with White: the wordmark and the first stop's name, which the item makes White. Next come
the Options Grey names and the leg profits in Value L. Those are bright by their role, but they are
grey and smaller than the main value. Then come the Prose Grey commodities and pad letters. Among
the orange, Orange Lit holds only 2,872,855 (the main value) and the route strip's next stop.
Orange holds the frame's own (Help, Ctrl+K, Maia). In the unscrolled captures, the cells, the
Loop choice, CHANGE and the one fill (FIND ROUTE) are what can be changed. Nothing needs the
commander in the recordings, and no colour is outside DESIGN.md's list.

**Not in the recordings.** The warning, a pad too small and an empty way back are not in
`trade.json`, so I did not invent fixtures for them. The tests cover them (the warning and the
struck pad by its accessible name, and "Fly back empty"), and the captures do not show them.

