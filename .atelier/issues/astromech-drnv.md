---
created_at: "2026-11-10T05:55:01.251561555+00:00"
id: "astromech-drnv"
issue_type: "feature"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-b2k3"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-xajc"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Finder filters collapse to their summary line"
updated_at: "2026-11-10T05:55:01.251561555+00:00"
---

## Goal

A finder shows results first. Today every Places finder, Colonisation and Powerplay's 'Where to work' put up to five rows of filter controls above the first result. The one-line summary that already sits above the results ('Within 100 ly of Sol · medium or large pad · reports up to 30 days old') becomes the filters' resting state.

## Done when

- [ ] app/src/lib/ui/SearchFilters.svelte (and SearchKit where it lays them out) shows the summary line with a CHANGE action; CHANGE opens the controls in place and they close again on a new search. The thing being searched for (commodity, module, service) stays an always-visible input.
- [ ] A filter that differs from its default is named in the summary, so nothing is hidden silently.
- [ ] Every finder that uses SearchFilters gets it with no page-level change; a browser test covers open, change, search, closed.
- [ ] Before-and-after captures of Places > Sell at 1920 x 1080 are in docs/design/; pnpm check, pnpm lint and pnpm test pass.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Mockup: docs/design/glow-up-sell.png. DESIGN.md's Lookup result row already specifies the summary line.
