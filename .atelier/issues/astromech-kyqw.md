---
created_at: "2026-11-05T06:19:46.182255093+00:00"
id: "astromech-kyqw"
issue_type: "validation"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-1d92"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-2hbw"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-f7i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-jp36"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-kwoi"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-nxz9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-rz0f"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T16:02:27.273090810+00:00"
status: "done"
title: "Validate the galaxy rework on the Linux build"
updated_at: "2026-11-05T16:02:27.273090810+00:00"
---

## Goal

Check the galaxy rework's outcome independently on the Linux build, against astromech-51ue's Done when.

## Done when

Each claim in astromech-51ue's Done when has an explicit result and a linked evidence receipt (`scripts/ui` screenshots and the tests run), and the handoff states failures, blocked checks and proof limits. Windows, game and second-monitor checks are out of scope here (astromech-cxgq).

## Context

DESIGN.md (Route strip, Lookup result row, Find or plot, Places home, States), decision astromech-bi94, the `#/mockup/ds-*` mockups.

## Close Reason

Validation report (validator-fable-kyqw, Fable 5.1, independent of the builders). All seven claims in astromech-51ue's Done when pass on the Linux build: route strip on every page but onboarding (astromech-9oml); Add stop and Set destination on every result, no silent replace, Resume offered (astromech-2xpe); Galaxy opens on Places with the tile grid, one page and URL per finder (10 finders), old URLs and palette entries open them (astromech-fol6); Powerplay/Exploration/Colonisation lookups keep the row and actions, Exploration's embed links to Services (astromech-a82a); every finder uses the DESIGN.md result row and States (astromech-o5jk); one Find or plot box with Ctrl+K plots, adds stops and opens finders from phrases (astromech-cls0); new finders have voice queries and the guide matches (astromech-27pq). Captured: app lint/check/build/test 464 tests, 0 skipped (astromech-lmf9); check-guide (astromech-03xb); cargo test for core, galaxy, market, tools 507 passed (astromech-o22a). Screenshots docs/design/validation/ (56). No failures. Proof limits: strip warning states, the no-position error, the build plan's embedded lookup and dimmed refresh rows are not producible in the preview and rest on unit tests; the core clipboard, parking and Spansh are stubbed by the fake core and rest on cargo tests. Out of scope: Windows, the real game, second monitor and DPI (astromech-cxgq). Minor observations: popover rows 35 px vs 34; blank strip while the first route read is pending; header two rows at 1600 px until fonts load; an impossible outfitting combination drops its mount (as on main).
