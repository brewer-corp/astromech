---
created_at: "2026-11-05T06:19:45.194670106+00:00"
id: "astromech-2hbw"
issue_type: "feature"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-nxz9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-rz0f"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Find or plot: one header box for systems, finders and Ctrl+K"
updated_at: "2026-11-05T06:19:45.194670106+00:00"
---

## Goal

Replace the header's systems-only plot box (`modules/route/PlotBar.svelte`) and the separate Ctrl+K palette with DESIGN.md's one Find or plot box: type a system to plot, add a stop or set the destination; type a phrase to open a finder; type a page, material, module or ship as the palette does today (decision astromech-bi94).

## Done when

- The header has one 320 px input, placeholder "Find or plot", "Ctrl+K" at its right end; Ctrl+K focuses it from any page (except while a bind capture has the keyboard), and `lib/ui/Palette.svelte` as a separate overlay is gone.
- Its popover lists, ranked together: systems (Spansh autocomplete) with Plot, Add stop and Set destination; finders, matched by name and by phrases with an input ("sell painite", "interstellar factors", "raw trader"), opening the finder filled in; and today's palette entries (`lib/palette.ts`).
- Plot keeps today's behaviour (the panel with `Plot.svelte` and the external links).
- Tests: the matcher (`palette.test.ts` or a new `find.test.ts`: a system, a phrase with an input, a page, a module), keyboard use, and the three system actions calling the core; `pnpm lint && pnpm check && pnpm build && pnpm test` pass.
- `scripts/ui` screenshots of the box at rest, with a system typed, and with a finder phrase typed.
- docs/guide (routes and the index's Ctrl+K text) updated.

## Scope

`App.svelte` header, `modules/route/PlotBar.svelte`, `lib/ui/Palette.svelte`, `lib/palette.ts`, `lib/find.ts`. Voice is out of scope: it already reaches lookups through its own grammar.

## Context

- DESIGN.md: Find or plot, Frame (header 64 px), Input.
- Needs the finders' URLs (astromech-nxz9, astromech-kwoi) and the Add stop and Set destination actions (astromech-rz0f).
