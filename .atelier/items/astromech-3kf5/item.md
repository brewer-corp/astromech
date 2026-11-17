---
schema: atelier.item
schema_version: 3
id: astromech-3kf5
title: 'Calmer pages: data in white and grey, orange for the ranked value and what you can act on'
status: approved
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-b2cz
implementers:
- thread:delegated-task:command%3Amcp%3Af257f609-54d7-4e2e-bfce-45bbbe671617%3Adelegate-task%3A2e53a792-ff91-43d6-9a1d-43aa21aef462
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.238277178Z
---

## Goal

The eye has somewhere to go. Today almost every piece of text is saturated orange at weight 500; Sumeet, 16 Nov 2026: "if everything draws the eye, then nothing can". Data becomes white and grey, and orange is spent on the value a list is ranked by, the main value, and what the commander can act on.

## Done when

- [x] `tokens.css` has DESIGN.md revision 5's greys and duller red; the type role classes and the shared components in `app/src/lib/ui/` (rows, tiles, Stat, Badge, Button, Choice, tabs, StatusBar, the layouts' side panel) take their colours from the roles, so a page gets the new palette without naming a colour.
- [x] Selection everywhere is the raised row with lit text; no row, tile or option is a solid orange fill. The filled orange button is unchanged.
- [x] Tiles and page blocks are off Hull (outline or Hull Raised); Hull is the frame's.
- [x] Every page built so far is restyled and checked in the preview at 1100 x 700 and 2560 x 1440: Route, Materials, the Places finders, Trade > Commodities, Settings, Powerplay, Exploration, Biology, Session, Mining, Station, On foot, Ships, Stored modules, Engineers and the Controls pages. On each, the main value, what can be changed and what needs the commander are the brightest things.
- [x] Before-and-after captures of Materials, the Sell finder, Powerplay and My controller at 2560 x 1440 in `docs/design/`; the design-system screenshots re-rendered.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass; no test asserts a colour.

## Scope

Colour, weight and selection. The two-line grouped result row is astromech-3ca6; the sidebar is astromech-uc7i. Controls pages change colour only.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). `tone5-sell-2560.png` and `tone5-engineers-laser-2560.png` are the reference; `app/src/mockups/nav/tone.css` on the spike holds the mockup's values as a starting point, written fresh here against the real tokens.

## Report

Pages now read the way revision 5 asks: data is white and grey, and orange is left for the ranked column, the page's main value and what the commander can act on. Before and after at 2560 x 1440 are in `docs/design/calmer-{materials,sell,powerplay,controller}-{before,after}.png`; the `design-system-*.png` mockups are re-rendered.

What changed:

- `tokens.css` has `--label-grey`, `--grey-dim`, `--mark-off` and `--red-low` (`#ca6657`, DESIGN.md's value, not the spike's). The body default is Prose Grey at 400, so anything a page doesn't colour is grey data. The type roles take the Hierarchy table's colours and weights, and `.type-cell` is new. `.main` on Value XL or Value L marks the one main value in Orange Lit; Stat takes a `main` prop for it (Session's Earned, Exploration's Value now, Mining's Refined).
- Selection is raised, never filled: result rows, material tiles, the ship rail, the selected preset and keybind, the next route stop, the controller sheet's lit actions (Orange Lit on Hull Raised). Highlighted popover rows (Find or plot results, Picker, Menu, Dropdown) are ember cells, and their current option is Orange Lit. A chosen option (Choice, onboarding's mic) is Orange Lit at 600 with a 2 px orange underline. The only orange fills left are the primary button, the active top tab (and onboarding's SETUP tab), and the listening cells (bind dialog, key tester), which DESIGN.md allows.
- Hull is only the frame's now (route strip, status bar). Tiles, spec bands, log rows, engineer cards and summaries are Seam outlines; the next route stop and the calibration prompt are Hull Raised. Side panels have a Seam rule instead of an orange one.
- Route stars carry the list's colours: scoopable Orange Mid, neutron White, others Label Grey, past Mark Off. An inhabited stop's security is Orange Mid when low and Red Low for anarchy; no other item owned this, so it is done here. Engineer cards carry theirs: unlocked names Options Grey with white rank marks, known or invited names Orange Mid, barred Red Low, not introduced Grey Dim for the name and state only, so the step to unlock stays readable (as before, a card you can work on now is not dimmed).
- Orange text at body size is 500. A computed-style sweep over 25 page URLs at 2560 x 1440 (every page the item names, Places home and four finders) found no orange text of 15 px or more below 500, no Hull outside the frame, and no Label Grey or Orange Mid text on an ember cell.
- DESIGN.md's Token map loses its "(new)" marks for these tokens and names `.main`; its header no longer lists colours, selection and tiles as pending. The guide (engineering, materials, places, settings) no longer says "filled orange".

Checked in the preview (`pnpm dev`, `?core=ready&route=timeline`, Text size 145 at 2560 x 1440 and 100 at 1100 x 700). Every page was captured at both sizes, and each was opened at one size or both after its last edit: every page the item names, plus Places home, Buy, Sell my cargo, Material traders, Services, Outfitting, the Ships build plan, On foot's three tabs, Data sharing, Diagnostics, Voice tester, Colonisation, Logbook and onboarding. States: a selected Sell row with its panel, a selected material tile, Find or plot results, an open Picker and Menu, the bind dialog listening, the controller sheet with an activity and a press lit, a Trade route result, and `route=exact` for the star colours. Not seen: an open native dropdown list (the headless browser doesn't draw `select::picker`), so its Orange Lit current option and ember-cell highlight are checked in code only; and a page in a failed state, since the preview with no core showed no failure line on the pages I tried.

Decisions and notes for review:

- I added and then removed `.ranked`, `.lifted`, `.dim` and `.raised` modifiers: nothing used them, because the result row, tiles and the timeline keep their own selection rules. Pages still name grey tokens for their own parts (`var(--label-grey)` and so on) rather than only role classes; astromech-4wi1 is the lint for raw colours and can tighten this.
- Powerplay marks no single main value: it is a board of four panels (standing, this system, merits, where to work) and none outranks the rest. Its ranked column (jumps in Where to work) is orange.
- On Sell my cargo and the Trade route, the ranked figure per row is orange and the total profit is the route page's main value in Orange Lit. On Session, the credits column is the ranked one.
- The Ships stats table shows a plan's new value in Options Grey at 500 and its change in Orange (a plan is something to act on).
- The Market's "+3,921/t" gain and Colonisation's still-to-deliver figures stay orange: both say where to act.
- The Engineers page's legend changed from "orange marks are your rank" to "white marks are your rank".
- Orange Lit stays on a few things that are the one answer their page is showing: the control just pressed (All keybinds), the newest press (Key tester), My controller's "press it again" prompt, and the current calibration step.
- Ran the CONTRIBUTING checks that apply to an app-only change: `pnpm build`, `scripts/check-dist.py`, `scripts/check-guide.py` and `tsc -p journeys`. No Rust changed, so the cargo checks and the journeys themselves were not run.

Left for other items: demand and pad colours (astromech-3ca6, with the two-line rows), the hexagon component (astromech-7did), the sidebar (astromech-uc7i) and the raw-colour lint (astromech-4wi1). The installed-app colour check on Windows is astromech-3340.

`pnpm check`, `pnpm lint` and `pnpm test` (640 tests) pass; no test asserts a colour.

### Responses

- 1-k9n1.1: fixed in f5a5f6e1
- 1-k9n1.2: fixed in f5a5f6e1
