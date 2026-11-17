---
schema: atelier.review
schema_version: 3
round: 1
commit: 48659fded5d1c201ead9382cb9be2053ea5e8f5d
reviewers:
- thread:delegated-task:command%3Amcp%3A4b7b4848-359d-4f1d-86e9-c6fca2d53526%3Adelegate-task%3A9fcdf580-da0d-45a8-96cc-72476c183786
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T12:59:44.958845093Z
---

## Summary

Complexity: medium. The change is bounded to the trade result and its type roles, but adds a shared pad component and an asynchronous snapshot of the pad used for an answer.

Value: high. Reading the route as stops and legs, with the hourly estimate first, directly delivers the item's goal.

Risk of merging: medium. Incorrect pad marking can mislead a player, though the core's warning remains visible and the UI change is easy to revert.

The value warrants the change's size. No materially smaller implementation would deliver the requested result. One pad snapshot bug needs fixing before merge.

I reviewed `git diff main...48659fded5d1c201ead9382cb9be2053ea5e8f5d`, checked every Done when line against the implementation and Report, and read the underlying filter resolution, trade-route construction, Workbench, tooltip and typography rules. The totals, sequential stops and legs, repeated loop endpoint, empty cargo, state rendering, token enforcement and guide update otherwise match the item.

I independently ran `pnpm check`, `pnpm lint`, `pnpm test` (97 files, 898 tests), `python3 scripts/check-guide.py`, and `ASTROMECH_CORE=/home/supadava/projects/astromech/target/debug/astromech-core node --test 'journeys/*.journey.ts'`. All passed. The shared debug core is the Report's binary; there is no core diff against main. I inspected the committed before/after captures, including the single hop and loop at the narrow size and the large-text loop, and the 1920-wide single hop and failure/empty captures. The committed audit report contains 20 captures with the specified sizes, text steps and sidebar states, and no entries in the six fault lists. No source code changed after its recorded commit. I did not independently recreate the before captures or rerun the full visual audit, and did not exercise Windows WebView2 or live Spansh.

I also ran the real Route component in headless Chromium with only its transport replaced, holding a filter-save promise until after the route appeared. That reproduced Finding 1 without sleeps or repository source edits.

## Finding 1: Use the submitted pad for an explicit override
Recommendation: fix now

In `app/src/modules/trade/Route.svelte:105-111`, a successful `followShip()` read always overrides the pad submitted with the search. That read loads the persisted filters (`:85-87`), whereas `changeFilter()` makes an edit immediately searchable before its save finishes (`:72-77`). The two can therefore refer to different pad choices. The core resolves the filters supplied to `trade_route`, not those in storage (`crates/astromech-api/src/lookups.rs:419` and `crates/astromech-galaxy/src/filters.rs:160`). Tauri dispatches commands through independent blocking tasks, so completion order is not guaranteed.

Reproduction: start with saved `{ pad: "S", pad_set: true }`, select Pad M, hold `set_search_filters` unresolved, and press Find route. Let `search_filters` return the still-saved S choice and let `trade_route` return the recorded single hop with its destination pad changed to S and the existing too-small-for-M warning. The captured request has `{ pad: "M", pad_set: true }`, and the warning says the station has no M pad, but the badge has accessible name `Small pad` and no `unfit` class. Releasing the save does not correct it because `need` has already been frozen as S. This misses the required red, struck-through pad and explanatory accessible name on a stop that the search rules out.

Preserve the submitted pad when `pad_set` is true, and make the automatic-ship path resolve the ship requirement without substituting an unrelated saved override. Keep the existing ship-swap behavior. Add a boundary test that holds the save while the search completes and verifies that the badge agrees with the pad used by that search.
