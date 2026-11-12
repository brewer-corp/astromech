---
schema: atelier.item
schema_version: 3
id: astromech-irtr
title: 'Increment 1 checkpoint: the foundation and the Route page seen together'
status: building
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

- [ ] On main, with every item this one waits on merged: an agent that wrote none of them captures the Route page and each converted page (Materials, a Places finder, Trade > Commodities, Settings > General, Powerplay, Exploration) at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160 from the browser preview and reports, per capture, anything clipped, overlapping, stretched so an action is far from its row, or below 12 px before scaling. Each problem is fixed or filed as a bug on astromech-xajc.
- [ ] It confirms from the code that app/src/modules/route uses a layout component, the type roles, Tip and Glyph's star icons, and that each part of the page-sizing check is on for exactly the files increment 1 promised: the width part for the files astromech-mwh5 converted, the font-size part for the files astromech-z5oh enrolled, and both parts for the Route page's own files (app/src/modules/route/View.svelte and the components that page is built from; Strip.svelte and PlotBar.svelte are frame parts and exempt). The two parts are enrolled separately, so a file can be on one list and not the other: Places home (app/src/modules/places/Home.svelte) has the font-size part on, from astromech-z5oh, and the width part off until astromech-gvy0. A file neither item converted is on neither list.
- [ ] scripts/preview-publish output for the Route page and for Materials is published and both links are in a note on astromech-xajc, with the captures' paths.
- [ ] The result is an evidence receipt on astromech-xajc.

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. After this, the orchestrator stops and sends Sumeet the links. Increments 2 to 4 (astromech-coq6 and the screens) are planned after he answers.
