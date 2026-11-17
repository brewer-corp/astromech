---
schema: atelier.item
schema_version: 3
id: astromech-nnn0
title: Ships > Suits as a loadout
status: approved
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-8e22
- astromech-w1lj
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Ac630ef7c-ddb2-43d7-bd7e-1029b654eca8
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.464459239Z
---

## Goal

A suit loadout reads as a ship's does: what the commander wears and carries in each slot, how far each item is upgraded and modified, what the plan adds, and what is still missing for it. Today Ships > Suits is On foot's blueprint picker moved in whole (astromech-5a01): a line of owned gear, picker cards and a shopping list table (`app/src/modules/suits/View.svelte`), with no loadouts, since the core did not keep them until astromech-w1lj.

## Done when

- [x] astromech-8og8 is fixed (taken over from the checkpoint): no action on Suits is more than one column from what it acts on, and the shopping list is as wide as its columns. The checkpoint's tool (`node scripts/browser/audit.mjs --base <dev server> --plan docs/design/increment2/plan.json --only suits` (the checkpoint's tool, from astromech-8e22)) does not flag this case under `far`, which measures only an action at a row's end, so the proof is the captures and a measurement: at 1920 x 1080 (100 percent), 2560 x 1440 (145 percent) and 3840 x 2160 (180 percent) the captures show each Remove beside its pick and the Need and Have columns next to the item names with the spare width empty, and the Report gives the gap from each action to the text it acts on, in layout px, none over 360 (the tool's `far` limit).
- [x] Ships > Suits is a List with detail page on the type roles, laid out as `docs/design/nav-spike/tone5-suits-2560.png`. Its head: the selected loadout's name as the Title, White, with "Loadout 1 of 4 · Maverick Suit · 3 of 4 items engineered or planned" in Label. The selected loadout is the worn one (`OdysseyView.loadouts`, astromech-w1lj) until the commander picks another; the choice is kept in the link (`#/suits?loadout=<id>`).
- [x] **Loadout**: one row per slot, Suit, Primary, Primary, Secondary: the slot in Label, the item's name in Name Options Grey, its grade as engineer hexagons (`Hexes.svelte`, `tone="grade"`, a planned grade lit to it in Orange Mid through `to`), its fitted modifications in Prose Grey ("None" in Label Grey), and the plan for it in Orange ("Grade 3 · Night Vision"). The plan for an item is the saved picks tied to it (`Pick.item` equal to the item's journal id, astromech-w1lj), and only those: a pick never shows on an item it is not tied to, so one Night Vision pick does not read as planned for every suit, and of two grade-1 Mavericks only the one with the pick shows it. An item worn in several loadouts shows its plan in each, and the shopping list counts it once, since the pick is per item, not per loadout. An empty slot says so in Label Grey.
- [x] Picks with no `item` (saved before this change) are tied to nothing: they show on no row and are listed under the shopping list as "Not tied to an item", each with a Dropdown of the owned items it fits, which ties it, and its remove action; they still count in the shopping list as one install each, as today. `pickKey` and `hasPick` (`app/src/lib/odyssey.ts:63`, `:94`) include `item`, so the same recipe for two items is two picks, not one deduplicated.
- [x] **Shopping list**: one list for every pick, as today's `Shopping.total`, need against have in two columns of Figures, a short line's have with an orange "!", in Name Options Grey for short and Prose Grey otherwise; its credits and "6 of 14 short" in the section's Label line. The bartender trade (`Shopping.trade`) stays under it as today, its rule in a Tip.
- [x] Owned suits and weapons in no loadout stay listed, as today's owned line, in one Label line under the strip ("Also owned: …" with grade and mods), so none is lost.
- [x] **Loadouts**: the other loadouts as a strip, each its name in Name, its suit and the suit's grade as hexagons in Label; the selected one raised, not filled.
- [x] Selecting a loadout row fills the side panel for that item: its name as the title, White; its grade hexagons and plan in Label; the cost of its planned upgrade as the panel's main value in Value XL Orange Lit ("2,850,000 cr to grade 3", from the pick's `PickResult.credits`, "at least" when `credits_partial`); what its picks need, item by item, need against have with the short ones marked; where an upgrade is bought ("Pioneer Supplies") and, for a modification, each engineer who fits it (`PickResult.fitted_by`: name, system, `unlocked`, `unconfirmed`) with their system's copy icon and Set destination, and Unlocked or not in Label, an unsure one marked "(?)" with its Tip as Engineers does. The mockup's "1,000,000 cr · 5 items, 4 short" for the engineer is not in the reply and is not shown.
- [x] Adding never doubles a plan. ADD TO PLAN on an item that already has a matching pick with no `item` (the same recipe, saved before this change) ties that pick to the item by its journal id and adds nothing; the same when PLAN OTHER GEAR would add a recipe already saved with no `item`. The pick-to-item table test covers it: a Maverick 1 to 3 pick with no `item` in localStorage, ADD TO PLAN on one grade-1 Maverick, then one pick in the list, costed once, that Maverick's row showing the plan and the other Maverick's row unchanged.
- [x] The picker stays reachable: ADD TO PLAN in the panel adds, tied to the selected item by its journal id, the next grade of that item, or a modification chosen from a dropdown of those it can take, as the picker does today (`ownedGrades`, `fromGrades`, mods already fitted left out); the rest of today's picker (a pick for owned gear in no loadout, tied to that item, or a recipe for gear not owned, tied to nothing) is reached from a text action, PLAN OTHER GEAR, in the head, opening the current picker's controls in the panel. Removing a pick stays on the pick. Picks stay in localStorage as today (`app/src/lib/odyssey.ts:87-91`).
- [x] States: no loadout known yet says so in one prose line and still shows the shopping list and the picker; loading one Label Grey line; a failed `odyssey_view` or `odyssey_shopping` keeps the last answer with the Stale line, or shows the shared Failure line with TRY AGAIN; a saved pick the game data no longer has keeps its removable line, as today.
- [x] The brightest things: the panel's cost to the planned grade (main value), ADD TO PLAN and the picks (what can be changed), and the short items' "!" (what needs the commander). Orange for the plan, actions and links only; no colour outside DESIGN.md's list.
- [x] No printed explanations; the sources of grades and mods ("from the journal") and the bartender rule go in Tips.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the preview odyssey world's four loadouts: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the panel goes below the loadout and the shopping list's two columns become one. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/suits/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/suits/**` in `PAGE_SIZING`, and the `suits` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: the cases in `suits/View.browser.test.ts` keep their behaviour on the new markup; new browser cases, on the recorded `odyssey.json`, cover picking another loadout filling the loadout rows, selecting the Maverick Suit filling the panel from the sample picks' shopping (the preview answers `odyssey_shopping` only for exactly the recorded picks, Maverick 1 to 3, Night Vision and Karma P-15 1 to 2, `app/src/preview/shell.ts:433`), and ADD TO PLAN on the second grade-1 Maverick sending a pick tied to that item's journal id to `odyssey_shopping` (what reached the backend), with that row showing it as planned and the first Maverick's row unchanged. The pick-to-item match is a pure function with a table test: two items of one recipe with a pick for one; one item in two loadouts; a pick with no `item`; a mod pick that fits but is tied elsewhere. No test of layout or copy.
- [x] `docs/guide/ships.md`'s Suits section describes loadouts, the panel and where the picker went.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Ships > Suits. The loadouts come from astromech-w1lj; suit engineers' distances from astromech-70ts are not needed here (the panel names the engineer and their system). The inventory is Materials' On foot kind and the suit engineers' tree is Engineers', both since astromech-5a01.

## Context

Mockup: `docs/design/nav-spike/tone5-suits-2560.png`, approved in revision 5's colours; follow it for layout and colour. It draws the loadout as a table with column heads; read them as the row's groups under DESIGN.md's Aligned Columns Rule, with no column per field beyond those.

Replies: `OdysseyView.ts` (`owned`, `gear`, `mods`, `engineers`, and `loadouts` from astromech-w1lj), `Shopping.ts` (`picks: PickResult[]`, `total: OdysseyNeed[]`, `credits`, `trade: BarterPlan`), `Pick.ts` (an upgrade `{gear, from, to}` or a mod `{id, maker}`, by recipe today, with no owned-item identity, which is why astromech-w1lj adds `item`; plan review round 4, finding 4-pi08.2). The browser preview answers `odyssey_shopping` only for the sample picks (`app/src/lib/odyssey.ts:4`).

## Report

Ships > Suits is now a List with detail page laid out as `tone5-suits-2560.png`: the selected
loadout slot by slot, one shopping list in two columns, the other loadouts as a strip, and a side
panel for the selected item. Code: `app/src/modules/suits/View.svelte`, the pick-to-item rules in
`app/src/lib/suits.ts` (table test `suits.test.ts`), guide in `docs/guide/ships.md#suits`.
Commit 5329e3d0.

### Decisions the item left open (advice for the lead; I carried on with these)

- **One upgrade pick per item.** ADD TO PLAN offers every grade above the planned one (default
  the next) and every mod the item can still take. A higher grade raises the item's existing
  upgrade pick's `to` rather than adding a second pick, so the panel's main value is one figure
  "to grade N" and the row says "Grade N". The pick's `from` is the item's own grade.
- **Picks tied to an item no longer owned** (sold) are listed with the untied ones under the
  shopping list, so they stay removable and can be tied again. The spec named only picks with no
  `item`.
- **No default selected row** (The One Fill Per Page Rule): the panel opens when a slot is
  clicked. Switching loadout closes the panel unless the new loadout has the same item.
- **"Also owned" names are links** that open the item in the panel, so a plan made for gear in no
  loadout (through PLAN OTHER GEAR) has somewhere to show and be removed. Its plan follows its
  name in Orange.
- **Shopping list order:** short lines first, then the rest, as the mockup draws it. The old
  Short and Barter-each columns are gone (the item asks for need against have); the barter total
  moved to a Label line under Bartender trade.
- The loadout strip is a `nav aria-label="Your loadouts"`, as the Ships page's rail is: it picks
  a thing, and the Two Tab Rows test counted it as a third tab row otherwise.
- `pickKey` and `hasPick` already included `item` on main (astromech-w1lj,
  `app/src/lib/odyssey.ts:63`); nothing to change there.

### astromech-8og8: gaps from each action to its text

Measured in layout px with the suit selected (sample picks) and with a pick tied to nothing,
by a puppeteer script on the dev preview (the audit's `far` measures only row-end actions). The
same at all three sizes, since every width is in rem:

| Action | 1920 x 1080 (100) | 2560 x 1440 (145) | 3840 x 2160 (180) |
|---|---|---|---|
| Remove after "Grades 2 and 3 need" | 115 | 115 | 115 |
| Remove after "Night Vision" | 170 | 170 | 170 |
| ADD TO PLAN after its dropdown | 8 | 8 | 8 |
| Set destination after the engineer's system | 44 | 44 | 44 |
| PLAN OTHER GEAR after the title | 24 | 24 | 24 |
| Tie dropdown after an untied pick's name | 16 | 16 | 16 |
| Remove after the Tie dropdown | 16 | 16 | 16 |
| Shopping list, widest gap from a name to its Have | 302 | 302 | 302 |

None is over 360. The shopping list's two columns are 768 px wide together, against a page
column of 1,446 to 1,813 px, and the spare width is empty: see `suits-suit-selected-1920x1080.png`,
`-2560x1440.png` and `-3840x2160.png` in `docs/design/suits/` (before: `docs/design/increment2/`).
The side panel keeps its 320 px width when it goes below the loadout at 1100, so its Remove stays
beside the pick there too.

### Browser check

`docs/design/suits/plan.json` captures three states (at rest, the Maverick Suit selected, PLAN
OTHER GEAR open) at 1100 x 700 and 1920 x 1080 at 100 percent, 2560 x 1440 at 145 and
3840 x 2160 at 180, with `route=exact`. All twelve list nothing under `sideways`, `clipped`,
`overlaps`, `small`, `contrast` or `far`. The sidebar is docked at 1920, 2560 and 3840 and is the
strip at 1100, where the panel goes below the loadout and the shopping list is one column.
Before captures at 1100 and 2560 are in `docs/design/suits/before/` (plan `before.json`).

`bright` ranks by luminance, so its top entries are the White title and selected name and the
Options Grey item names; that is the type scale, not a fault. Of the colours, the page's only
Orange Lit text is the panel's main value (2,850,000); Orange carries ADD TO PLAN (the one fill),
the plans on the rows, Remove, PLAN OTHER GEAR and the short lines' "!"; nothing else is orange.

### Checks

`pnpm check`, `pnpm lint` and `pnpm test` (775 tests) pass. `node --test 'journeys/*.journey.ts'`
passes (7) with `ASTROMECH_CORE` set to a debug core built at 06:04 today in another worktree
whose `crates/` and `Cargo.lock` are identical to this branch's (this branch changes no Rust), to
avoid a cold cargo build on this host. The previewstates table now expects the page's own empty,
loading and failure words.

### Responses

1-l936.1: fixed in 4e078e97: addPick uses up the matching untied pick and combines it with the item's own upgrade into one pick, with a table case for that combination.
1-l936.2: fixed in 4e078e97: combining upgrades keeps the higher target; a table case ties a 1 to 3 pick to an item planned to grade 5 and keeps one 1 to 5 pick.
1-l936.3: fixed in 4e078e97: the panel is reconciled in an effect on the loadout shown (click, Back, or a new worn loadout), keeping only an item that loadout has or one in no loadout; a browser case goes Back from Spare's Maverick to Explore.
