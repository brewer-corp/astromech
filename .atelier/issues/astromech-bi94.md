---
created_at: "2026-10-24T23:07:07.166119821+00:00"
id: "astromech-bi94"
issue_type: "decision"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-24T23:07:16.864284738+00:00"
status: "done"
title: "Galaxy rework: direction for routes and place-finding"
updated_at: "2026-10-24T23:07:16.864284738+00:00"
---

## Context

A UX audit on 24 Oct 2026 (main @ 54e1b7f4) found place-finding spread over about 13 lookups on Galaxy, Trade, Materials, Powerplay, Exploration, Colonisation and the ship planner, each with its own form and a Go button that silently replaces the followed route. Real plotting lives only in the header box, which takes systems only and isn't connected to results. The route rail shows on Galaxy pages only. Research covered ED Outrider, OD Explorer, EliteIntel, Observatory, Elite Nexus, SrvSurvey, Spansh, Inara, Ardent, EVE and Star Citizen tools, Awakened PoE Trade, Google Maps and Little Navmap.

The wider goal is to end the fragmentation of community tools: questions like "an HGE system near a material trader" or "a Platinum hotspot near a good buyer" today mean combining several tools in your head.

## Question

How should routes and "find a place to do X" work across the window?

## Options

- A family of finders sharing one engine and one result layout (Places), with a persistent route.
- A single omnibox as the only way in (rejected: hides the finders).
- A query engine for linked places (the "mega-form"), reached in steps: chaining ("find near this"), saved recipes, then goals worked out from the journal.

## Decision

Sumeet, 24 Oct 2026:

- Result actions are both Replace route and Add stop, never a silent replace. Add a "Find along route" lookup ranked by detour.
- Places is a family of finders with their own titles, main inputs, main value and URLs, on one engine. The Places home page is a visible grid of finders and recipes; the search box and voice are shortcuts into it, not the only way in.
- Mining hotspots and sales become finders in Places. Colonisation stays as it is for now.
- No pinned questions that re-run on every jump in v1, so there's no automatic re-search budget to set.
- The route becomes a persistent object, shown as a strip on every page.
- The engine is built as linked places from the start, so recipes and goals are additions, not rewrites. Spike the data side first.
- Formalise the design system (DESIGN.md) before any of this UI work. The window follows Elite's sparse, scannable UI ethos: one main value per element, state shown by brightness, orange only where it means something. Hierarchy, not density, is the goal.

## Close Reason

Places on one linked-place engine, persistent route, Replace + Add stop, design system first (Sumeet, 24 Oct)
