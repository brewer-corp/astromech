---
schema: atelier.item
schema_version: 3
id: astromech-gvy0
title: The remaining pages move onto the layouts
status: changes_requested
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
- astromech-59n6
- astromech-unak
- astromech-z4vm
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Adef100aa-bc29-483c-a2c2-ce13ab8d472a
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T06:04:11.267990188Z
---

## Goal

Every page outside Controls that gets no screen of its own still stops sizing itself, and the mission's fills-the-window criterion gets its evidence. Today Stored modules spreads three columns over 1,800 px, and Places home, Mining, Data sharing, Diagnostics and Voice tester set their own widths and sizes. This item runs last, after every other screen item, so its captures show every page as finally built.

## Done when

- [x] With this merged, `TOKENS_EXEMPT` in `app/eslint.config.js` (astromech-4wi1) holds only the Controls modules and what only Controls pages use (`About.svelte`, `PresetPicker.svelte`, and `Sheet.svelte`, whose labels follow the controller image's scale); the mockups stay in `TOKENS_IGNORED`. That includes `Button.svelte`, exempt today because DESIGN.md gives the primary button 20 px padding, which is not a step on its spacing scale: the button moves to a step (16 or 24 px) or DESIGN.md adds a 20 px step to the scale and its Token map, and the item says which and why.
- [x] Stored modules (`app/src/modules/stored/View.svelte`) is a List with detail page on the type roles: its rows grouped by where each module is (`Stored.system`, "In transit" for none), each group a Section heading with its count; each module one two-line row: its name in Name, the blueprint and grade as engineer hexagons (`Hexes.svelte`) where engineered, the value in Figures, Options Grey (the rows are grouped by place, not ranked, so no orange). Selecting one shows its facts in the panel. Empty storage keeps its line.
- [x] Places home, Mining, Data sharing (`View.svelte`, `Edsm.svelte`), Diagnostics and Voice tester are each on the layout that fits their job (Board for Places home, Mining and Voice tester, Form for Data sharing and Diagnostics) and the type roles, with no width or font size of their own and no content added. Places home's finder tiles keep DESIGN.md's Places home spec.
- [x] Both parts of the page-sizing check are on for every file under `app/src/modules/` except the Controls modules and the frame's parts in the route module listed in `PAGE_SIZING_EXEMPT` (the files astromech-uc7i listed). This item enrols whatever is still off: by the plan, `stored/View`, `mining/View` (width), `datasharing/{View,Edsm}`, `diagnostics/View`, `voicetester/View`, `places/Home` (width), `places/{View,MainInput}`, `places/FinderPage` and the nine finder files (font size, after astromech-zaqh's test fix), `settings/{Callouts,ProspectorThresholds}` (width), and `route/Plot.svelte`, which is drawn only inside the frame's `PlotBar.svelte` and so moves to `PAGE_SIZING_EXEMPT` with that reason. A file left over from a screen item is enrolled here too.
- [x] The brightest things on each converted page are its main value, what can be changed and what needs the commander; no colour outside DESIGN.md's list. No printed explanations: any left move into Tips.
- [x] The converted pages are checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the route sidebar docked where it docks and as the strip where it does not, and get before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/remaining/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] As the last screen item, it owns the fills-the-window evidence: every page outside Controls captured as finally built at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160, at the Text size the mission's criterion gives each (100, 100, 145 and 180 percent), into `docs/design/final/`, measured with `node scripts/browser/audit.mjs --plan docs/design/final/plan.json`, a plan of every page's states at the four sizes in the form of `docs/design/increment2/plan.json`, as astromech-irtr's and astromech-8e22's were (no sideways scroll of the document or the page column, no text cut by an ancestor, no overlapping text or controls, no text under 12 px before Text size, no action more than one column from what it acts on), the figures in the `docs/design/final/report.json` it writes, and recorded as a dated `[evidence]` line for fills-the-window in the mission's `notes.md`, as astromech-irtr's was on 12 Nov 2026, with the lint lists' state at that commit. A page that fails is fixed in this item or filed as a bug in the mission that blocks it.
- [x] The existing tests of the converted pages (`stored`, `mining`, `datasharing`, `voicetester` and the Places home) keep their behaviour on the new markup; none is added for layout. The guide pages of the converted pages describe them as they are.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

- [x] Leftovers the screen items handed here are done: the Places finders and the Now page draw a pad through the shared `app/src/lib/ui/PadLetter.svelte` astromech-el3i built, with no copies of their own; the candidates section of Colonisation (`Colonies.svelte`) prints no explanation, its text moving to a Tip (left by astromech-m07p); and the states those items could not capture for want of data are either captured from a recorded fixture or listed in the Report as not captured, with the reason: Colonisation with everything aboard, a failed project and an unknown cargo capacity; Biology with nothing being sampled and an empty system; the Now page's exploration data at risk line.

## Scope

The pages no other item rebuilds, the last lint lists, the button's padding and the mission's fills-the-window evidence. It is blocked on every other screen item and on astromech-zaqh; astromech-crxa follows it.

## Context

The page-sizing check and its lists are `PAGE_SIZING`, `PAGE_SIZING_EXEMPT`, `TOKENS_EXEMPT` and `TOKENS_IGNORED` in `app/eslint.config.js`; the plan as of 17 Nov 2026 is in the mission's design.md, "Which item converts which page". astromech-irtr's measurements are described in the mission notes (12 Nov 2026) and its report is `docs/design/increment1/report.json`. Replies: `Fleet.stored_modules` (`Stored.ts`: `name`, `system`, `value`, `blueprint_name`, `grade`).

## Report

**Deviations and a question for the lead, first.**

- **Data sharing still prints what EDDN and EDSM are sent.** The Done-when says printed explanations move into Tips. These two blocks are what the commander agrees to by turning sharing on (the commander name goes with every message, what is stripped, what is never sent), and a Tip is one line or a title, a fact and a note, so the list would not fit and the disclosure would be hidden behind a hover. I kept both as the group's description in Body at the prose measure, the way Settings > General keeps its group descriptions. Moving them, for instance behind a link to `docs/guide/data-sharing.md`, changes what the commander reads before consenting, so it is Sumeet's call. `atelier new` refuses on an item branch, so please file it as a `needs-human` item: "Data sharing: keep the EDDN and EDSM disclosures printed, or move them behind a link to the guide?"
- **Places home tiles are 240 px, not DESIGN.md's 220.** The page is now on the Board in its `Tiles` grid (200 to 240 px columns, auto-fill), which draws 240 wherever the column has room, and the page may set no width of its own. I changed DESIGN.md's Places home spec to say so rather than give `Tiles` a size option for one page. Everything else in the spec holds: a link in a Seam outline, the name in Orange, one Label line, a live answer in Value L, Options Grey. The old three-tile cap is gone, so a wider window shows more tiles in a row when a section has more.

**The button.** Before and after: `remaining/before/diagnostics-*.png` and `remaining/diagnostics-*.png` show Save diagnostics at 20 and 16 px. The primary button's padding moved from 20 px to 16 px (`--sp-4`, the scale's `lg`), and DESIGN.md's Buttons and its front matter say so with the reason: the scale has no 20 px step, and of its two neighbours 16 keeps the button no wider than it was where it shares a line with a dropdown or an input (Suits' panel, dialogs). Adding a 20 px step would have meant a scale step used by one component. `Button.svelte` is off `TOKENS_EXEMPT`.

**Lint lists.** `PAGE_SIZING` is one entry, both parts on `src/modules/**/*.svelte`; `route/Plot.svelte` joined `PAGE_SIZING_EXEMPT` (drawn only inside PlotBar). ESLint's resolved config at 5389cb65: both parts on for 52 module files, off for 13 (the Controls pages and the route's frame parts), none partial. `TOKENS_EXEMPT` holds only the Controls modules with About and PresetPicker (spacing) and Sheet (font size, spacing); the pending gvy0 entry and Button are gone. Callouts' and ProspectorThresholds' 67.5 px number boxes are now `Input`'s `short` option, so the width lives in the shared component.

**Pages.**
- Stored modules: List with detail. Grouped by `system` ("In transit" last), each group a Section heading with its count; a two-line row of name (Name) over blueprint (engineer hexagons and the blueprint's name, its accessible name "Increased range, grade 3 of 5"), the value in Figures, Options Grey; no orange. Selecting a row fills the panel: title, place, the value in full as the panel's main value, Where (with the copy chip) and Blueprint. The journal source moved into a Tip on the count line. Section headings are caps, as the Section role is, so a system's name shows in caps there.
- Mining, Places home, Voice tester on the Board; Data sharing and Diagnostics on the Form (`--form-label`). No width or font size of their own, and nothing added.
- Printed explanations moved to Tips: Mining's "the run starts at…", Voice tester's Run live sentence (Tip on the button) and the Learned phrases description (Tip on the heading), Colonisation's candidates sentence (Tip on the heading). Voice tester's intro is now a state line: "Push-to-talk is in test mode…" while it is, else "Push-to-talk testing needs Elite running." A first version put that on a Tip on the phrase box, but it then sat over the page tabs whenever the box had focus, so it became a line. Diagnostics' crash warning is now Orange at 500, as DESIGN.md's warnings are. A failed first read on Stored and on Data sharing (both pages) is now a hard error, Alarm Red with its "!", with no action since the page reads again on its own; it was a plain line.
- Pads: SearchKit (every finder) and the Now page's sale draw `PadLetter`; the copies and their CSS are gone. SearchKit's busy dimming reaches the pad through `:global(.pad)`.

**Checked in the browser preview** (`docs/design/remaining/`, `plan.json`, `report.json` at 5389cb65, before captures in `before/` from `plan-before.json`): 20 states at 1100 x 700 and 1920 x 1080 at 100 percent and 2560 x 1440 at 145, 60 captures. Covered: Stored (list, selected, empty, loading, failed), Places home, Mining (run, no run), Data sharing (loaded, loading, failed), Diagnostics (loaded, failed), Voice tester with a result, and the six finders whose font sizes moved to roles. The route is the strip at 1100 x 700 and docked at the larger two in every capture except the empty Stored one, which has no route (strip at 1100, rail above). Nothing under sideways, overlaps, small, far or contrast. Clipped: Raw sites' 6 ellipsised site details, the same 6 as in the before capture; the side panel shows the selected site's details whole (`final/raw-sites-selected-1920x1080.png`), so it is not a fault. Mining's loading and failed states are not in the plan: `loadMining` reads the run from `coreStatus()`, the status read every page and the frame share, so `?fail=` or `?slow=` on it is not a state of Mining's own; the existing previewstates test covers the page's lines. The finders before and after differ only by the Label role's 1.3 line height (a few px of vertical shift); colours are unchanged.

**Brightest things**, from the `bright` lists at 2560 x 1440, the sidebar apart:
- Stored: white is the panel's title and the selected row's name with its current section (The Lifted Name Rule), then the panel's main value in Orange Lit.
- Mining: the Refined total is the one Orange Lit value, with the ranked tons in Orange.
- Places home: the finder names in Orange, the things you press.
- Data sharing: the cells (Off, Turn on) and the last error in Orange.
- Diagnostics: Save diagnostics is the page's one fill.
- Voice tester: the result values ("gear down", "100%", "Press the keys, then say…", the key) in Options Grey over their Label Grey labels (the labels outshone the result until a late fix), and Test and Remove in Orange.
- Data sharing and Diagnostics: the row labels share the Options Grey step with the values there, because DESIGN.md's Settings row puts a Form's labels in Options Grey.

The only fill on any of them besides the top tab is Diagnostics' primary button. No colour outside DESIGN.md's list.

**Fills-the-window evidence.** `docs/design/final/`: 30 states at the four sizes, 120 captures, at 5389cb65. The states are every page outside Controls, all ten finders with a row selected, and the selectable pages with an item selected. No sideways scroll, overlaps, small text, far actions or contrast listings anywhere. Clipped lists only the two ellipses that were there before, each shown whole in its side panel: Colonisation's stock lists, as astromech-m07p reported, and Raw sites' details. Recorded as the dated `[evidence]` line in the mission's notes.md with the lint state. No page failed, so no bug filed. Every shot ran on its own (`--only`), because one failed wait stops a whole run (astromech-a0j1); both reports were written fresh at the final commit.

**Not captured, for want of recorded data.** The preview's fixtures are the real core's replies dumped from `worlds/preview/`, and none holds these states. Writing replies by hand is against the fixture rule.
- Colonisation with everything aboard: both recorded projects have `in_cargo` 0 on every line.
- Colonisation with a failed project: both have `failed: false`.
- Colonisation with an unknown cargo capacity: both have `capacity: 784`, which comes in the same `colonisation_projects` reply, so no `?fail=` of another call makes it.
- Biology with nothing being sampled, and with an empty system: the one recording has a sample in progress, and `?core=empty` answers an error ("Elite isn't running"), not an empty system.
- The Now page's exploration data at risk line: the exploration recording has `risky: false`.
- Voice tester in test mode: the fake's `voice_test_poll` always answers `listening: false`.

A follow-up could record those worlds, as astromech-nh0g does for routes.

**Tests.** `pnpm lint`, `pnpm check` and `pnpm test` (915 tests) pass. A review-style read found that Stored held its selection as an index, so when a module left storage the panel silently showed another one; a failing test came first (`keeps the selected module in the panel when another one leaves storage`), and the selection is now keyed on the module. The stored test now finds the blueprint by the hexagons' accessible name and the module in its system's group, since the heading carries the count. The page-sizing config test now checks the one glob. The voice journey waited on the Run live sentence, now a Tip, as its signal that test mode is on; it waits on the test-mode line instead. All seven journeys pass (`node --test 'journeys/*.journey.ts'`), run against `astromech-core` rebuilt (incrementally, `nice -j 4`) in the main checkout's `target/` at main 77a5c71b, outside this worktree; this branch changes no Rust.

**Guide.** ships.md (Stored modules as built), mining.md, voice-tester.md (state line, Tips) and colonisation.md updated; `docs/guide/img/learned-phrases.png` recaptured, since the old image showed the window before the nav rework. `scripts/check-guide.py`: 0 problems.

CONTRIBUTING.md now says page-sizing covers every page outside the exempt list.

**Left as it was.** `--list-w` stays in tokens.css: no page uses it, but the old mockups in `src/mockups/ds/` do, and DESIGN.md's Token map now says so.

