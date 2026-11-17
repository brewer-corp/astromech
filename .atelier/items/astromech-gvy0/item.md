---
schema: atelier.item
schema_version: 3
id: astromech-gvy0
title: The remaining pages move onto the layouts
status: draft
mission: astromech-xajc
labels:
- size-l
- feature
priority: medium
blocked_by:
- astromech-8e22
- astromech-coq6
- astromech-7vxi
- astromech-8wr4
- astromech-nnn0
- astromech-m6gw
- astromech-dpaz
- astromech-rj6i
- astromech-evj3
- astromech-el3i
- astromech-m07p
- astromech-fzhp
- astromech-zaqh
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T06:04:11.267990188Z
---

## Goal

Every page outside Controls that gets no screen of its own still stops sizing itself, and the mission's fills-the-window criterion gets its evidence. Today Stored modules spreads three columns over 1,800 px, and Places home, Mining, Data sharing, Diagnostics and Voice tester set their own widths and sizes. This item runs last, after every other screen item, so its captures show every page as finally built.

## Done when

- [ ] With this merged, `TOKENS_EXEMPT` in `app/eslint.config.js` (astromech-4wi1) holds only the Controls modules and what only Controls pages use (`About.svelte`, `PresetPicker.svelte`, and `Sheet.svelte`, whose labels follow the controller image's scale); the mockups stay in `TOKENS_IGNORED`. That includes `Button.svelte`, exempt today because DESIGN.md gives the primary button 20 px padding, which is not a step on its spacing scale: the button moves to a step (16 or 24 px) or DESIGN.md adds a 20 px step to the scale and its Token map, and the item says which and why.
- [ ] Stored modules (`app/src/modules/stored/View.svelte`) is a List with detail page on the type roles: its rows grouped by where each module is (`Stored.system`, "In transit" for none), each group a Section heading with its count; each module one two-line row: its name in Name, the blueprint and grade as engineer hexagons (`Hexes.svelte`) where engineered, the value as the ranked value in Value L Orange. Selecting one shows its facts in the panel. Empty storage keeps its line.
- [ ] Places home, Mining, Data sharing (`View.svelte`, `Edsm.svelte`), Diagnostics and Voice tester are each on the layout that fits their job (Board for Places home and Mining, Form for Data sharing and Diagnostics, Board for Voice tester unless it fits Form) and the type roles, with no width or font size of their own and no content added. Places home's finder tiles keep DESIGN.md's Places home spec.
- [ ] Both parts of the page-sizing check are on for every file under `app/src/modules/` except the Controls modules and the frame's parts in the route module listed in `PAGE_SIZING_EXEMPT` (the files astromech-uc7i listed). This item enrols whatever is still off: by the plan, `stored/View`, `mining/View` (width), `datasharing/{View,Edsm}`, `diagnostics/View`, `voicetester/View`, `places/Home` (width), `places/{View,MainInput}`, `places/FinderPage` and the nine finder files (font size, after astromech-zaqh's test fix), `settings/{Callouts,ProspectorThresholds}` (width), and `route/Plot.svelte`, which is drawn only inside the frame's `PlotBar.svelte` and so is listed exempt with that reason, or enrolled if it is a page part. A file left over from a screen item is enrolled here too.
- [ ] The brightest things on each converted page are its main value, what can be changed and what needs the commander; no colour outside DESIGN.md's list. No printed explanations: any left move into Tips.
- [ ] The converted pages are checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the route sidebar docked where it docks and as the strip where it does not, and get before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/remaining/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small` or `far`, or the Report says why a listing is not a fault.
- [ ] As the last screen item, it owns the fills-the-window evidence: every page outside Controls captured as finally built at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160, at the Text size the mission's criterion gives each (100, 100, 145 and 180 percent), into `docs/design/final/`, measured with `node scripts/browser/audit.mjs --plan docs/design/final/plan.json`, a plan of every page's states at the four sizes in the form of `docs/design/increment2/plan.json`, as astromech-irtr's and astromech-8e22's were (no sideways scroll of the document or the page column, no text cut by an ancestor, no overlapping text or controls, no text under 12 px before Text size, no action more than one column from what it acts on), the figures in the `docs/design/final/report.json` it writes, and recorded as a dated `[evidence]` line for fills-the-window in the mission's `notes.md`, as astromech-irtr's was on 12 Nov 2026, with the lint lists' state at that commit. A page that fails is fixed in this item or filed as a bug in the mission that blocks it.
- [ ] The existing tests of the converted pages (`stored`, `mining`, `datasharing`, `voicetester` and the Places home) keep their behaviour on the new markup; none is added for layout. The guide pages of the converted pages describe them as they are.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

The pages no other item rebuilds, the last lint lists, the button's padding and the mission's fills-the-window evidence. It is blocked on every other screen item and on astromech-zaqh; astromech-crxa follows it.

## Context

The page-sizing check and its lists are `PAGE_SIZING`, `PAGE_SIZING_EXEMPT`, `TOKENS_EXEMPT` and `TOKENS_IGNORED` in `app/eslint.config.js`; the plan as of 17 Nov 2026 is in the mission's design.md, "Which item converts which page". astromech-irtr's measurements are described in the mission notes (12 Nov 2026) and its report is `docs/design/increment1/report.json`. Replies: `Fleet.stored_modules` (`Stored.ts`: `name`, `system`, `value`, `blueprint_name`, `grade`).
