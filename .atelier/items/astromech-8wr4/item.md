---
schema: atelier.item
schema_version: 3
id: astromech-8wr4
title: Ships > Ships as a loadout
status: draft
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-8e22
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:41.140290178Z
---

## Goal

A ship reads as its loadout, the way the game's outfitting screen lists it: what is fitted in each slot, how far each module is engineered, and what its next grade needs. Today the Info view is a ship cartoon and stat boxes that are mostly dashes (`docs/design/cells-ships-after.png`), and the modules are seen only inside the build planner's form (`app/src/modules/engineering/Build.svelte`). The plan stays: the loadout is where it is made, and the plan's totals keep a view of their own.

## Done when

- [ ] astromech-5g9w is fixed (taken over from the checkpoint): Open in Coriolis and EDSY sit within one column of the ship's name, and no band stretches across the page column. The checkpoint's tool (`node scripts/browser/audit.mjs --base <dev server> --plan docs/design/increment2/plan.json --only ships` (the checkpoint's tool, from astromech-8e22)) does not flag this case under `far`, which measures only an action at a row's end, so the proof is the captures and a measurement: at 1920 x 1080 (100 percent), 2560 x 1440 (145 percent) and 3840 x 2160 (180 percent) the captures show the links beside the head and the hero values, loadout groups and fleet strip at their own widths with the spare width empty, and the Report gives the gap from Open in to the ship's name, in layout px, no more than 360 (the tool's `far` limit).
- [ ] Ships > Ships, for the selected ship (`engineering_fleet`, `#/engineering/<id>` as today), is a List with detail page on the type roles. Its head: the ship's name as the Title, White; type, ident and "N of M modules engineered" in Label; and a row of up to four hero values in Value XL with units in Label, the jump range in Orange Lit as the page's main value while no module is selected and the others Options Grey: jump range, shield strength, top speed and power used of capacity, from `engineering_stats`'s current stats (`Compared.current`, as `Specs.svelte` reads them). A figure the calculations leave out shows "–" with its Tip saying why, as Info does.
- [ ] The loadout lists every fitted module (`Ship.modules`) in four groups headed as the game's outfitting is: Hardpoints, Utility, Core, Optional. The group comes from the slot name (`Fitted.slot`): `*Hardpoint*` except `TinyHardpoint*` is Hardpoints, `TinyHardpoint*` Utility, `Armour`, `PowerPlant`, `MainEngines`, `FrameShiftDrive`, `LifeSupport`, `PowerDistributor`, `Radar` and `FuelTank` Core, `Slot*` Optional; cosmetic and fixed internals (`PlanetaryApproachSuite`, `ShipCockpit`, `CargoHatch`, paint, decals and the rest) are not listed. The mapping is one function with a table test over the slot names in `app/src/preview/fixtures/ships.json`.
- [ ] Each module is one row: class and rating as the module name gives them ("7A") in Figures, the module name in Name Options Grey, its grade as engineer hexagons (`Hexes.svelte`, `tone="grade"`, the planned grade lit to it in Orange Mid through `to`), then the blueprint name and the experimental effect in Prose Grey. A module not engineered shows no hexagons; one that cannot be engineered (`group: null`) says nothing more.
- [ ] Selecting a module raises its row (Hull Raised, lit text) and fills the side panel: the module name as its title, White; its blueprint and grade now (`Fitted.engineering`); the plan's target for it if the ship's plan has one, with that target's materials as need against have and the short ones marked with an orange "!" (`engineering_plan`'s `TargetCost.materials` and `PlanResult.materials`), and the engineers who apply its top grade (`TargetCost.engineers`). The plan's blueprint, grade and experimental dropdowns for that module move from `Build.svelte`'s rows into the panel; ADD TO PLAN, the primary button, adds a target one grade up on the module's current blueprint, or opens the blueprint dropdown for a module not yet engineered. ENGINEERS, a text action, opens Engineering > Engineers searched for the module's group (`#/unlocks?q=<group name>`; a full catalog group name matches, astromech-7did). With nothing selected the panel shows nothing.
- [ ] The rest of the fleet is a strip under the loadout, as today's rail: each ship's cartoon (`ShipIcon`), name in Name and type and where it is in Label; the current ship first, marked "Here" in Label, raised when selected. Selecting one changes `#/engineering/<id>`.
- [ ] The Info and Engineering choice becomes Loadout and Plan (`#/engineering/<id>` and `#/engineering/<id>/plan`, so old links keep opening the same views). Plan keeps what `Build.svelte` shows below its module rows today: the stats now against the plan done, the shopping list, the trades, what is still to collect and the engineer trip with Follow, on the List with detail layout with the material panel it has today. Nothing it does today is dropped.
- [ ] The Coriolis and EDSY links stay as text actions in the head, and the stale-loadout note stays as one Label line with its Tip. A stored ship's system and station links stay in the head's Label line.
- [ ] States: no fleet yet (`?core=empty`) one prose line; reading the journal one Label Grey line; a failed fleet poll keeps the last answer with the Stale line; the stats failing says so beside the hero row with the headline failure as today; a ship never flown while a journal was kept says its modules are unknown and shows no loadout.
- [ ] The brightest things: the selected module's planned grade and the panel's need (main value; in the preview that is a module of the sample plan), ADD TO PLAN and the dropdowns (what can be changed), and a short material's "!" (what needs the commander). With nothing selected the page's main value is the jump range. No colour outside DESIGN.md's list: hexagons as its Engineer rank and blueprint grade entry.
- [ ] No printed explanations; what a figure counts goes in a Tip. The Specs and Stats boxes are gone; the figures they held that are not in the hero row are kept: speed, boost, DPS and power use are already in the Plan view's stats now against plan (`Stats.svelte`); pitch, roll and yaw (the hull stand-in, with its Tip) and power retracted and deployed join that list; value and rebuy join the head's Label line.
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, on the sample Krait (the only preview ship with recorded stats and a plan) and on the Anaconda with no stats: with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100. At 1100 x 700 the panel goes below the loadout, which keeps its four groups in one column. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/ship-loadout/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/engineering/{View,Build,Specs,Stats}.svelte` (or what replaces them) in `PAGE_SIZING`, and the `engineering` entry leaves `TOKENS_EXEMPT`.
- [ ] Tests: the cases in `engineering/View.browser.test.ts` and `Build.browser.test.ts` keep their behaviour on the new markup (Info's become the loadout's, Engineering's the Plan's); new browser cases cover selecting a module filling the panel from the sample plan's target (ship 7's `FrameShiftDrive`, a target of the recorded plan in `app/src/preview/fixtures/engineering.json`, `plan_input`; the preview answers `engineering_plan` only for that ship, `app/src/preview/shell.ts:435`), and ADD TO PLAN sending `engineering_plan` a plan with the new target (what reached the backend) and the row then showing the planned grade. The slot grouping has its table test. No test of layout or copy.
- [ ] `docs/guide/ships.md` (A ship's page, Build planner, Ship stats) describes the loadout, the panel and the Plan view.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Ships > Ships only. Stored modules move onto a layout in astromech-gvy0; Suits is astromech-nnn0. No core change: the fleet, stats and plan replies give everything above.

## Context

Mockup: `docs/design/glow-up-ship.png`. Its layout stands; under revision 5 its rows are not filled cells (read-only rows on Void, the selected one Hull Raised, not filled orange), its grades are hexagons, not squares, the fleet strip's current ship is raised, not filled, and module names are Options Grey, not orange.

Replies: `Ship.ts` (`modules: Fitted[]`, `links`, `loadout_ts`), `Fitted.ts` (`slot`, `slot_name`, `item`, `name`, `group`, `engineering: Applied`), `Applied.ts` (`blueprint_name`, `grade`, `experimental_name`), `PlanResult.ts` and `TargetCost.ts` (per target: `materials`, `engineers`, `steps`). The plan is kept per ship in localStorage (`app/src/lib/engineering.ts:137-155`). The browser preview has no planner (`engineering.ts:79`): only the sample ship's sample plan evaluates, so the panel's needs show there for the sample plan's modules; in the app any plan evaluates. The ship stats are worked out by the core; in the preview only the sample Krait has them (`app/src/preview/fixtures/ships.json`).
