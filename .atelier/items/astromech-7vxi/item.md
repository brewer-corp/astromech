---
schema: atelier.item
schema_version: 3
id: astromech-7vxi
title: 'Station: the docked station''s market and outfitting read as answers'
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-8e22
- astromech-coq6
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:44.748586055Z
---

## Goal

Docked, the Station page answers "is it worth selling here, and does it sell anything better for my ship?" at a glance. Today it is a fact list, two market tables with a column per field, an upgrades table and a search table, all in one column (`app/src/modules/station/View.svelte`, `Market.svelte`). The services board, which the first plan put here, is on the Now page (astromech-coq6): the Station page is what is left of the station once Now has shown its services, read as answers.

## Done when

- [ ] Station, under NOW since astromech-coq6, is built on the Board layout and the type roles. It leads with the shared station head (`app/src/modules/station/Head.svelte`, from astromech-coq6), then one compact fact list in Label and Figures: the controlling faction and its state, government and allegiance, economy with its shares, landing pads with the largest, and the external links (`ExtLinks`) for the station and the system. The services badges go, since the board on Now shows them; any service the station reports that the board does not draw (engineer, Powerplay contact, tuning, carrier services and the like) stays here as one Label line, "Also: …", so none is lost.
- [ ] **Your cargo here.** Each sellable commodity aboard (`market_compare`'s `carried`) is a two-line result row in groups, as DESIGN.md's Lookup result row: the commodity and tonnes; "Here" what this station pays over "Best nearby" what the best station pays; the gain per tonne in Value L, Options Grey (the reply lists the hold in load order, most tonnes first, so the gain does not rank it), except the first line's, which is the page's main value in Orange Lit; the station that pays it, its system with copy, and jumps, distance and the report's age in the trip group. A line not looked up says so in Label with its Tip; a line no station takes in full says so.
- [ ] **For sale here** reads the same way from `for_sale`: the commodity with its stock, the price here over the best nearby price, the profit per tonne as the ranked value in Value L Orange (the reply lists them biggest gain first), and where.
- [ ] The market's age ("Market as of 10:42, 12 min ago") and Ardent's age stay as a Label line beside CHECK AGAIN, a text action. Left-out cargo (`kept`) is one Label line.
- [ ] **Outfitting for the ship flown**: each upgrade (`DockedHere.outfitting.upgrades`) a one-line row: slot name in Label, fitted module in Prose Grey, the module on sale in Name Options Grey, the price in Figures (the upgrades are not ranked by price). With none, the one Label line it has today. With no outfitting seen yet, the one prose line it has today.
- [ ] **Where did I see…** keeps its search box and the answer as two-line result rows: module name; station and system with copy; the price in Figures; seen and distance in the trip group (the core orders them newest seen first, `crates/astromech-station/src/book.rs:206`, so no orange column). Its empty, nothing-kept and no-match lines stay.
- [ ] States: not docked shows the current system with its links and one Label line, as today; loading one Label Grey line; a failed poll keeps the last answer with the Stale line, or says so with no action, since it re-reads every 5 s; a failed `market_compare` is the shared Failure line with CHECK AGAIN in the market section only.
- [ ] The brightest things are the gain per tonne on the hold's first line (the page's main value, Orange Lit), CHECK AGAIN and the search box (what can be changed), and nothing else needs the commander. Orange only for the main value, For sale here's ranked profit, actions and links; read-only data white and grey. No colour outside DESIGN.md's list.
- [ ] No printed explanations: "Best nearby" keeps its rule in a Tip (`BEST` in `Market.svelte`).
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, docked (the recorded `station` fixture) and not docked: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100; nothing clipped, nothing scrolls sideways, no action more than one column from its row. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/station/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/station/{View,Market}.svelte` in `PAGE_SIZING`, and the `station` entry leaves `TOKENS_EXEMPT` (`app/eslint.config.js`).
- [ ] The tests in `station/View.browser.test.ts` and `Market.browser.test.ts` keep their cases and pass on the new markup; none is added for layout. `docs/guide/now.md`'s Station section (moved there by astromech-coq6) describes the page.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

The Station page's body. The services board and the station head are astromech-coq6's. No core change: `docked_here`, `market_compare` and `outfitting_search` give everything above.

## Context

Mockup: the station head of `docs/design/glow-up-now.png` (the board below it is Now's). Rows follow DESIGN.md revision 5's Lookup result row and The Aligned Columns Rule; compare `docs/design/grouped-rows/`. Replies: `DockedHere.ts`, `MarketComparison.ts` and `MarketLine.ts` (`carried`, `sell_here`, `buy_here`, `stock`, `best: Offer`, `gain`), `Outfitting.ts`. `market_compare` asks Ardent for at most `MAX_LOOKUPS` goods per dock, cached 15 minutes (`crates/astromech-market/src/here.rs`), and asks again only on a new dock or CHECK AGAIN; that stays.
