---
schema: atelier.review
schema_version: 3
round: 3
commit: d5f614fae345ce2d4d7cf23030392707fc02da60
reviewers:
- thread:delegated-task:command%3Amcp%3A4b7b4848-359d-4f1d-86e9-c6fca2d53526%3Adelegate-task%3Ad3923c95-6c06-4d1d-864c-45a020670f04
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T13:12:13.930422193Z
---

## Summary

Complexity: medium. The result layout is bounded, but the shared pad badge adds an asynchronous dependency on the search's effective ship requirement.

Value: high. Ordered stops and legs, led by the hourly estimate, directly deliver the item's goal.

Risk of merging: medium. Incorrect pad marking misleads the commander, though the core's warning remains visible and the change is easy to revert.

The value warrants the size. I found no materially smaller implementation that delivers the requested result. The pending-save fix works, but the replacement pad read has two correctness problems that need fixing before merge.

I reviewed the full diff from main to `d5f614fae345ce2d4d7cf23030392707fc02da60` and the changes since round 2 against each Done when line and the Report. I read the core's route construction and filter resolution, the ship-status handler, filter controls, Workbench, type roles, Tip and pad components. The totals, ordered stops and cargo, repeated loop endpoint, empty return, state rendering, token enforcement and guide otherwise match the item.

I independently ran `pnpm check`, `pnpm lint`, the 11 Route browser cases, `pnpm test` (97 files, 900 tests), `python3 scripts/check-guide.py`, and `ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core node --test 'journeys/*.journey.ts'` (7 passed). All passed. There is no core diff against main. The suite covers both pending-save regressions and the ship swap before a search.

I inspected the committed narrow loop, large-text single hop and 1920-wide single-hop captures, and the corresponding narrow-loop and large-text single-hop before captures. I checked the plans and all 20 audit entries: the specified sizes, text steps and sidebar states are present, and all six fault lists are empty. The source changes since the audit's `1fb62535` affect pad resolution, not markup or styling. I did not recreate the before captures, rerun the complete visual audit, or check live Spansh or Windows WebView2.

I independently mounted the real Route component in headless Chromium with only its transport replaced and reproduced both findings below without repository source edits or sleeps. I also checked the offline pad discrepancy against the real core on the committed `worlds/preview/market` world.

## Finding 1: Resolve the ship's pad when an override is being cleared
Recommendation: fix now
Was: 2-m3ls.2
Response: fixed in 4dbfe2f1
Now: fixed

`measuredPad()` now resolves an automatic search independently of the stored override. The new regression holds the save while clearing an S override and verifies the too-small badge against the live M ship. The explicit-override and pre-search ship-swap regressions also pass. That fixes the previous reproduction; the new source and timing problems are below.

## Finding 2: Resolve the automatic pad from the journal-backed ship
Recommendation: fix now

`app/src/modules/trade/Route.svelte:96` treats `ship_status.pad` as the pad the route search used, but the commands have different sources. `ship_status` reads only the running session and returns default, null figures with Elite closed (`crates/astromech-core/src/app.rs:1214-1219`). The search resolves its filters from the journal-backed engineering fleet, which remains available with Elite closed (`crates/astromech-api/src/lookups.rs:207-213`). This is an ordinary supported route-planning state, not an unknown ship.

I started the real core on `worlds/preview/market` without starting Elite. `search_filters` for `trade.route` returned `{ pad: "M", pad_set: false }`, while `ship_status.pad` was null. Route defaults still supplied the ship's capacity, range and station. In the real component, with that automatic M filter and null status pad, returning the recorded single hop with its destination changed to S and the too-small-for-M warning produces the warning but a badge named `Small pad`, with no `unfit` class. The required red, struck-through pad and its explanatory accessible name are lost.

Use the same journal-backed ship requirement the search resolves, including when Elite is closed, while preserving the pending-save override fixes. Add the offline boundary regression; the current always-live `ship_status` answer cannot catch this discrepancy.

## Finding 3: Keep the pad requirement from the start of the search
Recommendation: fix now

`app/src/modules/trade/Route.svelte:112-113` waits for the entire Spansh job before reading the automatic ship requirement. The core resolves the filters before starting that job (`crates/astromech-api/src/lookups.rs:419-423`), which the page says can take a minute. A ship swap or Elite exit during the job therefore changes the badge's requirement without changing the route or its warning. Freezing `need` afterward preserves the wrong value.

I held the transport's `trade_route` answer after a search began for an M ship, changed the status ship to S while the loading line was visible, then released the recorded single hop with its destination pad changed to S and the too-small-for-M warning. The subsequent `ship_status` read returned S, so the badge again had accessible name `Small pad` and no `unfit` class despite the warning. The call trace was `trade_route_defaults`, `search_filters`, `search_filters`, `trade_route`, `ship_status`; the ship read happens only after the route answers. The existing ship-swap regression changes ships before clicking Find route and does not exercise this case.

Capture and retain the automatic requirement when issuing the search, using the same resolution as the core, rather than reconstructing it after the job finishes. Add an event-driven regression that changes ships while the route answer is held and checks the completed answer's pad marking.
