---
created_at: "2026-10-24T23:27:52.556049922+00:00"
id: "astromech-yxtz"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-yxtz
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Orange palette and caps per DESIGN.md: move pages from grey to the game's orange ramp"
updated_at: "2026-11-05T16:30:36.286387039+00:00"
---

## Goal

Move the pages onto DESIGN.md's palette: the game's orange as the base, rank and state by its brightness (lit, base, mid, dim) and a solid fill for selection, white only for titles, the current section and a key value, and caps only for the wordmark, top tabs, section headings and action words (DESIGN.md: The Intensity Rule, The One Fill Per Group Rule, The Narrow Red Rule, The Caps Are Wayfinding Rule).

## Done when

- Text on station-style pages uses `--orange-lit` / `--accent` / `--orange-mid` / `--orange-dim`; grey remains only for Controls and Settings row labels (`--options-grey`), prose (`--prose`) and the window.
- Active top tab is a solid orange box; active page tab is Orange Lit with a 2 px underline; inactive tabs are Orange Mid. Section headings are orange caps with a `--line` rule; the current section is white.
- Uppercase + `--caps` appear only in the four allowed roles.
- Warnings are an orange "!" plus words; hard errors a red "!" plus words and a recovery action; a stale row keeps its name Orange, steps its value and facts to Orange Mid and shows its age in Orange (no red). No filled blocks.
- The old grey tokens are removed from `tokens.css` once unused.
- Screenshots of Engineers, Trade, Keybinds, Settings and the route error before and after.

## Context

Pages use the first grey-based tokens (`--text`, `--text-lo`, `--cell-opt`); about 140 `text-transform: uppercase` rules; the active top tab is white while the deepest tabs are orange; "Could not read the route" is a solid orange block. DESIGN.md (revised 25 Oct) keeps grey only for the window background, Options-style row labels and prose, and adds `--alarm` red for hard errors only (decision astromech-5p9c); stale rows use Orange Mid facts with the age in Orange.

## Notes

- 2026-11-04 [note] Deferred from decision astromech-5p9c: (1) cell-hover contrast (Fable R9, Opus R8): Orange on Ember Cell Hover is 4.4:1 and the placeholder 3.1:1, below Orange on Ember Cell (5.1:1); pick a hover that keeps cell text at 4.5:1 or record why not. (2) placeholder and scrollbar greys in tokens.css (Opus R12): the global input::placeholder is still --text-lo grey and the scrollbar thumb #3a3c3e / #55585b; move them to --placeholder and the ramp when the pages move. (3) a definition of 'current section' (Opus R7): DESIGN.md makes its heading white but never says which section is current (the one holding the selection or focus? the one scrolled to?); define it before pages use it. Also: this issue's Done when line on stale rows was changed to DESIGN.md revision 3 (no red for stale, per 5p9c item 11).
- 2026-11-04 [note] When the Controls pages move: 'Unbound' is Orange Dim on Ember Empty (3.0:1), but DESIGN.md says Dim never carries text needed to act. Either state that 'Unbound' is state only, or use Orange Mid (review astromech-qy3c).
- 2026-11-05 [note] 2026-11-05 implementer (reversible choices, open to review): (1) current section = the section holding the item a side panel describes; none without such a selection; focus and scroll never make one current (DESIGN.md White and States). Applied on My controller, Presets and the Ships build's Shopping list. (2) --cell-hover #4d2b12 -> #482810: Orange on hover 4.6:1, placeholder on hover 3.3:1 (hint backed by the input's label; recorded in DESIGN.md Surfaces). (3) Empty binding slot's word ('Set' in the app; DESIGN said 'Unbound') is Orange Mid on Ember Empty, 4.7:1, since it is what the commander clicks; DESIGN.md binding-cell-empty and Deviations updated. (4) Scrollbar thumb: Seam at rest, Orange Dim on hover, Orange Mid while dragged. (5) Pages that re-read every few seconds show a red '!' and words without an action (the next read recovers); DESIGN.md States says so. (6) Stale's 'Could not refresh' line stays an orange '!' warning in Orange Mid, not red: the data is kept and re-read automatically. (7) --cell-opt kept for astromech-dfvp (its Done when names it); text on --cell-opt and on ember cells is at least Orange, with interim rules marked for dfvp where read-only cards and tiles still sit on cells. (8) V1 mockups keep the grey palette in a local .first-draft wrapper so their committed PNGs stay true.
