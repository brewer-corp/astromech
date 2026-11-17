---
schema: atelier.item
schema_version: 3
id: astromech-8e22
title: 'Increment 2 checkpoint: the new frame, colours and tabs seen together'
status: ready
mission: astromech-xajc
labels:
- size-s
- feature
priority: high
blocked_by:
- astromech-4wi1
- astromech-uc7i
- astromech-5a01
- astromech-7did
- astromech-3ca6
implementers: []
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.503240761Z
---

## Goal

The reworked foundation is seen whole before the screens are built on it: the new colours, the text size, the route sidebar, the new tabs and the grouped rows, on the real pages.

## Done when

- [ ] An agent that built none of increment 2 captures, in the browser preview, the Sell finder (a result selected, sidebar open), Materials, Engineers with "laser" searched, Ships, Suits, Powerplay, Settings and My controller at 1100 x 700 and 1920 x 1080 (100 percent), 2560 x 1440 (145 percent) and 3840 x 2160 (180 percent), into `docs/design/increment2/`.
- [ ] Each capture is measured as astromech-irtr's were: no sideways scroll, no clipped or overlapping text, no readable text under 4.5:1, no action further than one column from what it acts on. A failure is filed as a bug in the mission and fixed before this item closes.
- [ ] On each page the brightest things are the main value, what can be changed and what needs the commander; a page where something else out-shines them is filed.
- [ ] Preview links for the Sell finder and Engineers are published with `scripts/preview-publish` and recorded with the capture paths as a note on the mission.
- [ ] The measured dock width, header second-row width and rows-visible figures in DESIGN.md match what the captures show.

## Scope

Checking and evidence. It changes no page; it files what it finds.

## Context

Mission astromech-xajc. This checkpoint does not stop the mission: the screens start when it merges, and Sumeet sees the links when he next looks.
