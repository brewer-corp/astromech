---
schema: atelier.item
schema_version: 3
id: astromech-n41x
title: 'DESIGN.md revision 5: orange means act, the route in the frame, grouped rows'
status: in_review
mission: astromech-xajc
labels:
- size-m
- docs
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A40e7bd6a-75d7-481f-bc07-e18b62a688e1%3Adelegate-task%3Af3dabc04-aa30-4940-8987-97c1443ca39c
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.163538599Z
---

## Goal

DESIGN.md says what Sumeet approved on 17 Nov 2026, so every later item builds to one rulebook: pages are calm, orange means something, the route is part of the frame, results read as results, and text size is the commander's choice.

## Done when

- [x] Colour: a principle and a list. The principle: read-only data is white or grey in three steps; Orange is for the value a list is ranked by, the one main value, and what the commander can act on or change; colour beyond that is allowed where it carries a stated meaning or is one of Elite's own conventions. The list names every coloured thing and its meaning, starting from the spike's: demand against the hold (short, tight, plenty, with the bars), a pad too small, a stale report, star icons by what you do there, security, engineer state, hexagons for rank and grade. A colour not on the list needs adding to it.
- [x] The greys and the duller red step ("won't fit, won't sell") are tokens with their contrast on the window background and on Hull stated; readable text is 4.5:1 or better; the Narrow Red Rule says how the duller red differs from alarm red.
- [x] Selection is a raised row with lit text, not a solid orange fill; the filled orange button stays the one fill a page has. The One Fill Per Group rule, the States table and the Lifted Name Rule are rewritten to match (names are white or grey; the ranked value is orange).
- [x] The Lookup result row is two lines in groups (the value and what qualifies it; the place and what you need to dock; the trip), with the rows-visible figures restated. The Aligned Columns Rule is restated for groups.
- [x] A named rule: where Elite has a symbol players know (the engineer hexagon, star and station icons, the market's demand bars), use it, not a generic shape.
- [x] Frame: the route sidebar (Hull, flush left, headed ROUTE, joined to a Hull status bar; collapsed it is the strip; no route is a 48 px rail with two actions, Find or plot and Import a route, and the narrow strip's No route state offers the same two; below the dock width it opens over the page with the page dimmed), the full-width header rule, and the dimmed-page rule for overlays. Page tiles and blocks leave Hull for an outline or Hull Raised.
- [x] Navigation: the top tabs are NOW, SHIPS, ENGINEERING, PLACES, TRADE, EXPLORATION, POWERPLAY, LOGBOOK, CONTROLS, SETTINGS; SHIPS holds Ships, Stored modules, Suits; ENGINEERING holds Materials and Engineers.
- [x] Text size: the Height Scale Rule is replaced. Sizes are in rem; Text size sets the root size; a bigger window shows more and never changes text size. Container breakpoints are in rem.
- [x] Settings values: one ruling replaces the contradiction astromech-z5oh reported (the component table says 400, Typography says 400 looked thin): an input's value is Orange at 500, because it is what the commander can change.
- [x] The approved captures are on main under `docs/design/nav-spike/` (the `tone5-*` and `hier-B-*` files and the 1100 x 700, no-route, Ships and Materials ones the rules cite), and DESIGN.md links them. The earlier screen mockups' captures (`docs/design/glow-up-*.png`, on the branch `mockups/glow-up` only) come to main too, since the screen items cite them. `scripts/check-guide.py` passes.

## Scope

DESIGN.md and the reference captures. No tokens.css or component change; the items below own those.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). The spike's final report lists the DESIGN.md rules it changes; start from that list.

## Report

DESIGN.md now holds what Sumeet approved on 17 Nov 2026, so the increment 2 items build to one
rulebook. One change outside DESIGN.md: **PRODUCT.md's colour lines** still said red is only for
hard errors and selection is a solid fill, and DESIGN.md's Fixed rules cite PRODUCT.md as
binding. I rewrote those three phrases (lines 55, 69 and 95) to restate the 17 Nov decision, in
their own commit (16fda391), so it reverts on its own.

What changed in DESIGN.md (167848a3):

- **Colour.** The Orange Means Act Rule replaces the Intensity Rule. Greys in three readable
  steps (Options Grey, Prose Grey, and a new Label Grey `#8c8c8c`), plus Grey Dim `#6a6a6a` and
  Mark Off `#4f4a45` as state only, below AA on purpose, as Orange Dim already was. A new
  section, "Colours that carry a meaning", lists each coloured thing with its meaning and its
  non-colour cue: demand and supply, a pad too small, a stale report, stars in the route, the
  next stop, security, engineer state, hexagons, a short amount, a material at its cap, a
  binding conflict and the commander's position. Contrast is given on Void, Hull and Hull
  Raised.
- **The duller red is `#ca6657`, not the spike's `#c86455`.** The spike's value is 4.46:1 on
  Hull Raised, so a short or unfit selected row would fail 4.5:1. `#ca6657` is 5.1 / 4.9 / 4.57.
  Alarm red has almost the same contrast (4.53 on Hull Raised), so the Narrow Red Rule tells the
  two reds apart by use and saturation, not brightness: alarm red always comes with a "!" and a
  recovery action, and Red Low marks a fact and always has a second cue.
- **Selection and fills.** The One Fill Per Page Rule replaces One Fill Per Group. The States
  table, the Lifted Name Rule (names are white or grey, never orange), the side panel, tabs and
  Deviations are rewritten to match.
- **Rows.** The Lookup result row is now 60 px, two lines in three groups. The Aligned Columns
  Rule is restated for groups. Rows visible: 5 whole rows at 1100 x 700, counted on
  `tone5-sell-1100.png`. At 2560 x 1440 the capture holds 8 rows plus free space; about 11 is
  computed from that space, not counted, and DESIGN.md says so. Revision 4's one-line rows
  fitted about 18 on a 1000 px window. astromech-3ca6 measures the built row. Only Sell's
  groups are written down; each other finder's groups are left to astromech-3ca6.
- **New named rules:** The Game's Symbols Rule (under Icons, squared with No Frontier assets),
  The Full-Width Header Rule and The Dimmed Page Rule (Layout). There is a new Route sidebar
  section, a rewritten Frame and route strip, the ten top tabs, and The Text Size Rule in place
  of The Height Scale Rule, with the six steps and first-run thresholds from astromech-b2cz.
- **Settings values:** a new Cell role, 15/500, Orange. Input, dropdown and the Settings row
  use it, and a Weight paragraph in Typography states the ruling. Orange at 15 px and over is
  never below 500; orange in Label size (a choice, "Ctrl+K", a stale age) keeps Label's 400.
  The front matter's weights match the Hierarchy table (checked by script).
- **Front matter:** sizes are in rem, the new colours are added, and component colours are
  updated. There are new `route-sidebar`, `route-rail` and `status-bar` components, and a new
  `cell` typography.
- **Token map:** each new token is marked "(new)" with the item that adds it. tokens.css is
  untouched, as the Scope says.
- **Captures:** 14 files are in `docs/design/nav-spike/`: the `tone5-*` and `hier-B-*`
  captures, `hier-A-over-page-1100`, `4-no-route-2560`, `5-ships-2560`,
  `7-materials-on-foot-2560`, and `header-stacked-1590`, which the Frame row cites. The eight
  `glow-up-*.png` files from `mockups/glow-up` are in `docs/design/`; `glow-up-icons.png` was
  already on main. A Reference captures section links all of them and says which predate the
  calmer colours. Every file on `spike/nav-rework` under `docs/design/nav-spike/` and every
  `glow-up-*.png` on `mockups/glow-up` that `.atelier/` cites now exists on this branch,
  checked by looping over the full file lists.

Calls Sumeet hasn't seen, made where no approved capture settles the point, each the narrowest
rule the principle gives:

1. **A chosen option** (Choice, Settings choice rows) is an ember cell with Orange Lit text at
   600 and a 2 px orange underline, not a fill.
2. **A dropdown's or Find or plot's highlighted row** is an ember cell, and the current option
   is Orange Lit.
3. **The listening binding cell stays filled orange** while it listens. It is the game's own
   listening cell and Controls changes colour only.
4. **Material tile:** the count is Options Grey (a grid is read, not ranked), the name Prose
   Grey, the track Prose Grey turning Orange Lit at the cap, and the selected tile Hull Raised.
   No tone5 capture shows Materials.
5. **Places home tiles:** the finder name stays Orange because it is a link; the live figure
   is Options Grey.
6. **The side panel's separator is a Seam rule**, as every tone5 capture draws it. The 1 px
   orange line now marks an overlay's open edge.
7. **Figures and Nav are 400.** Grey data at 400, as the spike drew it.
8. **Mark Off `#4f4a45`** is one token for the spike's two unlit colours: `#4f4a45` for
   hexagons and past stars, and `#3a3633` for the demand bars.
9. **Meters:** a Mark Off track filled Prose Grey, Orange Lit at the material cap or a complete
   shopping list. Revision 4 had a Seam track filled Orange Mid.
10. **Find or plot's highlighted row:** the kind word steps up to Prose Grey, because Label Grey
    is 4.4:1 on an ember cell.
11. **A page's search box keeps its orange outline** while it holds the search the page is
    answering, as `tone5-engineers-laser-2560.png` draws it.
12. **The Dimmed Page Rule** sets the scrim and click-outside behaviour for the route overlay
    only. Dialogs keep their own (`Dialog.svelte`, black at 70 %), which already dims, so no
    item has to change them.

Not done, and why:

- I did not find the spike's final report (the item says to start from it). I searched git and
  ran T3 thread searches for "nav-rework" and "DESIGN.md rules it changes"; they found only
  the planner's thread, [UX overhaul mission readiness](t3-thread://v2/f69ed1d3-873d-498a-9515-8ab96392223c/65a8ff7c-e901-48a1-be52-323bbbd8e80c),
  which I did not read in full. My sources were the spike's commit messages, its `tone.css`
  and Svelte mockups, the mission's revision 5 list in `design.md`, and the captures. I opened
  each capture that DESIGN.md cites for a claim.
- `scripts/check-guide.py` passes, but it reads only README, CONTRIBUTING and `docs/guide`, so
  its pass says nothing about DESIGN.md. I ran its link and anchor functions over DESIGN.md
  separately: 0 problems.
- The dock width, the width where the ten tabs stack, and rows visible are the spike's figures,
  measured under the old zoom and labelled that way. astromech-uc7i, astromech-5a01 and
  astromech-3ca6 measure them again.
- `4-no-route-2560.png` shows one rail action in the old colours, and `hier-A-over-page-1100.png`
  is Variant A's surfaces. DESIGN.md says so where it cites them, and the rule (two actions,
  Variant B) wins.

