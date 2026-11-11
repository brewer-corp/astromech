---
schema: atelier.item
schema_version: 3
id: astromech-mwh5
title: 'Page layouts: Board, List with detail, Workbench and Form replace the width caps'
status: ready
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-b2k3
implementers: []
ready:
  by: conversion
  at: 2026-11-10T06:04:03.897072879Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:54:57.572421657Z
---

## Goal

A page uses the window at any size without stretching rows. Today eight pages cap themselves at max-width 60rem and sit in the top-left of a big window, and Stored modules and On foot spread three columns across 1,800 px. Four shared layouts own widths, gutters and the detail panel, and a page picks one. A sparse page stays sparse: a layout places content well, it does not add any.

## Done when

- [ ] app/src/lib/ui/layout/ has Board (tile grid), ListDetail (a list and a 320 px detail panel that fills from the selected row), Workbench (inputs left, result right) and Form, built to DESIGN.md revision 4.
- [ ] One page per layout is converted and sets no width of its own: Materials (Board with a detail panel; its tiles are astromech-z5oh's), the Places finders through their shared FinderPage and SearchKit (ListDetail, the selected row's detail in the panel), Trade > Commodities (Workbench), Settings > General (Form).
- [ ] At 1100 x 700 each converted page shows everything with nothing clipped (the detail panel moves below the content when there is no room); at 2560 x 1440 and 3840 x 2160 no action is further than one column from what it acts on. Before-and-after captures at 1100 x 700, 1920 x 1080 and 2560 x 1440 are in docs/design/.
- [ ] One check, page-sizing, runs in pnpm lint and fails on a page module that sizes itself. It has two parts, each switched on per file from one list of files and globs in app/eslint.config.js, so a module can be enrolled a page at a time: width (a max-width or fixed width declaration in a file under app/src/modules/; a condition in a media query and max-width: var(--measure) on prose are allowed) and font size (any font-size declaration; added by astromech-z5oh). This item builds the check with both parts and its tests, and turns the width part on for the files it converts (for Places that is FinderPage and the finder pages, not Home.svelte, which astromech-gvy0 converts). Exempt from both parts for good: the Controls modules (controller, keybinds, presets, keytester), and the frame's parts that live in a page module (route/Strip.svelte and route/PlotBar.svelte), which are not pages.
- [ ] pnpm check, pnpm lint and pnpm test pass.

## Scope

The layout components, the four conversions and the page-sizing check. The other pages convert in their own items (the coverage table in the strategy says which).

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. The layouts are visible in glow-up-materials.png (Board with detail) and glow-up-sell.png (ListDetail). UI rules: CONTRIBUTING.md (UI check; a visual change gets captures, not a test).

## Unknowns

Whether page-sizing is an ESLint rule over the Svelte style block or a small script that pnpm lint runs; either, as long as it fails the lint and has tests.
