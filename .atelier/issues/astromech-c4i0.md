---
created_at: "2026-10-24T23:07:07.397323815+00:00"
id: "astromech-c4i0"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: worktree-agent-adb7e7a56c235ac60
  work_branch: task/astromech-c4i0
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Write DESIGN.md: the window's design system"
updated_at: "2026-10-24T23:08:01.974490751+00:00"
---

## Goal

Write the design system the "Elite Controls dialect" never had, so pages are reviewed against rules rather than "does it look like Elite". It must carry Elite's sparse, scannable ethos (the in-game material trader screen is the reference, though it is too sparse for us) and fix what the audit found: the brown cell on read-only things, caps and tracking at every level, orange as decoration with the tab hierarchy upside down, the monospace font in 29 files, up to four tab rows, five input and five button styles, no primary action, and tall result rows whose key value is the smallest text.

## Done when

- `DESIGN.md` at the repo root (impeccable format) defines: principles; colour roles (when orange is allowed); type scale, case and when the monospace font is allowed; spacing and content widths; surfaces vs cells (a cell means editable); one style each for input, dropdown, button (primary and secondary), tab and badge; states (selected, disabled, stale, warning, error, empty); result rows for lookups; the route strip; and what it keeps from PRODUCT.md (stock orange, no Frontier assets, English).
- `app/src/design/tokens.css` matches it, or the gaps are listed as follow-up issues.
- Static before/after mockups for Materials and a Places finder result list (Sell, plus Mining to show two finders reading differently on one layout), with screenshots under `docs/design/`.
- PRODUCT.md and V1-DESIGN §4 point at DESIGN.md instead of contradicting it.
- Sumeet approves the direction before merge.

## Context

Decision astromech-bi94. The audit's evidence is the screenshot set taken 24 Oct 2026; the pages left out of the facelift (System, Station, Settings, onboarding) are denser, but density is not the goal.
