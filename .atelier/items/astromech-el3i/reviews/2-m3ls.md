---
schema: atelier.review
schema_version: 3
round: 2
commit: 4c589d3e06a3b45dcbbbada5e878bce17d19f687
reviewers:
- thread:delegated-task:command%3Amcp%3A4b7b4848-359d-4f1d-86e9-c6fca2d53526%3Adelegate-task%3A92010fef-51e0-4492-ad11-827e6072ef44
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T13:06:27.090686905Z
---

## Summary

Complexity: medium. The result layout is bounded, but the new pad badge depends on an asynchronous snapshot of the search's effective filters.

Value: high. Stops joined by legs and the leading hourly estimate directly deliver the item's goal.

Risk of merging: medium. A wrong pad badge misleads the commander, though the core's warning remains visible and the change is easy to revert.

The value warrants the size. I found no materially smaller change that delivers the requested result. The explicit-override fix works, but the automatic-ship path still needs fixing before merge.

I reviewed the full diff from main to `4c589d3e06a3b45dcbbbada5e878bce17d19f687` and the changes since round 1 against every Done when line and the Report. I read the route construction and filter resolution in the core, the shared filter controls, Workbench, type roles, Tip and pad components. The totals, ordered stops and cargo, repeated loop endpoint, empty return, state rendering, token enforcement and guide otherwise match the item.

I independently ran `pnpm check`, `pnpm lint`, `pnpm test` (97 files, 899 tests), `python3 scripts/check-guide.py`, and `ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core node --test 'journeys/*.journey.ts'` (7 passed). All passed. No core files differ from main. The suite includes the new pending-save explicit-override regression and the ship-swap regression.

I inspected the committed single-hop and loop captures at 1100 and 2560, their before captures, and the 1920 single-hop capture. I checked the plans and all 20 audit entries: the requested sizes, text steps and sidebar states are present, and all six fault lists are empty. The source change since the audit's `1fb62535` is pad-resolution logic, not markup or styling. I did not recreate the before captures or rerun the full audit, and did not check live Spansh or Windows WebView2.

I also mounted the real Route component in headless Chromium with only the transport replaced, held the filter save unresolved, and reproduced Finding 2 without changing repository source files.

## Finding 1: Use the submitted pad for an explicit override
Recommendation: fix now
Was: 1-sh7u.1
Response: fixed in 28201cda
Now: fixed

`find()` now retains the submitted filters and uses `sent.pad` when `sent.pad_set` is true. The new regression holds the save while the search completes and checks both the submitted M override and the small pad's explanatory accessible name. That fixes the explicit-override reproduction. The remaining automatic-ship case is Finding 2.

## Finding 2: Resolve the ship's pad when an override is being cleared
Recommendation: fix now

In `app/src/modules/trade/Route.svelte:85-86`, `followShip()` returns `undefined` whenever storage still contains an explicit pad override. At line 113, a search with `sent.pad_set === false` then falls back to `sent.pad`. That field is not the effective pad for an automatic search: the core replaces it with the current ship's pad (`crates/astromech-galaxy/src/filters.rs:103-107,160-161`), even when storage still holds an override. The pad snapshot can therefore still disagree with the route's warning.

Reproduction in the real component: the current ship needs M, and storage starts with `{ pad: "S", pad_set: true }`. Open the filters, hold `set_search_filters` unresolved, and click `Pad S, current ship` (the initial controls derive that label from the loaded override). This clears `pad_set`. Press Find route before releasing the save. The captured search has `{ pad: "S", pad_set: false }`, which the core resolves to M; the parallel `search_filters` read still returns the stored explicit S override. Return the recorded single hop with its destination pad changed to S and the too-small-for-M warning. The route displays that warning, but the badge is `Small pad` with no `unfit` class, rather than the required red, struck-through pad with its reason. The result freezes that incorrect requirement, so the later save does not repair it.

Resolve the automatic search's ship requirement independently of a persisted override, rather than treating the submitted pad as authoritative when `pad_set` is false. Preserve the corrected explicit-override behavior and the ship-swap behavior. Add the boundary regression for clearing an override while its save is pending.
