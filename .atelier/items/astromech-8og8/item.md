---
schema: atelier.item
schema_version: 3
id: astromech-8og8
title: 'Suits: Remove sits over 1,000 px from the plan it removes on a wide window'
status: draft
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T10:08:06.608284491Z
---

## Goal

On Ships > Suits, Remove sits next to the plan it removes at any window size (The Next To It Rule). Found by astromech-8e22 on 17 Nov 2026: each plan block and the shopping list run the page column's full width, so a plan's cost and Remove sit at the right edge. Remove is about 1,100 layout px from the plan's name at 2560 x 1440 (145 percent) and about 1,650 at 3840 x 2160 (180 percent), and the shopping list's Need, Have, Short and Barter columns spread over 1,500 px. Captures: docs/design/increment2/suits-2560x1440.png and suits-3840x2160.png.

## Done when

- [ ] At 2560 x 1440 and 3840 x 2160, no action on Suits is more than one column from what it acts on, and the shopping list is as wide as its columns.

## Scope

Ships > Suits. astromech-nnn0 rebuilds this page as a loadout but waits on astromech-8e22, which waits on this fix; the lead decides whether to fix the current page or let 8e22 close with this case handed to nnn0.
