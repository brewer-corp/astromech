---
schema: atelier.item
schema_version: 3
id: astromech-3kf5
title: 'Calmer pages: data in white and grey, orange for the ranked value and what you can act on'
status: building
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-b2cz
implementers:
- thread:delegated-task:command%3Amcp%3Af257f609-54d7-4e2e-bfce-45bbbe671617%3Adelegate-task%3A2e53a792-ff91-43d6-9a1d-43aa21aef462
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.238277178Z
---

## Goal

The eye has somewhere to go. Today almost every piece of text is saturated orange at weight 500; Sumeet, 16 Nov 2026: "if everything draws the eye, then nothing can". Data becomes white and grey, and orange is spent on the value a list is ranked by, the main value, and what the commander can act on.

## Done when

- [ ] `tokens.css` has DESIGN.md revision 5's greys and duller red; the type role classes and the shared components in `app/src/lib/ui/` (rows, tiles, Stat, Badge, Button, Choice, tabs, StatusBar, the layouts' side panel) take their colours from the roles, so a page gets the new palette without naming a colour.
- [ ] Selection everywhere is the raised row with lit text; no row, tile or option is a solid orange fill. The filled orange button is unchanged.
- [ ] Tiles and page blocks are off Hull (outline or Hull Raised); Hull is the frame's.
- [ ] Every page built so far is restyled and checked in the preview at 1100 x 700 and 2560 x 1440: Route, Materials, the Places finders, Trade > Commodities, Settings, Powerplay, Exploration, Biology, Session, Mining, Station, On foot, Ships, Stored modules, Engineers and the Controls pages. On each, the main value, what can be changed and what needs the commander are the brightest things.
- [ ] Before-and-after captures of Materials, the Sell finder, Powerplay and My controller at 2560 x 1440 in `docs/design/`; the design-system screenshots re-rendered.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass; no test asserts a colour.

## Scope

Colour, weight and selection. The two-line grouped result row is astromech-3ca6; the sidebar is astromech-uc7i. Controls pages change colour only.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). `tone5-sell-2560.png` and `tone5-engineers-laser-2560.png` are the reference; `app/src/mockups/nav/tone.css` on the spike holds the mockup's values as a starting point, written fresh here against the real tokens.
