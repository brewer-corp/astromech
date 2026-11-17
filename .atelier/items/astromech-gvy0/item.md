---
schema: atelier.item
schema_version: 3
id: astromech-gvy0
title: The remaining pages move onto the layouts
status: draft
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T06:04:11.267990188Z
---

## Goal

Every page outside Controls that gets no new screen still stops sizing itself: Stored modules (three columns spread over 1,800 px today), Places home, Mining, Data sharing, Diagnostics and Voice tester.

## Done when

- [ ] (provisional) With this merged, `TOKENS_EXEMPT` (astromech-4wi1) holds only the Controls modules and the mockups. That includes `Button.svelte`, exempt today because DESIGN.md gives the primary button 20 px padding, which is not a step on its spacing scale: the button moves to a step, or DESIGN.md adds one, and says which.
- [ ] (provisional) Each page named is on a layout component and the type roles, and the page-sizing check is on for its module. Stored modules groups its rows by where the module is. `settings/Callouts.svelte`, `settings/ProspectorThresholds.svelte` and `route/Plot.svelte` are enrolled or listed exempt (left open by astromech-mwh5 and astromech-ok29).
- [ ] (provisional) With this merged and the screen items done, both parts of the page-sizing check are on for every file under app/src/modules/ except the Controls modules and the frame's parts in the route module (the files astromech-uc7i lists exempt); this item enrols whatever file is still off the list.
- [ ] (provisional) Captures at 1100 x 700 and 2560 x 1440 in docs/design/.
- [ ] (provisional) As the last screen item, it owns the fills-the-window evidence: every page outside Controls captured as finally built at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160 (at the Text size the mission's criterion gives each), measured as astromech-irtr's were, and recorded as the criterion's evidence on the mission. It is blocked on every other screen item when it is made ready.

## Context

Colours, selection, result rows, text size and the tabs follow DESIGN.md revision 5 (astromech-n41x); captures are taken at 1100 x 700 at 100 percent and 2560 x 1440 at 145 percent. Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Preliminary: planned in detail after the increment 1 checkpoint.

## Unknowns

Whether Mining stays a page or becomes part of the Now page (astromech-coq6 decides).
