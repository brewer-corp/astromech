---
created_at: "2026-11-10T06:04:06.710935433+00:00"
id: "astromech-irtr"
issue_type: "validation"
labels: []
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
    id: "astromech-drnv"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-dsyu"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-k6pl"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-mwh5"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-ok29"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-q79q"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-sixj"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-v4bw"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
  - kind: "issue"
    id: "astromech-z5oh"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Increment 1 checkpoint: the foundation and the Route page seen together"
updated_at: "2026-11-10T06:04:06.710935433+00:00"
---

## Goal

Increment 1 of mission astromech-xajc is proven as a whole before any other screen is planned: the Route page really stands on the foundation, the converted pages hold at every size, and Sumeet has a link to look at. This is the mission's one planned stop (strategy, Authority).

## Done when

- [ ] On main, with every item this one waits on merged: an agent that wrote none of them captures the Route page and each converted page (Materials, a Places finder, Trade > Commodities, Settings > General, Powerplay, Exploration) at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160 from the browser preview and reports, per capture, anything clipped, overlapping, stretched so an action is far from its row, or below 12 px before scaling. Each problem is fixed or filed as a bug on astromech-xajc.
- [ ] It confirms from the code that app/src/modules/route uses a layout component, the type roles, Tip and Glyph's star icons, and that each part of the page-sizing check is on for exactly the files increment 1 promised: the width part for the files astromech-mwh5 converted and for app/src/modules/route, the font-size part for the files astromech-z5oh enrolled and for app/src/modules/route. The two parts are enrolled separately, so a file can be on one list and not the other: Places home (app/src/modules/places/Home.svelte) has the font-size part on, from astromech-z5oh, and the width part off until astromech-gvy0. A file neither item converted is on neither list.
- [ ] scripts/preview-publish output for the Route page and for Materials is published and both links are in a note on astromech-xajc, with the captures' paths.
- [ ] The result is an evidence receipt on astromech-xajc.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. After this, the orchestrator stops and sends Sumeet the links. Increments 2 to 4 (astromech-coq6 and the screens) are planned after he answers.
