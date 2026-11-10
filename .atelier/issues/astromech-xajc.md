---
created_at: "2026-11-10T05:54:16.662362547+00:00"
id: "astromech-xajc"
issue_type: "mission"
labels: []
strategy:
  path: docs/strategies/glow-up.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Glow-up: every page reads at a glance and fills the window"
updated_at: "2026-11-10T06:25:19.926420879+00:00"
---

## Outcome

A commander opens the window on a page about what they are doing now. Every page outside Controls uses the window well at any size from 1100 x 700 to 4K, shows one obvious main value per element, and uses a screen built for its job where a table was. A sparse page stays sparse: a layout places content, it never adds any. Text is readable from a seat next to the game. The plan, the design-system amendments and the mockups are in docs/strategies/glow-up.md.

## Success criteria

Both cover every page outside Controls (My controller, All keybinds, Presets, Key tester), which get only the frame, the scale and Tip.

- fills-the-window: every page is built on one of the four layout components and passes the page-sizing check (no page file sets its own width or font size); shown by the check being on for every page file except the Controls ones and the frame's two parts in the route module, and by captures of every page at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160 with nothing clipped and no action further than one column from what it acts on.
- glance-test: on every page a reader can point to the main value, the thing they can change and the thing that needs them, without reading; shown by astromech-crxa: expected answers written per page first ('none' where a page has nothing to change or nothing needing attention), a fresh agent answering from the 2560 x 1440 screenshots alone, and every page passing. A page left failing leaves the criterion unmet.
