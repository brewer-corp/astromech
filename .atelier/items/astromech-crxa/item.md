---
schema: atelier.item
schema_version: 3
id: astromech-crxa
title: 'Glance test: an independent read of every page at 2560 x 1440'
status: draft
mission: astromech-xajc
labels:
- validation
priority: medium
blocked_by:
- astromech-gvy0
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:46.190820706Z
---

## Goal

Mission astromech-xajc's glance-test criterion: on every page outside Controls a reader can point to the main value, the thing they can change and the thing that needs them, without reading. A fresh agent that built none of the pages is tested against answers written down first.

## Done when

- [ ] The pages are every page outside Controls as astromech-gvy0 left them: Now (docked, in space, Elite not running), Station, Session, Mining, Ships (a module selected), Stored modules, Suits, Materials, Engineers (a node selected, and with "laser" searched), Places home and each Places finder with a result selected, Commodities with a trade route found, Colonisation, System (a body selected), Biology (sampling), Powerplay, Logbook, Settings > General, Data sharing, Diagnostics and Voice tester. The screenshots are astromech-gvy0's 2560 x 1440 captures at 145 percent in `docs/design/final/`, plus any state above they lack, taken the same way from the browser preview.
- [ ] The three answers are for the page's own job. The route sidebar is on every screenshot and is frame: the sheet has one entry for it (main value: the next stop; changeable: the destination; needs the commander: a refuel or off-route warning, else none), asked once, and an answer that points into the sidebar for a page is a fail for that page.
- [ ] An answer sheet lists, for every page, the expected main value, the expected changeable thing and the expected thing that needs the commander in the screenshot's state, taken from the page's job as its item's Done when lines and its guide page state it. A page with nothing to change or nothing needing attention says "none" for that question, and "none" is then the right answer.
- [ ] A fresh agent, given only the screenshots and not the sheet, answers the three questions per page; each answer is marked pass or fail against the sheet.
- [ ] Every failed page is fixed and asked again with a new agent until it passes. The criterion is met only when every page passes; a page left failing leaves it unmet unless Sumeet takes that page out of scope.
- [ ] The sheet, the answers and the marks are committed beside the captures (`docs/design/final/glance-test.md`) and recorded as a dated `[evidence]` line for glance-test in the mission's `notes.md`.

## Scope

Checking and evidence. A fix to a failing page is made here, in that page's module, or filed as a bug in the mission that blocks this item.

## Context

The criterion is in `.atelier/missions/astromech-xajc/mission.md`, Success criteria. DESIGN.md's test is in its Overview: "can a commander find the main value, the thing they can change, and the one thing that needs them, without reading?" Each screen item says what its page's three answers are meant to be; the sheet starts from those lines.
