---
created_at: "2026-10-24T23:07:07.397323815+00:00"
id: "astromech-c4i0"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
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
closed_at: "2026-11-05T01:17:54.991555368+00:00"
status: "done"
title: "Write DESIGN.md: the window's design system"
updated_at: "2026-11-05T01:17:54.991555368+00:00"
---

## Goal

Write the design system the "Elite Controls dialect" never had, so pages are reviewed against rules rather than "does it look like Elite". It must carry Elite's sparse, scannable ethos (the in-game material trader screen is the reference, though it is too sparse for us) and fix what the audit found: the brown cell on read-only things, caps and tracking at every level, orange as decoration with the tab hierarchy upside down, the monospace font in 29 files, up to four tab rows, five input and five button styles, no primary action, and tall result rows whose key value is the smallest text.

## Done when

- `DESIGN.md` at the repo root (impeccable format) defines: principles; colour roles (when orange is allowed); type scale, case and when the monospace font is allowed; spacing and content widths; surfaces vs cells (a cell means editable); one style each for input, dropdown, button (primary and secondary), tab and badge; states (selected, disabled, stale, warning, error, empty); result rows for lookups; the route strip; and what it keeps from PRODUCT.md (stock orange, no Frontier assets, English).
- `app/src/design/tokens.css` matches it, or the gaps are listed as follow-up issues.
- Static before/after mockups for Materials and a Places finder result list (Sell, plus Mining to show two finders reading differently on one layout), with screenshots under `docs/design/`.
- PRODUCT.md and V1-DESIGN §4 point at DESIGN.md instead of contradicting it.
- The direction is approved (decision astromech-5p9c: independent panel, per Sumeet's 4 Nov 2026 direction).

## Context

Decision astromech-bi94. The audit's evidence is the screenshot set taken 24 Oct 2026; the pages left out of the facelift (System, Station, Settings, onboarding) are denser, but density is not the goal.

## Close Reason

Implementer report. Changed: DESIGN.md (revision 3, impeccable format) at the repo root, with .impeccable/design.json. app/src/design/tokens.css gains the design tokens plus a role-to-token map, and --row-hover is now warm #21170f (it was grey). Static mockups are in app/src/mockups/ds/, with before/after screenshots in docs/design/design-system-*.png. scripts/mockup-shots.sh renders the ds-* mockups. PRODUCT.md, V1-DESIGN section 4, AGENTS.md and CONTRIBUTING.md point at DESIGN.md. Follow-ups yxtz, dfvp, pey5, sllo and dzwy carry the page work and the deferred panel items. Decision: direction approved with 13 changes by an independent panel (Fable 5.1 and Opus 5.5) per Sumeet's 4 Nov 2026 direction; see astromech-5p9c. Red is for hard errors only. Proof: pnpm lint, check, build and test (397/397); check-guide; atelier doctor; evidence after merging main; PR #185 CI green, including WebKitGTK and WebView2. Review: approval astromech-82p7; xmdr fixed; qy3c deferred to yxtz; f6w9 no_action. Deviations: Sumeet's approval was replaced by the panel decision, at his direction. Open items: how the full-orange text reads on a real second monitor beside the game, and at Windows DPI, is unverified (needs the PC). Live pages beyond the --row-hover tint are unchanged until the follow-ups.
