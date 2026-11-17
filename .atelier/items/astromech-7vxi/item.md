---
schema: atelier.item
schema_version: 3
id: astromech-7vxi
title: 'Station: the docked station''s market and outfitting read as answers'
status: in_review
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-8e22
- astromech-coq6
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A7ca8a4c4-e671-4665-aa58-7951c9c97a4c
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:44.748586055Z
---

## Goal

Docked, the Station page answers "is it worth selling here, and does it sell anything better for my ship?" at a glance. Today it is a fact list, two market tables with a column per field, an upgrades table and a search table, all in one column (`app/src/modules/station/View.svelte`, `Market.svelte`). The services board, which the first plan put here, is on the Now page (astromech-coq6): the Station page is what is left of the station once Now has shown its services, read as answers.

## Done when

- [x] Station, under NOW since astromech-coq6, is built on the Board layout and the type roles. It leads with the shared station head (`app/src/modules/station/Head.svelte`, from astromech-coq6), then one compact fact list in Label and Figures: the controlling faction and its state, government and allegiance, economy with its shares, landing pads with the largest, and the external links (`ExtLinks`) for the station and the system. The services badges go, since the board on Now shows them; any service the station reports that the board does not draw (engineer, Powerplay contact, tuning, carrier services and the like) stays here as one Label line, "Also: …", so none is lost.
- [x] **Your cargo here.** Each sellable commodity aboard (`market_compare`'s `carried`) is a two-line result row in groups, as DESIGN.md's Lookup result row: the commodity and tonnes; "Here" what this station pays over "Best nearby" what the best station pays; the gain per tonne in Value L, Options Grey (the reply lists the hold in load order, most tonnes first, so the gain does not rank it), except the first line's, which is the page's main value in Orange Lit; the station that pays it, its system with copy, and jumps, distance and the report's age in the trip group. A line not looked up says so in Label with its Tip; a line no station takes in full says so.
- [x] **For sale here** reads the same way from `for_sale`: the commodity with its stock, the price here over the best nearby price, the profit per tonne as the ranked value in Value L Orange (the reply lists them biggest gain first), and where.
- [x] The market's age ("Market as of 10:42, 12 min ago") and Ardent's age stay as a Label line beside CHECK AGAIN, a text action. Left-out cargo (`kept`) is one Label line.
- [x] **Outfitting for the ship flown**: each upgrade (`DockedHere.outfitting.upgrades`) a one-line row: slot name in Label, fitted module in Prose Grey, the module on sale in Name Options Grey, the price in Figures (the upgrades are not ranked by price). With none, the one Label line it has today. With no outfitting seen yet, the one prose line it has today.
- [x] **Where did I see…** keeps its search box and the answer as two-line result rows: module name; station and system with copy; the price in Figures; seen and distance in the trip group (the core orders them newest seen first, `crates/astromech-station/src/book.rs:206`, so no orange column). Its empty, nothing-kept and no-match lines stay.
- [x] States: not docked shows the current system with its links and one Label line, as today; loading one Label Grey line; a failed poll keeps the last answer with the Stale line, or says so with no action, since it re-reads every 5 s; a failed `market_compare` is the shared Failure line with CHECK AGAIN in the market section only.
- [x] The brightest things are the gain per tonne on the hold's first line (the page's main value, Orange Lit), CHECK AGAIN and the search box (what can be changed), and nothing else needs the commander. Orange only for the main value, For sale here's ranked profit, actions and links; read-only data white and grey. No colour outside DESIGN.md's list.
- [x] No printed explanations: "Best nearby" keeps its rule in a Tip (`BEST` in `Market.svelte`).
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, docked (the recorded `station` fixture) and not docked: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100; nothing clipped, nothing scrolls sideways, no action more than one column from its row. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/station/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/station/{View,Market}.svelte` in `PAGE_SIZING`, and the `station` entry leaves `TOKENS_EXEMPT` (`app/eslint.config.js`).
- [x] The tests in `station/View.browser.test.ts` and `Market.browser.test.ts` keep their cases and pass on the new markup; none is added for layout. `docs/guide/now.md`'s Station section (moved there by astromech-coq6) describes the page.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

The Station page's body. The services board and the station head are astromech-coq6's. No core change: `docked_here`, `market_compare` and `outfitting_search` give everything above.

## Context

Mockup: the station head of `docs/design/glow-up-now.png` (the board below it is Now's). Rows follow DESIGN.md revision 5's Lookup result row and The Aligned Columns Rule; compare `docs/design/grouped-rows/`. Replies: `DockedHere.ts`, `MarketComparison.ts` and `MarketLine.ts` (`carried`, `sell_here`, `buy_here`, `stock`, `best: Offer`, `gain`), `Outfitting.ts`. `market_compare` asks Ardent for at most `MAX_LOOKUPS` goods per dock, cached 15 minutes (`crates/astromech-market/src/here.rs`), and asks again only on a new dock or CHECK AGAIN; that stays.

## Report

The Station page now answers "is it worth selling here?" first: under the shared station head and
a five-line fact list, the hold's rows lead with the first commodity's gain per tonne (+3,921 cr/t
on the fixture) in Orange Lit, and For sale here ranks its profit in Orange. Market, upgrades and
the outfitting search are DESIGN.md's Lookup result rows instead of tables with a column per field.

**What changed** (`app/src/modules/station/View.svelte`, `Market.svelte`):

- On `Board`, led by `Head.svelte`. Facts in Label and Figures: faction in its state, government
  and allegiance, economy shares, pads with the largest, and Links (Spansh and Inara for the
  station, then the system). The service badges are gone. "Also: …" lists every reported service
  not in `MENU_SERVICES`, derived from that list, so the fixture reads "Also: Docking, Autodock,
  Contacts, Tuning, Engineer". Docking is kept: "none is lost" is the rule, and leaving one out
  would be a choice the item didn't make.
- Your cargo and For sale here sit on one grid with subgrid rows, so the two lists line up
  (The Aligned Columns Rule). The groups are: commodity over tonnes (or stock); price here over
  best nearby (`BEST` in the head's Tip); gain or profit in Value L; station type icon and name
  over the system with copy; trip (jumps over light years and the report's age). Only the place
  track gives way, with an ellipsis. A row with no best price says why across the place and trip
  ("Not looked up" with its Tip, "No station nearby takes all of it", "No station nearby buys it").
- The market line reads "Market as of 2026-10-16 20:30 UTC, 32 d ago · Ardent Insight, asked 32 d
  ago" with CHECK AGAIN right after it. I kept the date because a market can be days old. Left out
  and Not looked up are one Label line each. Asking again keeps the old rows Grey Dim. A failed
  `market_compare` is `Failure` ("Could not compare the market: …") with CHECK AGAIN, in the
  market section only.
- Staleness follows DESIGN.md's list, which is binding although the item doesn't name it: a
  report older than 30 days as of Ardent's `fetched_at` shows its age in Orange, and the gain
  steps down to Orange Mid. The fixture has no stale row.
- Upgrades are one-line rows: slot in Label, fitted in Figures (Prose Grey), the better module in
  Name, price in Figures. A Label line above says how many modules were for sale and when they
  were seen. The none and not-seen lines are as before.
- Where did I see… keeps its box and lines. Its answer is two-line rows: the module; the station
  over the system with copy; the price; seen over distance. Nothing is orange.
- States: not docked shows the system as a Title with copy, its links and the Label line. Loading
  is "Reading the journal…". A failed first read is the error with no action. A later failure
  keeps the page with the `Stale` line.
- `eslint.config.js`: both page-sizing parts are on for `station/{View,Market}.svelte`, and the
  station entry is off `TOKENS_EXEMPT`.

**Checks.** `pnpm check`, `pnpm lint` and `pnpm test` (821 tests) pass, as does
`python3 scripts/check-guide.py`. The 7 journeys pass with
`ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core`, the main
checkout's debug core. This worktree has no core build, and I did not start a full Rust build on
this host. `git diff HEAD c1872bd6 -- crates app/src-tauri Cargo.lock` is empty, so that binary
is built from the same core code. The station tests keep all their cases. Only the expected
strings changed: cells are read with `innerText`, and the service-badge assertion is now the
"Also:" line.

**Browser check.** `docs/design/station/plan.json` covers ten states at 1100 x 700 and 1920 x
1080 at 100 percent and 2560 x 1440 at 145 percent:

- docked, and docked scrolled to the market, to For sale here and to outfitting;
- a search for "5a fsd";
- the market asking (`slow=never:market_compare`) and the market failed;
- the page loading and the page failed (`docked_here`);
- not docked.

Every capture lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `contrast` or
`far`. The route is the strip at 1100 and the docked sidebar at 1920 and 2560. The before
captures are in `docs/design/station/before/` (`plan-before.json`), taken unscrolled, so the old
market tables show only at 2560.

**`bright`, the sidebar apart** (docked 1100, scrolled to the market). It is sorted by luminance,
so it leads with White (the wordmark, the station title from `Head`), then the Options Grey names
and the Prose Grey figures, as DESIGN.md makes them. Among the orange, Orange Lit holds only
+3,921, the main value. Orange holds CHECK AGAIN, the For sale profits (the ranked value), the
Spansh and Inara links, and the frame's own (Help, Ctrl+K). The search box is the page's ember
cell. Nothing needs the commander. No colour outside DESIGN.md's list.

**Tool changes, each in its own commit.**

- `fe866009`: `audit.mjs` measured overlap on clipped boxes. At 1100 the bottom row's second
  line, cut to a sliver by the status bar, counted its ordinary leading as overlap (4 findings).
  The check now measures against the whole line boxes. The same rows show no overlap when
  scrolled into full view (`docked-for-sale-1100x700`).
- `c14ca954`: a plan can now `type` into an input, so the search shot is captured by the plan
  instead of checked by hand.
