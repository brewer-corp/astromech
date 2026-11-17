---
schema: atelier.item
schema_version: 3
id: astromech-n41x
title: 'DESIGN.md revision 5: orange means act, the route in the frame, grouped rows'
status: ready
mission: astromech-xajc
labels:
- size-m
- docs
priority: high
blocked_by: []
implementers: []
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

- [ ] Colour: a principle and a list. The principle: read-only data is white or grey in three steps; Orange is for the value a list is ranked by, the one main value, and what the commander can act on or change; colour beyond that is allowed where it carries a stated meaning or is one of Elite's own conventions. The list names every coloured thing and its meaning, starting from the spike's: demand against the hold (short, tight, plenty, with the bars), a pad too small, a stale report, star icons by what you do there, security, engineer state, hexagons for rank and grade. A colour not on the list needs adding to it.
- [ ] The greys and the duller red step ("won't fit, won't sell") are tokens with their contrast on the window background and on Hull stated; readable text is 4.5:1 or better; the Narrow Red Rule says how the duller red differs from alarm red.
- [ ] Selection is a raised row with lit text, not a solid orange fill; the filled orange button stays the one fill a page has. The One Fill Per Group rule, the States table and the Lifted Name Rule are rewritten to match (names are white or grey; the ranked value is orange).
- [ ] The Lookup result row is two lines in groups (the value and what qualifies it; the place and what you need to dock; the trip), with the rows-visible figures restated. The Aligned Columns Rule is restated for groups.
- [ ] A named rule: where Elite has a symbol players know (the engineer hexagon, star and station icons, the market's demand bars), use it, not a generic shape.
- [ ] Frame: the route sidebar (Hull, flush left, headed ROUTE, joined to a Hull status bar; collapsed it is the strip; no route is a 48 px rail; below the dock width it opens over the page with the page dimmed), the full-width header rule, and the dimmed-page rule for overlays. Page tiles and blocks leave Hull for an outline or Hull Raised.
- [ ] Navigation: the top tabs are NOW, SHIPS, ENGINEERING, PLACES, TRADE, EXPLORATION, POWERPLAY, LOGBOOK, CONTROLS, SETTINGS; SHIPS holds Ships, Stored modules, Suits; ENGINEERING holds Materials and Engineers.
- [ ] Text size: the Height Scale Rule is replaced. Sizes are in rem; Text size sets the root size; a bigger window shows more and never changes text size. Container breakpoints are in rem.
- [ ] Settings values: one ruling replaces the contradiction astromech-z5oh reported (the component table says 400, Typography says 400 looked thin): an input's value is Orange at 500, because it is what the commander can change.
- [ ] The approved captures are on main under `docs/design/nav-spike/` (the `tone5-*` and `hier-B-*` files and the 1100 x 700, no-route, Ships and Materials ones the rules cite), and DESIGN.md links them. The earlier screen mockups' captures (`docs/design/glow-up-*.png`, on the branch `mockups/glow-up` only) come to main too, since the screen items cite them. `scripts/check-guide.py` passes.

## Scope

DESIGN.md and the reference captures. No tokens.css or component change; the items below own those.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). The spike's final report lists the DESIGN.md rules it changes; start from that list.
