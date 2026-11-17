---
schema: atelier.item
schema_version: 3
id: astromech-5g9w
title: 'Ships: Open in Coriolis and EDSY sits over 1,000 px from the ship it opens on a wide window'
status: dropped
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-17T10:08:06.657503384Z
---

## Goal

On Ships > Ships, Open in Coriolis and EDSY sit next to the ship they open (The Next To It Rule). Found by astromech-8e22 on 17 Nov 2026: the links are at the right edge of the page column, about 1,000 layout px from the ship's name at 2560 x 1440 (145 percent) and more at 3840 x 2160 (180 percent). The stats band (Value, Rebuy, Engineered) and the Speed, Pitch, Jump and Power band stretch the same way. Captures: docs/design/increment2/ships-2560x1440.png and ships-3840x2160.png.

## Done when

- [ ] At 2560 x 1440 and 3840 x 2160, Open in sits within one column of the ship it acts on, and the page leaves spare width empty instead of stretching its bands.

## Scope

Ships > Ships. astromech-8wr4 rebuilds this page as a loadout and carries this case as a Done when line: astromech-8e22 handed it there and closed (its second Done when line names the handoff).

## Report

Dropped: Another item covers it: astromech-8wr4 rebuilds the page and carries this case as a Done when line.
