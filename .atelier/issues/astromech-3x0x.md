---
created_at: "2026-10-22T19:30:25.054825868+00:00"
id: "astromech-3x0x"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-6bjl"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-d4of"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Route page plots neutron routes locally, with Spansh as the fallback"
updated_at: "2026-10-22T23:33:16.588145634+00:00"
---

## Goal

**Plot and follow** with Neutron selected uses the local plotter when the neutron star file is there, and Spansh when it is not, and the page says which one plotted the route.

## Done when

- `route_plot` in the shell picks the local plotter when `neutron.bin` is present and readable, else Spansh's `/api/route`; a broken file falls back to Spansh and the page says why.
- The Efficiency field shows only for the Spansh fallback; local routes have no setting.
- The result line says where it was plotted ("on this PC" or "by Spansh") with the existing jumps, distance and boost counts.
- Component test for both paths (`View.test.ts` or a new one) and a shell-level test or recorded run for the fallback.
- docs/guide/routes.md and docs/PLOTTING.md describe both paths and the local plotter's limits (no fuel planning, sparse gaps estimated, no via systems).

## Scope

`app/src-tauri/src/main.rs` (`route_plot`, `route_plot_ship`), `app/src/modules/route/Plot.svelte`, `app/src/lib/route.ts`, the two docs. Exact routes stay on Spansh.

## Context

The plotter and the download are sibling children. UI checks follow CONTRIBUTING.md (the UI check with `scripts/ui`).

## Notes

- 2026-10-22 [note] 2026-10-22, from review finding astromech-f5zt on 6bjl: the shell must load the new neutron file after a download finishes (or when names() reports the file changed), with a test; plot() should keep Route::fewest so the page can say a long route is near the fewest jumps; list that among the limits in routes.md and PLOTTING.md.
