---
created_at: "2026-11-10T05:54:57.572421657+00:00"
id: "astromech-mwh5"
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
    id: "astromech-b2k3"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Page layouts: Board, List with detail, Workbench and Form replace the width caps"
updated_at: "2026-11-10T05:54:57.572421657+00:00"
---

## Goal

A page fills the window at any size without stretching rows. Today eleven page modules cap themselves at max-width 60rem and sit in the top-left of a big window, and Stored modules and On foot spread three columns across 1,800 px. Four shared layouts own widths, gutters and the detail panel, and a page picks one.

## Done when

- [ ] app/src/lib/ui/layout/ has Board (tile grid), ListDetail (a list and a 320 px detail panel that fills from the selected row), Workbench (inputs left, result right) and Form, built to DESIGN.md revision 4.
- [ ] One page per layout is converted and sets no width of its own: Materials (Board), a Places finder (ListDetail, with the selected row's detail in the panel), Trade > Commodities (Workbench), Settings > General (Form).
- [ ] At 1100 x 700 each converted page shows everything with nothing clipped (the detail panel moves below the content when there is no room); at 2560 x 1440 no action is further than one column from what it acts on. Before-and-after captures at 1100 x 700, 1920 x 1080 and 2560 x 1440 are in docs/design/.
- [ ] A rule in app/eslint-rules fails on max-width in a file under app/src/modules/, enabled for the converted modules, with its own test.
- [ ] pnpm check, pnpm lint and pnpm test pass.

## Scope

The layout components, the four pages and the lint rule. The other pages convert in their own items.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. The caps: grep max-width app/src/modules. The layouts are visible in the mockups glow-up-materials.png (Board with detail), glow-up-sell.png (ListDetail). UI rules: CONTRIBUTING.md (UI check, tests: a visual change gets captures, not a test).
