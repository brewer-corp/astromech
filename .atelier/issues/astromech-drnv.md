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
updated_at: "2026-11-10T06:04:05.615896996+00:00"
---

## Goal

A finder shows results first. Today every Places finder, Colonisation and Powerplay's 'Where to work' put up to five rows of filter controls above the first result. The one-line summary that already sits above the results ('Within 100 ly of Sol · medium or large pad · reports up to 30 days old') becomes the filters' resting state.

## Done when

- [ ] At rest a finder shows the summary line with a CHANGE action and no filter controls. The thing being searched for (commodity, module, service) stays an always-visible input.
- [ ] CHANGE opens the controls in place. Edits apply live as they do today (SearchKit searches again when a saved filter changes) and the panel stays open across them, so two filters can be changed without reopening it. It closes on DONE, on Escape, or on CHANGE again; never on its own.
- [ ] SearchFilters.svelte owns the open or closed state and renders the summary from a summary prop its parent passes (SearchKit builds it from the answer, as it does now). A filter that differs from its default is always named in the summary. A failed search or a failed save leaves the panel as it was, with the error where it shows today.
- [ ] Both kinds of consumer work: the ones that search on every change (SearchKit's finders) and the one with an explicit button (app/src/modules/trade/Route.svelte uses SearchFilters directly with FIND ROUTE), which may be wired at page level; there the panel stays open until its search is run.
- [ ] A browser test covers: open, change two filters, results update, DONE; a failing search with the panel open; and the explicit-search consumer.
- [ ] Before-and-after captures of Places > Sell at 1920 x 1080 are in docs/design/; pnpm check, pnpm lint and pnpm test pass.

## Context

Strategy: docs/strategies/glow-up.md (mission astromech-xajc). Mockups: docs/design/glow-up-*.png. Mockup: docs/design/glow-up-sell.png. DESIGN.md's Lookup result row already specifies the summary line.
