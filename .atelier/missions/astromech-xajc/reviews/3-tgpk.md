---
schema: atelier.review
schema_version: 3
round: 3
commit: 837a6e3a494b7366e87c7d68f106ff96ac3f79ac
reviewers:
- 01a1e807-8187-75d3-be0c-5e9880d38d65
model: grok-4.6
verdict: approve
created_at: 2026-11-17T05:43:19.003489868Z
---

## Summary

- **Complexity:** high. Increment 2 still rewrites the frame, the colour contract, text size and the tab tree, and adds engineer-search logic.
- **Value:** high. The pages are in daily use, increment 1 is on main, and the five changes match Sumeet's recorded answer at the stop.
- **Risk of merging it:** medium. Visual failures show in the preview and revert per page. Grade advice and demand colours can be wrong without looking wrong; those two name tests. Text size has a migration contract. The three holes from round 2 are closed.

The value is worth the rest. Round 2's three fix-now findings are addressed in the items they named. Increment 2 can start.

I read `mission.md`, `design.md`, `notes.md`, every mission item (ready, draft, done and dropped), DESIGN.md Typography and Frame, `app/src/lib/modules.ts`, `App.svelte`'s `MOVED` map and default page, `eslint.config.js` `PAGE_SIZING` lists, `SearchKit.svelte` and its consumers (`Targets.svelte`, `colonisation/View.svelte`, `Colonies.svelte`, `Nearby.svelte`), `search.ts` `factColumns`, `ship_status`, `cargo.rs` `sellable`, `engineers::offers`, `config.rs` `TextSize`, the strip and Find or plot `#/route` links, and `worlds/preview/engineering` (commander at Shinrarta Dezhra). I checked `spike/nav-rework` at `dea5f0bb` still holds the `tone5-*` / `hier-B-*` captures, and `mockups/glow-up` still holds `docs/design/glow-up-*.png`. I did not run the app, the spike mockups, or Windows WebView2.

## Finding 1: Keep report details reachable in shared lookups without a side panel
Recommendation: fix now
Was: 2-bu7e.10
Now: fixed

`astromech-3ca6` now moves Updated to the side panel only where the list has one, and keeps the report's age on the row for panel-less SearchKit consumers, naming Powerplay's `Targets.svelte` and Colonisation's `View.svelte`. Age stays in the trip group at any age (grey when fresh, Orange past 30 days). Acceptance adds captures of those two lists and a browser test of a 12-day-old report on a panel-less list, so the over-30-days mark is not the only way a younger report's age stays reachable.

## Finding 2: `#/route` cannot land on the current page through the MOVED map
Recommendation: fix now
Was: 2-ylm3.10
Now: fixed

`astromech-uc7i` now splits the two cases: in-app controls that used to open the Route page open the sidebar and change no page; a cold `#/route` lands on the default page with the sidebar open (or the overlay below the dock width). It says `MOVED` only rewrites paths, so the item adds what opens the sidebar.

## Finding 3: DESIGN.md's empty rail is still the spike's, without Import
Recommendation: fix now
Was: 2-ylm3.11
Now: fixed

`astromech-n41x`'s Frame bullet now puts Find or plot and Import a route on the 48 px empty rail, and the same two on the narrow strip's No route state, matching `astromech-uc7i`.
