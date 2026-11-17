---
schema: atelier.item
schema_version: 3
id: astromech-wo8s
title: My controller at 1100 x 700 draws the bound actions on the sheet at 8 px
status: ready
mission: astromech-xajc
labels:
- bug
priority: low
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T10:14:46.678176257Z
trivial: null
caused_by: null
created_at: 2026-11-17T10:08:06.707288043Z
---

## Goal

The bound actions on My controller's sheet are readable at the smallest window. Found by astromech-8e22 on 17 Nov 2026: at 1100 x 700 at 100 percent the sheet is 636 px wide and its text (the action names, button numbers and keys) draws at 8 px. It scales with the sheet's width, not with Text size, so it reads at 14 px at 1920 x 1080 and 18 px at 2560 x 1440. Capture: docs/design/increment2/controller-1100x700.png. This predates increment 2: the strip took no width from the page column.

## Done when

- [ ] At 1100 x 700 at 100 percent, the sheet's action names draw at 12 px or more, or the page offers a readable form of them at that size (the side panel's list, a zoom).

## Scope

My controller only. Controls get only the frame, the text size, the colours and Tip in mission astromech-xajc, so this may belong outside the mission; the lead decides.
