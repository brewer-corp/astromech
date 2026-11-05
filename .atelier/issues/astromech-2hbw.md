---
created_at: "2026-11-05T06:19:45.194670106+00:00"
id: "astromech-2hbw"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-nxz9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-rz0f"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T15:37:08.591342110+00:00"
status: "done"
title: "Find or plot: one header box for systems, finders and Ctrl+K"
updated_at: "2026-11-05T15:37:08.591342110+00:00"
---

## Goal

Replace the header's systems-only plot box (`modules/route/PlotBar.svelte`) and the separate Ctrl+K palette with DESIGN.md's one Find or plot box: type a system to plot, add a stop or set the destination; type a phrase to open a finder; type a page, material, module or ship as the palette does today (decision astromech-bi94).

## Done when

- The header has one 320 px input, placeholder "Find or plot", "Ctrl+K" at its right end; Ctrl+K focuses it from any page (except while a bind capture has the keyboard), and `lib/ui/Palette.svelte` as a separate overlay is gone.
- Its popover lists, ranked together: systems (Spansh autocomplete) with Plot, Add stop and Set destination; finders, matched by name and by phrases with an input ("interstellar factors", "raw trader", "outfitting fsd", "shipyard python"), opening the finder filled in; the matcher reads the list of finders the Places module declares, so finders added later (astromech-kwoi, astromech-f7i0) are matched without changing this box; and today's palette entries (`lib/palette.ts`).
- Plot keeps today's behaviour (the panel with `Plot.svelte` and the external links).
- Tests: the matcher (`palette.test.ts` or a new `find.test.ts`: a system, a phrase with an input for each of astromech-nxz9's finders: Services, Outfitting, Shipyard, Material traders; a page; a module), keyboard use, and the three system actions calling the core; `pnpm lint && pnpm check && pnpm build && pnpm test` pass.
- `scripts/ui` screenshots of the box at rest, with a system typed, and with a finder phrase typed.
- docs/guide (routes and the index's Ctrl+K text) updated.

## Scope

`App.svelte` header, `modules/route/PlotBar.svelte`, `lib/ui/Palette.svelte`, `lib/palette.ts`, `lib/find.ts`. Voice is out of scope: it already reaches lookups through its own grammar.

## Context

- DESIGN.md: Find or plot, Frame (header 64 px), Input.
- Needs astromech-nxz9's finders and URLs and the Add stop and Set destination actions (astromech-rz0f). Its phrase tests cover only astromech-nxz9's finders so it does not wait for astromech-kwoi; whichever of the two lands second adds phrase tests for the Trade and Materials finders ("sell painite").
- Search box and voice keep separate matchers that target the same finder IDs and URLs (decision astromech-g8d5, item 7).

## Notes

- 2026-11-05 [note] 2026-11-05: decision astromech-ghyo item 9: below the width where the one-row header fits (about 1600 px), the top tabs move to a 44 px second row under the wordmark. 2hbw implements this breakpoint with the one-row header and remeasures the width once the 320 px Find or plot box and its Ctrl+K hint are in, then updates DESIGN.md's Frame table figure.
- 2026-11-05 [note] 2026-11-05: deferred findings from the nxz9 review (reviewer-fable-nxz9), part of this child: astromech-5pjm: give each Finder in app/src/lib/places.ts its phrase/entry list (input vocabulary + a route-with-input builder) read by both the box's matcher and modules/places/palette.ts, so kwoi and f7i0 add finders without editing the box; today palette.ts hand-lists FINDERS+SERVICES+MODULES+SHIPS and leaves TRADERS out ('raw trader' opens Services). astromech-ux31: App.svelte:54 normalises a moved link on first load with a location.hash push; use history.replaceState like the in-window path, so Back from a bookmarked #/services doesn't land on the old hash.
- 2026-11-05 [note] 2026-11-05: deferred findings from the kwoi review (reviewer-fable-kwoi), part of this child: astromech-7r5k: docs/V1-DESIGN.md's Trade row (~194) and Galaxy rows (~196-198) still describe Trade's Sell/Buy and the old Galaxy pages; bring them in line with Places. astromech-6q7m: stale 'opens its nearby search' comments in app/src/lib/palette.ts:5 and lib/ui/Palette.svelte:4, and FINDERS' input 'Hold' (Sell my cargo) has no reader yet: give it one or drop it when the box reads FINDERS.

## Close Reason

Implementer report. Find or plot replaces the header plot box and the Ctrl+K overlay: one 320 px box with a Ctrl+K hint (the clear glyph replaces it once typed), focused by Ctrl+K from any page except during a bind capture; lib/ui/Palette.svelte is gone and the route strip's focusFindOrPlot() still works. lib/find.ts ranks Spansh systems (PLOT, Add stop, Set destination) with pages, finders, materials, modules and ships in one list (whole label, start, word start/initials, anywhere, all words; a typed exact system 0, else after word-start matches; 10 rows). Each Finder declares inputs() (label, route, keywords), read by both the Places palette entries and the phrase matcher ("raw trader" → Material traders, "outfitting fsd", "shipyard python", "sell painite", "painite hotspots"), so later finders need no box change (5pjm). Plot keeps its panel; Add stop / Set destination show the FollowNote. Folded in: header breakpoint 64 or 64 + 44 px, one row from about 1580 px with the usual status (measured; up to about 1900 px with a long status, estimated), written into DESIGN.md Frame (ghyo 9); history.replaceState for moved links on first load (ux31); V1-DESIGN Trade and Galaxy rows (7r5k); stale comments, Sell my cargo's Hold input now names its hold list (6q7m). DESIGN.md gained one sentence on popover rows; PRODUCT.md's Ctrl+K line describes the box (reviewer: factual update). Proof: app lint, svelte-check 0/0, build, 464 tests; find.test.ts phrase tests fail with phrase matching off; App history test fails with the old redirect; check-guide 0 problems; screenshots docs/design/find-or-plot-*.png; the reviewer exercised phrases, Ctrl+K and the strip shortcut in headless Chrome. Review: reviewer-sonnet-2hbw; fix_now gasg (Enter before Spansh answered opened a loose entry instead of plotting) fixed by the orchestrator in d16846ab (matches carry sure; Enter plots typed text unless sure or the highlight moved), approved astromech-kzye; n1xc and hdj3 → astromech-4odm, vm1x → astromech-uk11; zp4e records decisions (Tab-only row actions, accept buttons in options for now, keep commodity values for Sell and Buy, live header switch, accept the DESIGN sentence, keep PRODUCT.md), mbng no action.
