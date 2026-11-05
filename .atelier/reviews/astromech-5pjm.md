---
schema: atelier.review
schema_version: 1
id: astromech-5pjm
title: finding on astromech-nxz9 @326fd18e
target: astromech-nxz9
identity: reviewer-fable-nxz9
commit: 326fd18eebaf7994530bf4843f8fa9d107921807
created_at: 2026-11-05T08:46:20.206008456Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/places.ts:176 and app/src/modules/places/palette.ts:9 The Finder declaration (id, title, section, what, input) serves the home and the page map (PAGES is typed on FinderId, so a finder without a page fails the typecheck), but it carries no input vocabulary or route-with-input builder, so palette.ts hand-lists FINDERS + SERVICES + MODULES + SHIPS and leaves TRADERS out: Ctrl+K 'raw trader' opens places/services/raw_trader (Services), not the Material traders finder. astromech-2hbw's Done when says its matcher reads the Places finder list so later finders (kwoi, f7i0) match without changing the box; with today's declaration each new finder's phrases need palette.ts and the matcher edited by hand. Fix in 2hbw or kwoi: give Finder an optional per-finder entry list (label, detail, route) or items + route(input), have palette.ts and the Find or plot matcher read it, and add the TRADERS kinds.
