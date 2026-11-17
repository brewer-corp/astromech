---
schema: atelier.item
schema_version: 3
id: astromech-5a01
title: 'Top tabs: SHIPS splits from ENGINEERING, and On foot stops being a page'
status: approved
mission: astromech-xajc
labels:
- size-m
- feature
priority: high
blocked_by:
- astromech-3kf5
- astromech-uc7i
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Ac6e1e425-3280-4881-864e-a15cb9a8e2a2
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

- [x] Top tabs in order: SHIPS, ENGINEERING, PLACES, TRADE, EXPLORATION, POWERPLAY, LOGBOOK, CONTROLS, SETTINGS (NOW joins at the front with astromech-coq6). SHIPS holds Ships, Stored modules and Suits; ENGINEERING holds Materials and Engineers.
- [x] On foot is no longer a page. Its parts move whole, restyled only as far as their new page needs: loadouts, blueprints, the shopping list and the bartender trade to Ships > Suits; the inventory to Materials as an On foot kind beside Raw, Manufactured and Encoded; the suit engineers to Engineers, beside the ship engineers (this takes in astromech-eq5k's first part).
- [x] `pnpm lint` passes with the lint astromech-4wi1 added: the On foot files' entry in `TOKENS_EXEMPT` moves with the files to the pages that take them, or the moved parts are made clean, so Materials, Engineers and Suits do not start failing.
- [x] Old links land: `#/odyssey` and each `?view=` go to the page that now holds that part; `#/engineering`, `#/stored` and the palette's entries still open their pages under the new tab.
- [x] The header's second tab row comes in at the width the nine tabs this item ships need, measured at each Text size step, and DESIGN.md's header widths are updated with the measurement. astromech-coq6 measures again when NOW joins.
- [x] `app/src/tabrows.test.ts` passes for the new tree and excludes the Ships rail and the Presets grid by role, not by markup (astromech-eq5k's second part).
- [x] The guide's pages follow the pages: an On foot guide page no longer exists as such, and Ships, Materials and Engineers say what moved in. Captures of each moved page at 2560 x 1440.

## Scope

Where pages live. The GALAXY to PLACES rename is astromech-uc7i's, which lands first. The Suits screen's redesign is astromech-nnn0, the engineer tree astromech-m6gw, the engineer search astromech-7did.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). `5-ships-2560.png` and `7-materials-on-foot-2560.png` show the tab row and the On foot kind. The `odyssey` module holds inventory, blueprints, engineers and the bartender in one View.

## Report

The tabs now say what is in them, and On foot is gone as a page with nothing it showed lost.

**Tabs.** `lib/modules.ts` orders SHIPS, ENGINEERING, PLACES, TRADE, EXPLORATION, POWERPLAY, LOGBOOK, CONTROLS, SETTINGS. Ships (`engineering`) and Stored modules move to `ships`; Suits is new there. The window still opens on My controller: NOW and the default page are astromech-coq6's.

**On foot's parts.**
- Ships > Suits is `modules/suits`, which is `modules/odyssey` moved with `git mv`. It keeps the owned gear line, picker, pick cards, shopping list and bartender trade unchanged, and drops only the view choice. Its `TOKENS_EXEMPT` entry moves with it, until astromech-nnn0.
- Materials > On foot is a kind added after Other. It has a tile section per ShipLocker category, Held only and All items, and the held total with what a bartender pays. A mission count is a tile's corner mark. The locker, backpack, mission, barter and sale lines are in the side panel. The table became tiles because Materials is a Board of tiles, as in the approved `7-materials-on-foot-2560.png`. I left out that capture's "need N" marks and "Wanted by" panel: they cross-reference plans, which is new design work. These parts are written lint-clean, so Materials needs no exemption. `odyssey_view` is polled only while On foot is chosen. Selection is `#/materials/onfoot/<id>`, so an item id can never select a material.
- Engineers > Suit engineers cards list every mod from the Odyssey data, with the "(?)" Tip, and Colonia. If the read fails, the cards fall back to the two-item specialties. This was astromech-eq5k's first part. Each suit card also says what that engineer asks to refer the next ("Refers Wellington Beck: Provide 5 Settlement Defence Plans"). The tree shows only "Unlock <name> first" on the referred card until the introducer is unlocked, so this line is how the items can be collected ahead. On foot's "next step" line did not move: the tree's own line covers it.

**Old links.** `App.svelte` maps `#/odyssey` and `?view=inventory` to `materials?kind=onfoot`, `?view=blueprints` to `suits`, and `?view=engineers` to `unlocks?kind=suit`. `#/engineering` and `#/stored` keep their ids. Find or plot lists every page by manifest and gains an "On foot" entry that opens the kind. No Rust code or journey uses these routes. A test checks each old address against the top tab and page tab it lands on.

**Tab-row test.** `tabrows.browser.test.ts` (the item said `tabrows.test.ts`) now leaves out rows by role: rows inside a table (`table`, `[role=table]`, `[role=grid]`) and rows inside a named navigation landmark (`nav[aria-label]`, the Ships rail "Your ships"). The picture and mixed-markup heuristics are gone. To give Presets a role, its grid is now an ARIA table with column heads, so each cell is named by its context. That is the one change outside the moved pages. The top tabs and page tabs are unnamed `nav`s, and a new case checks that both rows are still counted, so labelling them later would fail the test instead of silently skipping them.

**Header width.** Measured in the preview by bisecting the window width at each step. The header fits on one row from 95.5 rem: 1529 px at 100 percent, 1757 at 115, 1987 at 130, 2216 at 145, 2446 at 160 and 2751 at 180. So 1920 x 1080 and 2560 x 1440 at their starting steps show one row. That is in DESIGN.md's Header row. `oneRow()` already measures, so no code changed. Finish setup is not measured: the preview has no scenario with setup unfinished after onboarding. DESIGN.md says the button adds its own width.

**Guide.** `engineering.md` is now "Engineers", with the suit mod list. `ships.md` gains `## Suits` (the manifest's `help` anchor). `materials.md` gains `## On foot`. `index.md`, `using-the-app.md` (the areas table, which listed Controls first) and V1-DESIGN's navigation paragraph and On foot row follow. `scripts/check-guide.py` finds 0 problems.

**Captures**, 2560 x 1440 at Text size 145, in `docs/design/`:
- after: `tabs-ships-2560-after.png`, `tabs-suits-2560-after.png`, `tabs-materials-on-foot-2560-after.png`, `tabs-materials-on-foot-selected-2560-after.png`, `tabs-engineers-suit-2560-after.png`;
- Stored modules: `tabs-stored-2560-before.png` and `tabs-stored-2560-after.png`;
- before, from `main`: `tabs-ships-2560-before.png`, `tabs-on-foot-{inventory,blueprints,engineers}-2560-before.png`, `tabs-engineers-suit-2560-before.png`;
- the stacked header at 1100 x 700: `tabs-header-1100-after.png`.

**Checks.** `pnpm lint && pnpm check && pnpm build && pnpm test` pass (86 files, 723 tests), and so do `scripts/check-dist.py` and `scripts/check-guide.py`. No Rust changed, so I did not run cargo.

### Responses

1-l4sg.1: fixed in 2242b5dd, with a test that fails without it (ShipLocker pending, then failed, with the ship materials answered).
1-l4sg.2: fixed in 8937386b, with a check that Hero Ferrari's card says "Refers Wellington Beck: Provide 5 Settlement Defence Plans" before Hero is unlocked.
1-l4sg.3: fixed in 6cc109e7; the before captures from `main` draw in a fallback font, but the content and tabs are the same.

