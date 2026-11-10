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
updated_at: "2026-11-10T05:54:16.662362547+00:00"
---

## Outcome

A commander opens the window on a page about what they are doing now. Every page fills the window from 1100 x 700 to 4K, shows one obvious main value per element, and uses a screen built for its job where a table was. Text is readable from a seat next to the game. The plan, the design-system amendments and the mockups are in docs/strategies/glow-up.md.

## Success criteria

- fills-the-window: every page is built on one of the four layout components and no page module sets its own max-width; shown by a lint rule and screenshots of every page at 1100 x 700, 1920 x 1080 and 2560 x 1440.
- glance-test: on every page a reader can point to the main value, the thing they can change and the thing that needs them without reading; shown by an independent validation over the 2560 x 1440 screenshots.
