---
created_at: "2026-10-24T23:07:07.848384897+00:00"
id: "astromech-51ue"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-1d92"
  - kind: "issue"
    id: "astromech-2hbw"
  - kind: "issue"
    id: "astromech-f7i0"
  - kind: "issue"
    id: "astromech-ibmy"
  - kind: "issue"
    id: "astromech-jp36"
  - kind: "issue"
    id: "astromech-kwoi"
  - kind: "issue"
    id: "astromech-kyqw"
  - kind: "issue"
    id: "astromech-nxz9"
  - kind: "issue"
    id: "astromech-rz0f"
  - kind: "issue"
    id: "astromech-sgwi"
  - kind: "issue"
    id: "astromech-wla4"
  - kind: "issue"
    id: "astromech-xlbz"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-yn6y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Galaxy rework: route strip, Places finders, one search box"
updated_at: "2026-11-05T06:20:05.328988989+00:00"
---

## Goal

Replace the scattered place-finding pages and the header plot box with the model decided in the galaxy direction decision: a persistent route shown on every page, Places as a family of finders on one linked-place engine (Replace route / Add stop on every result, Find along route, chaining "find near this", recipes), and one "Find or plot" box shared with Ctrl+K and voice.

## Done when

On the Linux build, shown by the children's tests and `scripts/ui` screenshots, an independent review of the branch, and the validation child's receipts:

- Every page except onboarding shows the route strip; the Galaxy-only rail is gone.
- Every lookup result offers Add stop and Set destination, and no action replaces the followed route without saying so and offering Resume.
- Galaxy opens on Places: a home page of finder tiles, one page and URL per finder, covering today's Services, Outfitting, Shipyard, material traders, Sell, Buy, Sell my cargo, raw sites and faction states, plus Mining hotspots; old URLs and palette entries still open them. Colonisation stays where it is.
- Every finder shows the DESIGN.md Lookup result row and States.
- The header has one Find or plot box, also reached with Ctrl+K, that plots or adds a system and opens a finder from a phrase.
- The linked-place engine answers the spike's two recipes from Places, with "Find near this" on result rows, and Find along route ranks places by detour.
- Each new finder has a voice query, and the guide describes what shipped (`scripts/check-guide.py` passes).

The play check on Windows next to the game is astromech-cxgq (needs-human), not part of this Done when.

## Unknowns

- Corridor queries for Find along route have not been measured (astromech-wla4).
- Whether the local station, ring and hotspot file is needed depends on live request volume and on the Spansh maintainer's answer (astromech-5rm2, astromech-ibmy).

## Background

Decision astromech-bi94 sets the direction; DESIGN.md (Route strip, Lookup result row, Find or plot, Places home, States) and the `#/mockup/ds-*` mockups set the look; spike astromech-yn6y's close reason sets the engine's shape, budget and which legs are live. Shared controls and icons come from astromech-pey5. Children share this issue's branch.
