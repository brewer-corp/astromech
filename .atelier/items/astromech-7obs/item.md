---
schema: atelier.item
schema_version: 3
id: astromech-7obs
title: 'The Sell dock-width test fails on main: the sidebar''s dock width was measured before the grouped rows'
status: ready
mission: astromech-xajc
labels:
- bug
- size-s
priority: high
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T09:13:15.774735374Z
trivial: null
caused_by: astromech-3ca6
created_at: 2026-11-17T09:13:15.683726201Z
---

## Goal

`pnpm test` passes on main, and the route sidebar docks at the width the pages need today. astromech-uc7i measured the dock width (1,756 px at 100 percent Text size) against the one-line Sell row and left a browser test in `App.browser.test.ts` that fails when the row's width changes. astromech-3ca6 then rebuilt the row in groups, and the test has failed on main since 435705e (seen by astromech-uzg6's implementer and reviewer, 736 of 737 passing).

## Done when

- [ ] The dock width is measured again against the grouped Sell row with a result selected, at every Text size step, and set to the narrowest width at which the rows stay clear of the detail panel beside the sidebar. The test passes because the number is right, not because the test was loosened.
- [ ] The Report gives the new width and what it means for the windows that matter: a maximised 2560 x 1440 window at 145 percent (about 1,765 px of layout) docked by about 10 px before; say what the margin is now, and whether a 1920 x 1080 window at 100 and at 115 percent docks.
- [ ] If the grouped row lets the sidebar dock on narrower windows, it does. DESIGN.md's Route sidebar section has the measured width.
- [ ] `pnpm test`, `pnpm lint` and `pnpm check` pass.

## Scope

The dock width and its test. The finder header above the rows is astromech-fom5, which does not change a row's width.
