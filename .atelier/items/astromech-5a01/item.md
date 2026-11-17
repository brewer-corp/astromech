---
schema: atelier.item
schema_version: 3
id: astromech-5a01
title: 'Top tabs: SHIPS splits from ENGINEERING, and On foot stops being a page'
status: ready
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-3kf5
- astromech-uc7i
implementers: []
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.350695414Z
---

## Goal

The tabs say what is in them. ENGINEERING today holds Ships, Stored modules, Engineers, Materials and On foot: the first two are what the commander owns, and On foot is a small copy of the whole section cut by mode. Sumeet, 16 Nov 2026: "the page selection feels very weird".

## Done when

- [ ] Top tabs in order: SHIPS, ENGINEERING, PLACES, TRADE, EXPLORATION, POWERPLAY, LOGBOOK, CONTROLS, SETTINGS (NOW joins at the front with astromech-coq6). SHIPS holds Ships, Stored modules and Suits; ENGINEERING holds Materials and Engineers.
- [ ] On foot is no longer a page. Its parts move whole, restyled only as far as their new page needs: loadouts, blueprints, the shopping list and the bartender trade to Ships > Suits; the inventory to Materials as an On foot kind beside Raw, Manufactured and Encoded; the suit engineers to Engineers, beside the ship engineers (this takes in astromech-eq5k's first part).
- [ ] `pnpm lint` passes with the lint astromech-4wi1 added: the On foot files' entry in `TOKENS_EXEMPT` moves with the files to the pages that take them, or the moved parts are made clean, so Materials, Engineers and Suits do not start failing.
- [ ] Old links land: `#/odyssey` and each `?view=` go to the page that now holds that part; `#/engineering`, `#/stored` and the palette's entries still open their pages under the new tab.
- [ ] The header's second tab row comes in at the width the nine tabs this item ships need, measured at each Text size step, and DESIGN.md's header widths are updated with the measurement. astromech-coq6 measures again when NOW joins.
- [ ] `app/src/tabrows.test.ts` passes for the new tree and excludes the Ships rail and the Presets grid by role, not by markup (astromech-eq5k's second part).
- [ ] The guide's pages follow the pages: an On foot guide page no longer exists as such, and Ships, Materials and Engineers say what moved in. Captures of each moved page at 2560 x 1440.

## Scope

Where pages live. The GALAXY to PLACES rename is astromech-uc7i's, which lands first. The Suits screen's redesign is astromech-nnn0, the engineer tree astromech-m6gw, the engineer search astromech-7did.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). `5-ships-2560.png` and `7-materials-on-foot-2560.png` show the tab row and the On foot kind. The `odyssey` module holds inventory, blueprints, engineers and the bartender in one View.
