---
schema: atelier.item
schema_version: 3
id: astromech-irtr
title: 'Increment 1 checkpoint: the foundation and the Route page seen together'
status: approved
mission: astromech-xajc
labels:
- validation
priority: high
blocked_by:
- astromech-b2k3
- astromech-drnv
- astromech-dsyu
- astromech-k6pl
- astromech-mwh5
- astromech-ok29
- astromech-q79q
- astromech-sixj
- astromech-v4bw
- astromech-z5oh
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A173cd11b-5725-4326-8aff-742d50f27fd1
ready:
  by: conversion
  at: 2026-11-10T06:04:06.710935433Z
trivial: null
caused_by: null
created_at: 2026-11-10T06:04:06.710935433Z
---

## Goal

Increment 1 of mission astromech-xajc is proven as a whole before any other screen is planned: the Route page really stands on the foundation, the converted pages hold at every size, and Sumeet has a link to look at. This is the mission's one planned stop (strategy, Authority).

## Done when

- [x] On main, with every item this one waits on merged: an agent that wrote none of them captures the Route page and each converted page (Materials, a Places finder, Trade > Commodities, Settings > General, Powerplay, Exploration) at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160 from the browser preview and reports, per capture, anything clipped, overlapping, stretched so an action is far from its row, or below 12 px before scaling. Each problem is fixed or filed as a bug on astromech-xajc.
- [x] It confirms from the code that app/src/modules/route uses a layout component, the type roles, Tip and Glyph's star icons, and that each part of the page-sizing check is on for exactly the files increment 1 promised: the width part for the files astromech-mwh5 converted, the font-size part for the files astromech-z5oh enrolled, and both parts for the Route page's own files (app/src/modules/route/View.svelte and the components that page is built from; Strip.svelte and PlotBar.svelte are frame parts and exempt). The two parts are enrolled separately, so a file can be on one list and not the other: Places home (app/src/modules/places/Home.svelte) has the font-size part on, from astromech-z5oh, and the width part off until astromech-gvy0. A file neither item converted is on neither list.
- [x] scripts/preview-publish output for the Route page and for Materials is published and both links are in a note on astromech-xajc, with the captures' paths.
- [x] The result is an evidence receipt on astromech-xajc.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. After this, the orchestrator stops and sends Sumeet the links. Increments 2 to 4 (astromech-coq6 and the screens) are planned after he answers.

## Report

Increment 1 holds together: the Route page and the converted pages show nothing clipped, overlapping or too small at any of the four sizes, and Sumeet has two links to open. No bug needed filing.

**Captures.** 32 captures of 8 page states (Route with an exact route; Materials; the Sell finder, plain and with a result selected; Trade > Commodities; Settings > General; Powerplay; Exploration) at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160, from the browser preview (`pnpm dev`, `?core=ready`) in headless Chrome at device scale 1. They are in `docs/design/increment1/`, with `report.json` holding each capture's measurements:

- the root zoom (1, 1.08, 1.44, 1.8);
- horizontal scroll of the document and of `main`: none;
- text cut by an ancestor that hides overflow: none;
- overlapping text or controls in view, ignoring content scrolled under the status bar: none;
- text below 12 px computed before the zoom: none;
- a row action more than 360 layout px from its row's last text: none.

I also looked at every page at 1100 x 700 and 3840 x 2160. Every action sits at the end of its row, its field, or the inputs it submits. At 1100 x 700 the Sell finder's panel stacks below the list as astromech-mwh5 built it.

Two things looked loose at 4K but move no action from its row, so I did not file them:

- Exploration's bodies table spreads to the full width, with about 640 px between a body's tags and its value. astromech-dpaz replaces that table.
- The Sell finder's From input runs the list's full width.

**Code.** Checked from ESLint's resolved config for every `.svelte` file under `app/src/modules`, not just the list in `eslint.config.js`:

- The width part is on exactly for astromech-mwh5's files: materials/View, places/FinderPage, the nine finder pages, trade/View and trade/Route, and settings/View.
- The font-size part is on exactly for astromech-z5oh's files: materials/View, places/Home, settings/View, Callouts, ProspectorThresholds, session/View and mining/View.
- Both parts are on for the Route page's files: View, Timeline, Title, PlotSetup, ImportDialog and NeutronFile.
- No other file is on either list. Places Home has font-size only. Strip and PlotBar are exempt. route/Plot.svelte is off both lists, because only PlotBar, a frame part, draws it.

The Route page uses the layout and the shared parts:

- `route/View.svelte` is built on `Board`.
- It uses the type roles: `type-title`, `type-name`, `type-label`, `type-figures` and `type-body`.
- `Timeline.svelte` draws stars with Glyph's `star-*` icons (picked by `starKind` in `lib/route.ts`) and puts Tips on them through `tip`. Title's help button uses Tip.

`pnpm lint` passes.

**Links.** Published with `scripts/preview-publish` and listed in a note in `.atelier/missions/astromech-xajc/notes.md` with the captures' paths:

- Route: https://claude.ai/artifact/Syy9cQByFc1AAqD9UJaVbQ (`--page route --query route=exact`)
- Materials: https://claude.ai/artifact/1N4Dkdu8crQgLJyxw8KNPH (`--page materials`)

Both are private until shared from the page's Share menu. The evidence receipt is the `[evidence]` entry in the same notes file. Atelier has no receipt command now, so the receipt follows astromech-kjfs's `[note]` form.
