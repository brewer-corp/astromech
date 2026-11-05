---
created_at: "2026-11-05T06:19:44.872165553+00:00"
id: "astromech-nxz9"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-jp36"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Places home and the Galaxy finders as Places pages"
updated_at: "2026-11-05T08:27:06.333434639+00:00"
---

## Goal

Turn the Galaxy area into Places: a home page that shows every finder as a tile under section headings, and one page per finder with its own title, main input and URL (decision astromech-bi94, DESIGN.md Places home). This child builds the home and moves the finders Galaxy has today; the Trade and Engineering finders follow in astromech-kwoi.

## Done when

- Galaxy's page tabs are Places and Route. The mockups also show a System tab; it is not built here, because System is already a page in Exploration and moving it is outside this rework (decision astromech-g8d5, item 4), so screenshot comparisons ignore that tab. `#/places` shows the Places home: finder tiles (220 px Hull links, name, one Label line, no Lit text without an answer) under Trade, Ship, Engineering, Mining and Route headings. In this child the Ship section links Services, Outfitting and Shipyard, and Engineering links Material traders (the Services lookup on the raw, manufactured and encoded trader services). Tiles for finders not built yet are left out, not shown disabled.
- Each finder is its own page at `#/places/<finder>[/<input>]` with a "Places" crumb, its title, its main input (service, module, ship, trader kind) and the result rows from astromech-jp36. No third tab row.
- Old links keep working: `#/services`, `#/outfitting/<module>[?…]`, `#/shipyard/<ship>` and the Ctrl+K palette's module and ship entries open the matching finder (`MOVED` in `App.svelte` or the manifests). `lib/ui/FindWant.svelte` is gone; the finder's main input replaces it.
- A tile shows a figure only from an answer already in hand (the finder's last result this session), never a new request on opening the home page.
- Tests: routing of old and new URLs, the tile list, palette routes (`palette.test.ts`); `pnpm lint && pnpm check && pnpm build && pnpm test` pass.
- `scripts/ui` screenshots of `#/places` and the Services, Outfitting and Material traders finders, compared with `docs/design/design-system-places-home-after.png`.
- docs/guide: the Galaxy pages become a Places page (index and Help links updated; `python3 scripts/check-guide.py` passes).

## Scope

`app/src/modules/{services,outfitting,shipyard}` (merged or wrapped into a places module), `lib/modules.ts`, `App.svelte` routing, the palette routes. Nothing in the core.

## Context

- DESIGN.md: Places home, Tabs, The Two Tab Rows Rule; mockup `app/src/mockups/ds/PlacesHome.svelte` (`#/mockup/ds-places`).
- docs/ARCHITECTURE.md §18.2 (module contract, palette routes).
