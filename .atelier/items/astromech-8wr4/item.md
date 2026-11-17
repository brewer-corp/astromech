---
schema: atelier.item
schema_version: 3
id: astromech-8wr4
title: Ships > Ships as a loadout
status: in_review
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-8e22
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A600a29ef-288f-45ad-b2fa-4bc787ff94fe
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:41.140290178Z
---

## Goal

A ship reads as its loadout, the way the game's outfitting screen lists it: what is fitted in each slot, how far each module is engineered, and what its next grade needs. Today the Info view is a ship cartoon and stat boxes that are mostly dashes (`docs/design/cells-ships-after.png`), and the modules are seen only inside the build planner's form (`app/src/modules/engineering/Build.svelte`). The plan stays: the loadout is where it is made, and the plan's totals keep a view of their own.

## Done when

- [x] astromech-5g9w is fixed (taken over from the checkpoint): Open in Coriolis and EDSY sit within one column of the ship's name, and no band stretches across the page column. The checkpoint's tool (`node scripts/browser/audit.mjs --base <dev server> --plan docs/design/increment2/plan.json --only ships` (the checkpoint's tool, from astromech-8e22)) does not flag this case under `far`, which measures only an action at a row's end, so the proof is the captures and a measurement: at 1920 x 1080 (100 percent), 2560 x 1440 (145 percent) and 3840 x 2160 (180 percent) the captures show the links beside the head and the hero values, loadout groups and fleet strip at their own widths with the spare width empty, and the Report gives the gap from Open in to the ship's name, in layout px, no more than 360 (the tool's `far` limit).
- [x] Ships > Ships, for the selected ship (`engineering_fleet`, `#/engineering/<id>` as today), is a List with detail page on the type roles. Its head: the ship's name as the Title, White; type, ident and "N of M modules engineered" in Label; and a row of up to four hero values in Value XL with units in Label, the jump range in Orange Lit as the page's main value while no module is selected and the others Options Grey: jump range, shield strength, top speed and power used of capacity, from `engineering_stats`'s current stats (`Compared.current`, as `Specs.svelte` reads them). A figure the calculations leave out shows "–" with its Tip saying why, as Info does.
- [x] The loadout lists every fitted module (`Ship.modules`) in four groups headed as the game's outfitting is: Hardpoints, Utility, Core, Optional. The group comes from the slot name (`Fitted.slot`): `*Hardpoint*` except `TinyHardpoint*` is Hardpoints, `TinyHardpoint*` Utility, `Armour`, `PowerPlant`, `MainEngines`, `FrameShiftDrive`, `LifeSupport`, `PowerDistributor`, `Radar` and `FuelTank` Core, `Slot*` Optional; cosmetic and fixed internals (`PlanetaryApproachSuite`, `ShipCockpit`, `CargoHatch`, paint, decals and the rest) are not listed. The mapping is one function with a table test over the slot names in `app/src/preview/fixtures/ships.json`.
- [x] Each module is one row: class and rating as the module name gives them ("7A") in Figures, the module name in Name Options Grey, its grade as engineer hexagons (`Hexes.svelte`, `tone="grade"`, the planned grade lit to it in Orange Mid through `to`), then the blueprint name and the experimental effect in Prose Grey. A module not engineered shows no hexagons; one that cannot be engineered (`group: null`) says nothing more.
- [x] Selecting a module raises its row (Hull Raised, lit text) and fills the side panel: the module name as its title, White; its blueprint and grade now (`Fitted.engineering`); the plan's target for it if the ship's plan has one, with that target's materials as need against have and the short ones marked with an orange "!" (`engineering_plan`'s `TargetCost.materials` and `PlanResult.materials`), and the engineers who apply its top grade (`TargetCost.engineers`). The plan's blueprint, grade and experimental dropdowns for that module move from `Build.svelte`'s rows into the panel; ADD TO PLAN, the primary button, adds a target one grade up on the module's current blueprint, or opens the blueprint dropdown for a module not yet engineered. ENGINEERS, a text action, opens Engineering > Engineers searched for the module's group (`#/unlocks?q=<group name>`; a full catalog group name matches, astromech-7did). With nothing selected the panel shows nothing.
- [x] The rest of the fleet is a strip under the loadout, as today's rail: each ship's cartoon (`ShipIcon`), name in Name and type and where it is in Label; the current ship first, marked "Here" in Label, raised when selected. Selecting one changes `#/engineering/<id>`.
- [x] The Info and Engineering choice becomes Loadout and Plan (`#/engineering/<id>` and `#/engineering/<id>/plan`, so old links keep opening the same views). Plan keeps what `Build.svelte` shows below its module rows today: the stats now against the plan done, the shopping list, the trades, what is still to collect and the engineer trip with Follow, on the List with detail layout with the material panel it has today. Nothing it does today is dropped.
- [x] The Coriolis and EDSY links stay as text actions in the head, and the stale-loadout note stays as one Label line with its Tip. A stored ship's system and station links stay in the head's Label line.
- [x] States: no fleet yet (`?core=empty`) one prose line; reading the journal one Label Grey line; a failed fleet poll keeps the last answer with the Stale line; the stats failing says so beside the hero row with the headline failure as today; a ship never flown while a journal was kept says its modules are unknown and shows no loadout.
- [x] The brightest things: the selected module's planned grade and the panel's need (main value; in the preview that is a module of the sample plan), ADD TO PLAN and the dropdowns (what can be changed), and a short material's "!" (what needs the commander). With nothing selected the page's main value is the jump range. No colour outside DESIGN.md's list: hexagons as its Engineer rank and blueprint grade entry.
- [x] No printed explanations; what a figure counts goes in a Tip. The Specs and Stats boxes are gone; the figures they held that are not in the hero row are kept: speed, boost, DPS and power use are already in the Plan view's stats now against plan (`Stats.svelte`); pitch, roll and yaw (the hull stand-in, with its Tip) and power retracted and deployed join that list; value and rebuy join the head's Label line.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, on the sample Krait (the only preview ship with recorded stats and a plan) and on the Anaconda with no stats: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. At 1100 x 700 the panel goes below the loadout, which keeps its four groups in one column. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/ship-loadout/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/engineering/{View,Build,Specs,Stats}.svelte` (or what replaces them) in `PAGE_SIZING`, and the `engineering` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: the cases in `engineering/View.browser.test.ts` and `Build.browser.test.ts` keep their behaviour on the new markup (Info's become the loadout's, Engineering's the Plan's); new browser cases cover selecting a module filling the panel from the sample plan's target (ship 7's `FrameShiftDrive`, a target of the recorded plan in `app/src/preview/fixtures/engineering.json`, `plan_input`; the preview answers `engineering_plan` only for that ship, `app/src/preview/shell.ts:435`), and ADD TO PLAN sending `engineering_plan` a plan with the new target (what reached the backend) and the row then showing the planned grade. The slot grouping has its table test. No test of layout or copy.
- [x] `docs/guide/ships.md` (A ship's page, Build planner, Ship stats) describes the loadout, the panel and the Plan view.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Ships > Ships only. Stored modules move onto a layout in astromech-gvy0; Suits is astromech-nnn0. No core change: the fleet, stats and plan replies give everything above.

## Context

Mockup: `docs/design/glow-up-ship.png`. Its layout stands; under revision 5 its rows are not filled cells (read-only rows on Void, the selected one Hull Raised, not filled orange), its grades are hexagons, not squares, the fleet strip's current ship is raised, not filled, and module names are Options Grey, not orange.

Replies: `Ship.ts` (`modules: Fitted[]`, `links`, `loadout_ts`), `Fitted.ts` (`slot`, `slot_name`, `item`, `name`, `group`, `engineering: Applied`), `Applied.ts` (`blueprint_name`, `grade`, `experimental_name`), `PlanResult.ts` and `TargetCost.ts` (per target: `materials`, `engineers`, `steps`). The plan is kept per ship in localStorage (`app/src/lib/engineering.ts:137-155`). The browser preview has no planner (`engineering.ts:79`): only the sample ship's sample plan evaluates, so the panel's needs show there for the sample plan's modules; in the app any plan evaluates. The ship stats are worked out by the core; in the preview only the sample Krait has them (`app/src/preview/fixtures/ships.json`).

## Report

Ships > Ships now reads as the ship's loadout. The Info and Engineering choice is Loadout and Plan (`#/engineering/<id>` and `#/engineering/<id>/plan`). The Loadout lists every fitted module under Hardpoints, Utility, Core and Optional, with grades as engineer hexagons. Selecting a module fills the side panel with its target, materials, engineers, the plan's three dropdowns, ADD TO PLAN and ENGINEERS. The Plan view keeps the stats now against the plan, the rolls, the shopping list, trades, what is still to collect and the engineer trip, with the material panel. Commits: 9253a114 (the page), 51deb967 (audit tool), 651761d4 (panel order, palette), 9c27f6eb (captures and report).

**Read first: the mockup's two columns of groups show only on wide windows.** One group of rows is 672 layout px (class 40, name 240, grade 76, blueprint 256, gaps and padding). Two side by side beside the panel need about 1,785 px of page column. The column count is worked out as if the panel were there, so selecting a module never reflows the rows. With the docked sidebar that never happens at 1920 x 1080, 2560 x 1440 at 145 or 3840 x 2160 at 180: all show one column at its own width with the spare width empty. `krait-module-wide-no-route-2400x1350.png` (no route, 100 percent) shows the two-column branch working. Making two columns appear at the standard sizes would mean narrower rows (the blueprint column wrapping), which I judged worse than one column.

**astromech-5g9w.** Open in Coriolis and EDSY sit on the title row, right after the ship's name. The new `gap` step in `scripts/browser/audit.mjs` measures it once the page is quiet: 24 layout px from the name's right edge to "Open in" at 1100 x 700, 1920 x 1080, 2560 x 1440 (145) and 3840 x 2160 (180), and at 2400 x 1350 (`openGap` in `docs/design/ship-loadout/report.json`). The limit is 360. The hero values, loadout groups, fleet strip and Plan sections keep their own widths; the fleet strip's tiles wrap and never stretch, and its old full-width top rule is gone.

**The preview has no ship with both stats and a plan.** The item says the sample Krait has both; it doesn't. Recorded stats exist only for ship 1 (Synthetic_Krait, `ships.json` cases) and the recorded plan only for ship 7 (Long Way Round, `engineering.json`). So `krait-stats-*` shows the hero row (ship 1), `krait-module-*` and `krait-plan-*` show the panel and Plan (ship 7, hero row "–" with the preview's stats message beside it), and `anaconda-*` is the ship with no stats. The hero row has its own `targets: []` stats request, so a plan the stats can't work out never blanks it.

**Calls I made that the item didn't settle** (I couldn't reach the starting thread, so these are for the reviewer):
- Military slots (`Military01`…) go under Optional, as the game lists them. The item's slot table has no line for them, but they hold engineerable modules and today's page showed them; the table test covers `Military01`.
- ADD TO PLAN when the slot already has a target raises that target one grade; at the blueprint's top grade it is disabled with a Tip saying so. In the preview the FSD (ship 7) and the power distributor (ship 1) are at top grade, so their captures show it disabled; the wide shot selects the grade-3 burst laser, where it is enabled. The test covers both the enabled path and the not-engineered path (focus moves to the blueprint dropdown; `showPicker()` opens it where the engine allows).
- A module not engineered but planned shows hexagons (none lit, the planned ones Orange Mid). The item says a module not engineered shows none; I read that as "nothing to show", and a planned grade is something to show.
- The panel's need column is Orange (a plan, per DESIGN.md's list) and the planned grade is the panel's one Orange Lit value. Making every need Orange Lit would give the panel a dozen main values.
- Rolls per grade moved from the old default side panel into the Plan list as its own section; the explanations that panel printed are now Tips on the section heading and on the Trade column.
- Selecting a module persists across a Loadout and Plan switch; clicking a selected row or material again closes the panel.

**Brightness.** In every capture outside the sidebar the only Orange Lit text is the chosen view and the main value: the jump range with nothing selected (`krait-stats`), the planned "G5" with a module selected (`krait-module`), and the material's need in the Plan panel. A "–" jump range is not lit. Orange is kept to the links, the dropdowns, ADD TO PLAN, ENGINEERS, the need column and short "!" marks, and hexagons per the Engineer rank and blueprint grade entry (reached grades Orange, planned Orange Mid). The audit's `bright` list ranks by luminance, so White and Options Grey names lead it; read the orange entries for the judgement.

**Audit.** `node scripts/browser/audit.mjs --base <dev server> --plan docs/design/ship-loadout/plan.json`, 19 captures at commit 651761d4, lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast`. Before captures are `before-*.png` with `before.json` and `before-report.json`.

**What else changed.**
- `lib/engineering.ts`: `MODULE_KINDS` and `groupFitted` are replaced by `slotGroup` and `loadoutGroups`, with a table test over every slot in both preview fleets plus `Military01`, `ShipCockpit`, `CargoHatch`, `PaintJob` and `Decal1`.
- The plan state moved out of `Build.svelte` into `plan.svelte.ts` (`ShipPlan`), owned by the page, so both views share one plan and each edit is evaluated once. A side effect: the shopping list now reaches the core's arrival briefing whenever the Ships page is open, not only on the Plan view.
- `Specs.svelte` is deleted. Its figures not in the hero row joined the Plan's stats table: power retracted, laden and hull mass, cargo and fuel, and a Handling group (pitch, roll, yaw as the hull stand-in, with its Tip). Value and rebuy are in the head's Label line.
- Find or plot's blueprint and experimental entries now open the ship's Loadout, where the dropdowns are, instead of the Plan.
- `PAGE_SIZING` checks both parts on `engineering/{View,Build,Stats,Loadout,ModulePanel}.svelte`, and the `engineering` entry is off `TOKENS_EXEMPT`.

**Not verified in the preview.** ENGINEERS opens `#/unlocks?q=<group name>` (for example `Frame%20Shift%20Drive`). The preview only answers its recorded searches, so there it says no match. The core matches a full catalog group name (`crates/astromech-engineering/src/search.rs`, `ship_match`), so it should work in the app; I checked that by reading the code, not by running it.

**Checks.** `pnpm check`, `pnpm lint` and `pnpm test` (90 files, 780 tests) pass, and so does `scripts/check-guide.py`. The journeys pass 7 of 7. This worktree had no core build, and building one here would hold the machine, so the journeys ran with `ASTROMECH_CORE` set to a debug core copied from another worktree whose `crates/` matches main exactly. This item changes no core code.

### Responses

1-mvep.1: fixed in 85676016: the row's hexagons stay the fitted blueprint's and a plan for another blueprint is named after it ("→ Charge enhanced G5", Orange Mid); a test covers it.
1-mvep.2: fixed in 85676016: the Stale line shows after an empty fleet answer, with a test; the no-fleet audit shot now skips setup and captures the Ships empty state (f6c0784b).
2-iq82.3: fixed in ff87a726: the test installs fake setInterval and advances the poll 1000 ms, as the Settings status-poll test does.
4-t6g0.1: fixed in 50a4c238: the test installs fake setInterval and advances the engineer poll 1000 ms for the failure and again for the recovery; no engineering test waits on a real timer now.
