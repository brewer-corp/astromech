---
created_at: "2026-10-24T23:27:52.556049922+00:00"
id: "astromech-yxtz"
issue_type: "task"
labels:
- "size-m"
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
status: "todo"
title: "Orange palette and caps per DESIGN.md: move pages from grey to the game's orange ramp"
updated_at: "2026-10-25T19:02:00.937904392+00:00"
---

## Goal

Move the pages onto DESIGN.md's palette: the game's orange as the base, rank and state by its brightness (lit, base, mid, dim) and a solid fill for selection, white only for titles, the current section and a key value, and caps only for the wordmark, top tabs, section headings and action words (DESIGN.md: The Intensity Rule, The One Fill Per Group Rule, The Narrow Red Rule, The Caps Are Wayfinding Rule).

## Done when

- Text on station-style pages uses `--orange-lit` / `--accent` / `--orange-mid` / `--orange-dim`; grey remains only for Controls and Settings row labels (`--options-grey`), prose (`--prose`) and the window.
- Active top tab is a solid orange box; active page tab is Orange Lit with a 2 px underline; inactive tabs are Orange Mid. Section headings are orange caps with a `--line` rule; the current section is white.
- Uppercase + `--caps` appear only in the four allowed roles.
- Warnings are an orange "!" plus words; hard errors a red "!" plus words and a recovery action; stale report ages are red with the row dimmed. No filled blocks.
- The old grey tokens are removed from `tokens.css` once unused.
- Screenshots of Engineers, Trade, Keybinds, Settings and the route error before and after.

## Context

Pages use the first grey-based tokens (`--text`, `--text-lo`, `--cell-opt`); about 140 `text-transform: uppercase` rules; the active top tab is white while the deepest tabs are orange; "Could not read the route" is a solid orange block. DESIGN.md (revised 25 Oct) keeps grey only for the window background, Options-style row labels and prose, and adds `--alarm` red for stale report ages and hard errors.
