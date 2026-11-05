---
created_at: "2026-10-24T23:27:51.919381107+00:00"
id: "astromech-dzwy"
issue_type: "task"
labels:
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-dzwy
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T18:07:11.737826359+00:00"
status: "done"
title: "One face: move the window to variable Saira with tabular figures; retire the static cuts and Share Tech Mono"
updated_at: "2026-11-05T18:07:11.737826359+00:00"
---

## Goal

Use DESIGN.md's one face everywhere: Saira variable at 87.5 % width with tabular figures (DESIGN.md, Typography).

## Done when

- `--font` becomes `--face` at `font-stretch: var(--face-stretch)` after a side-by-side check that page metrics do not shift; the static Saira Semi Condensed and Share Tech Mono imports and dependencies are removed (and `@fontsource/michroma` if still unused).
- Every number in a column, tile, cell or strip uses tabular figures and is right-aligned; paths and logs use the same face.
- Screenshots of Settings, Trade, Ships and Diagnostics before and after under `docs/design/`.

## Context

Saira Semi Condensed (fontsource and the upstream static files) has no tabular figures. `@fontsource-variable/saira` (wdth 50-125) has `tnum`; astromech-c4i0 added it to `tokens.css` as `--face` / `--face-stretch`, used by the mockups only. Share Tech Mono (`--mono`) is used in 32 files, for names, dates, prices and prose as well as paths.

## Close Reason

Implementer report. The window uses DESIGN.md's one face: tokens.css sets --face (Saira Variable) at --face-stretch 87.5 % with tabular figures on html and body. --font, --mono, the static Saira Semi Condensed cuts, Share Tech Mono and the unused Michroma are removed with their packages (@fontsource/michroma, @fontsource/saira-semi-condensed, @fontsource/share-tech-mono) and lockfile entries; dist ships only three Saira wdth .woff2 files (~193 KB); Saira is SIL OFL-1.1 (NOTICE.md updated). All 43 --mono sites keep their sizes in the one face; .mono classes renamed for what they hold; columns of figures in trade route legs, Sell my cargo and the farm list right-aligned. Side-by-side before switching: 0.66 % narrower on average (max 1.1 %), identical line heights, no vertical moves on six pages. Header breakpoint 1569 → 1563 px with "Waiting for Elite"; DESIGN.md and the guide say about 1565 px. Proof: app lint, svelte-check 0/0, build, 465 tests; frozen lockfile consistent; greps clean; screenshots docs/design/face-*. Review: reviewer-sonnet-dzwy; fix_now gged (the Tabular Rule reworded: a column of quantities is right-aligned, tiles follow their spec, fact lists and dates stay left) and tn3c (Ships spec values at Value L line height 1.2) fixed by the orchestrator in 977bab1a; approved astromech-z8bf. Reviewer decisions: tabular figures page-wide; fact lists, tiles and date columns as above; Station and Diagnostics facts at 15 px like their neighbours. Open: a look in WebView2 on Windows (CI builds it).
