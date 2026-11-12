---
schema: atelier.item
schema_version: 3
id: astromech-v4bw
title: Explanations move from page prose into Tips
status: in_review
mission: astromech-xajc
labels:
- size-s
priority: medium
blocked_by:
- astromech-sixj
implementers:
- thread:delegated-task:command%3Amcp%3Ac95b33ef-c19f-4414-ad69-248cbccdf307%3Adelegate-task%3A9c80e690-22c8-428d-90cf-657a3fee81fb
ready:
  by: conversion
  at: 2026-11-10T06:04:05.980926316Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:02.350811962Z
---

## Goal

A page says what it shows and no more; how to read it is one hover away. Today Powerplay prints a paragraph of rules under each of 13 activities, and Exploration, Biology, Session, Mining, On foot, Station and Trade carry footnote paragraphs, one naming scripts/gen-odyssey.py.

## Done when

- [x] Each explanatory paragraph or footnote on Powerplay, Exploration, Biology, Session, Mining, On foot, Station and Trade is either in a Tip on the thing it explains or deleted; none remains as page prose. Data-source and data-age facts a commander needs to trust a number ("Ardent Insight, 24 d ago") stay, as one Label line.
- [x] No page names a script, a fixture or a 'sample' source outside the browser preview.
- [x] docs/guide still explains what was removed from the pages; scripts/check-guide.py passes.
- [x] Before-and-after captures of Powerplay and Exploration are in docs/design/; pnpm check, pnpm lint and pnpm test pass.

## Scope

Not the Route page: its on-page help goes with its redesign (astromech-irrv).

## Context

Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Tip is astromech-sixj.

## Report

The eight pages now say what they show; what a figure, column or tag means is in a Tip on it.

- **Powerplay**: each activity's how, and an unverified rule's reason, are in a rich Tip on its
  name (title, how, "Not checked in game: …"). Ethos and Weak spot badges explain themselves in
  their own Tip, so the key under the list is gone. The foot is one Label line, "Community guides,
  as of 16 Oct 2026 (rules v1)", with "Frontier rebalances Powerplay often" in its Tip. The week
  merits note is a Tip on the week's count; "As of your last jump" stays as one Label line and the
  "system map, not the HUD" sentence is deleted (the guide says it).
- **Exploration**: the footnote is split into Tips on Value now, Mapping the best, the Value and
  Mapped heads, and the Worth mapping and First footfall likely badges.
- **Biology**: the sampling rule is a Tip on the Sample N tile ("On A 2" stays); "Predicted from
  the scan" keeps its words with the DSS in a Tip; the Vista Genomics note is a Tip on both Value
  heads; the source line is now "Spansh, from other players' EDDN reports".
- **Session**: the Earned note is a Tip on the Earned tile; the three parentheticals under Also
  are Tips on their lines.
- **Mining**: the note is now "Since … UTC" with "Docking ends the run" in its Tip; the rate's
  rule is a Tip on Per hour.
- **On foot**: the barter, Sells for and Mission notes are Tips on those heads; the bartender
  assumption is a Tip on the Bartender trade heading; each "(?)" carries its own Tip. The foot is
  "Game data: Elite Dangerous Wiki and EDCD/FDevIDs, 2026-10-16", with the unsure-values count
  in its Tip.
- **Station**: the market's two explanation lines become one Label line ("Market as of … UTC
  (… ago) · Ardent Insight, asked … ago") beside Check again; the comparison rule is a Tip on both
  Best nearby heads; "Not looked up" keeps its words with the dozen-per-dock rule in a Tip; the
  search how-to is a Tip on the module box.
- **Trade**: the one-line description moved to Tips on Single hop and Loop.

Scripts and samples: the On foot source came from `data/catalog/odyssey.json` and named
`scripts/gen-odyssey.py`. The generator, the catalog and the preview fixture now say "Elite
Dangerous Wiki and EDCD/FDevIDs". Every other `source` a page shows ("Sample system", "Sample
history") is set only by the browser preview (`preview/recorded.ts` `labelled`); the core sends
none.

`Badge` now spreads its attributes, so it takes `{@attach tip(…)}`; its unused `title` prop is
gone.

Guide: `using-the-app.md` says pages keep explanations in Tips and source and age in one line;
`powerplay.md`, `exploration.md`, `mining.md`, `engineering.md` and `logbook.md` say where to
point. `scripts/check-guide.py`: 22 pages, 0 problems.

Captures in `docs/design/`: `tips-powerplay-before.png`, `tips-powerplay-after.png`,
`tips-powerplay-hover-after.png`, `tips-exploration-before.png`, `tips-exploration-after.png`,
`tips-exploration-hover-after.png` (1920 x 1080, browser preview).

Checks: `pnpm check` 0 errors 0 warnings; `pnpm lint` clean; `pnpm test` passed (577 tests after
round 1 removed the Suggestions tests). The browser project once failed to start on "Port 63315 is already in use" (another
checkout's run); a rerun passed. Tests changed: the Suggestions tests read the how, the tag
meanings and the unverified reason from each target's Tip (`aria-describedby`); the Powerplay
page test counts one Ethos badge (the key is gone); the old-address test looks for Trade's Places
line instead of the removed description.

### Responses

- 1-bvka.1: fixed in bb269601: `docs/guide/exploration.md` says a species nobody had logged pays five times.
- 1-bvka.2: fixed in bb269601: `Suggestions.browser.test.ts` deleted; Tip's own tests cover the description.
- 2-svbq.3: fixed in 5fe6d164: the "shows what earns merits" case is removed; 576 tests pass.
