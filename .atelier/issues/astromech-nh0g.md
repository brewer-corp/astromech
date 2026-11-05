---
created_at: "2026-11-05T07:57:02.360015345+00:00"
id: "astromech-nh0g"
issue_type: "task"
labels:
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-1d92"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Preview fixtures: per-stop jumps on the neutron and local routes, and lookups near the route scenario's commander"
updated_at: "2026-11-05T07:57:02.360015345+00:00"
---

## Goal

The browser preview (`?core=...`) shows the route strip and lookups as the real app would, so screenshots taken from it are honest layout evidence.

## Done when

- `app/src/lib/fixtures/plot.json`'s `neutron` and `local` routes carry per-stop `jumps` matching their totals (finding astromech-9ju1), so the preview's Plot and follow shows "N jumps left" as the real plotter does.
- With `?core=ready&route=exact` the lookups' origin follows the scenario's commander system rather than Sol (finding astromech-7464).
- `pnpm test` passes; a `scripts/ui` shot of Services with `?route=exact` shows the right origin.

## Context

Deferred findings from the 1d92 review (reviewer-fable-1d92). Preview only; the real app is unaffected.
