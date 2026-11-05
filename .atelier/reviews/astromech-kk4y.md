---
schema: atelier.review
schema_version: 1
id: astromech-kk4y
title: approval on astromech-1d92 @5afebb00
target: astromech-1d92
identity: reviewer-fable-1d92
commit: 5afebb00886ec2d015edb250a6fab6d5e189e093
created_at: 2026-11-05T07:56:30.414471400Z
event:
  event: approval
---

Reviewed b936a744..5afebb00 against 1d92's Done when, decisions bi94, g8d5 (item 10) and ghyo (item 9), DESIGN.md (Frame, Route strip, Badge, States, Icons, Narrow Red Rule, Token map), PLOTTING.md. Read Strip.svelte, Strip.test.ts, App.svelte/test, route.ts, fakecore.ts, PlotBar's focus hook, the three Galaxy views, Route page View.svelte, route.rs (Waypoint.jumps serde default, Route::new merge, jumps_left Option sum, add_next clearing the following leg, Desk::plotted jumps 1), galaxy plot.rs/neutron_plot.rs (exact: 1 per stop, neutron: the leg's jumps) and the Tauri route_plot path (serialises Plotted into route_set, so jumps reach the core). Reran: cargo test -p astromech-core -p astromech-galaxy 418 pass incl. the old-format load and jumps_left tests; pnpm test 408 pass; pnpm lint and svelte-check clean; check-guide and check-deps OK. Looked at the nine screenshots: one line at 1100 px incl. the long-name and clipboard-warning worst case; Services/Outfitting/Shipyard keep their spacing. Red only on a failed call or first read. Not run: pnpm build, the Tauri shell build. Five low findings, all defer (astromech-w52v, 9ju1, 7464, aeub, 9e5p); no fix_now.
