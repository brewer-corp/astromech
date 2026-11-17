---
schema: atelier.item
schema_version: 3
id: astromech-coq6
title: 'Now page: the window opens on what you are doing'
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
created_at: 2026-11-10T05:55:38.805008138Z
---

## Goal

The window opens on a page that follows the game: docked shows the station and its services, in space the system, with the best sale for the cargo aboard, Powerplay cycle time and session earnings. Station, Session and Mining move here from Logbook; the app's default page becomes Now.

## Done when

- [ ] (provisional) NOW is the first top tab; the header's second-row width is measured again with ten tabs at each Text size step and DESIGN.md updated.
- [ ] (provisional) A new module app/src/modules/now in a first nav group NOW; it shows the docked, in-system and not-running states from existing replies; old links to station, session and mining still open.
- [ ] (provisional) Built on the layout components and type roles with no page-level width or font size; captures at 1100 x 700 and 2560 x 1440 in docs/design/; the page's guide in docs/guide updated.

## Context

Colours, selection, result rows, text size and the tabs follow DESIGN.md revision 5 (astromech-n41x); captures are taken at 1100 x 700 at 100 percent and 2560 x 1440 at 145 percent. Strategy: .atelier/missions/astromech-xajc/ (mission astromech-xajc). Mockup: docs/design/glow-up-now.png and glow-up-now-1100.png. Preliminary: planned in detail after increment 1 (the foundation and the Route page astromech-ok29) has merged and Sumeet has seen its preview link.

## Unknowns

Which facts each state shows, and whether Station, Session and Mining become sections of Now or page tabs under it. Whether any new core command is needed (none expected).
