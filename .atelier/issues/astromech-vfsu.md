---
created_at: "2026-11-07T13:49:47.872685749+00:00"
id: "astromech-vfsu"
issue_type: "bug"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T17:03:58.280859125+00:00"
status: "superseded"
title: "Main's Linux CI fails in the frontend test src/lib/palette.test.ts"
updated_at: "2026-11-07T17:03:58.280859125+00:00"
---

## Goal

The Linux CI job fails on main: `src/lib/palette.test.ts` has 1 failing test of 528 (run 42352408463 on 5aac2e85, and the same assertion on PR #217). A red main hides new failures, so find the cause and make the test pass again or fix the code it catches.

## Done when

- [ ] The cause of the `palette.test.ts` failure is named in the Report.
- [ ] The Linux CI job passes on main.

## Report

Already fixed on main by astromech-bp23's b6733015 (merged in 04977f56): a static import cycle lib/transport.ts -> preview/transport -> lib modules -> lib/transport let Vite's module runner hand a concurrent loader a half-initialised lib/transport, so Materials' palette call threw a TDZ ReferenceError that collect() swallowed. A regression test (src/lib/transport.test.ts) fails on the old code. Main's Linux CI then failed once more at 04977f56 on a different flake (site_lookup.rs:256), fixed by astromech-7149 (3025130d, merged 5af00ae4); CI on main passed at 5af00ae4 and e6ae38df.
