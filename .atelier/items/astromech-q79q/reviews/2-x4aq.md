---
schema: atelier.review
schema_version: 3
round: 2
commit: 22055b6742f4851bcbca9e2dd497d3c673d2cb4d
reviewers:
- thread:delegated-task:command%3Amcp%3A9f4927b5-7446-4540-87a5-04f4cae0b321%3Adelegate-task%3Ab414fbb5-5b74-448b-8bda-095678c19f9f
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T09:54:22.651825414Z
---

## Summary

Complexity: medium. The bar adds a window command, generated bindings, polling and a fleet cache across Rust and Svelte.

Value: high. Keeping the current ship and app health on every page is the item's requested outcome.

Risk of merging: medium. The change affects every page's frame and displayed ship data, but is straightforward to undo.

The value justifies the size. I found no substantially smaller change that would deliver the outcome. The implementation addresses both earlier findings, but the new regression test must follow the repository's fixture rules before merging.

I reviewed the full diff against main at 22055b6742f4851bcbca9e2dd497d3c673d2cb4d and the changes since round 1. I checked every Done when line, the game-state sources and fleet invalidation, the generated bindings, fake handler, world and fixture registration, browser tests, guide and both committed captures. The cache now notices same-ID changes to name and jump range; the new browser regression passes. The guide describes the footer, availability and refresh behavior and updates the header widths.

I ran `pnpm check`, `pnpm lint`, `pnpm test` (566 passed), and `python3 scripts/check-guide.py` (22 pages, no problems). In headless Chromium, with fonts loaded, I independently measured the materials frame at 1100 x 700: header 108 px, route strip 36 px, page tabs 44 px, bar 32 px, leaving main 480 px. At 2560 x 1440, the header is 64 px, main 1264 px and bar 32 px.

The first `cargo test -p astromech-core -p astromech-app` run failed in `every_preview_fixture_is_the_cores_reply_or_says_why_not`: the unchanged exploration fixture expected two organic samples, but the core returned one (organic 33,324,500 versus 19,010,800). A diagnostic rerun of that test passed. The remaining Rust tests passed when run with that fixture test excluded; two existing helper/voice tests remain ignored. This is an intermittent result, not a clean all-green run. The new ship fixture and every-window-command case passed.

I did not run the Windows/Tauri window or live Elite, independently repeat the exact header breakpoint search, or rerun clippy/fmt. No implementation files were changed during review.

## Finding 1: Refresh the current ship when its loadout changes without a ship swap
Recommendation: fix now
Was: 1-77tl.1
Response: fixed in 8474debf: ship_status also carries the last Loadout's ship_name and max_jump_range, and the bar reads engineering_fleet again once whenever ship_id, name or range changes; StatusBar.browser.test.ts covers a same-ID range change (18.0 to 21.4 ly).
Now: fixed

## Finding 2: Update the window guide for the moved health line and new ship bar
Recommendation: fix now
Was: 1-77tl.2
Response: fixed in 8474debf: docs/guide/using-the-app.md describes the status bar (ship, jump range, fuel and cargo, when they drop out, the 2 s refresh) and the health line in it, and gives the remeasured header widths.
Now: fixed

## Finding 3: Dump the refit replies from a world instead of inventing successful test data
Recommendation: fix now

`app/src/lib/ui/StatusBar.browser.test.ts:44-51` writes `max_jump_range: 21.4` directly into both successful command replies. No raw Loadout or dumped fixture produces this refitted state; the test constructs the matching replies itself. CONTRIBUTING.md's Tests rules explicitly require data the page shows to come from recorded fixtures and permit replies written in tests only for failure, empty/absent states or faults. Using the transport seam satisfies the mocking boundary rule but does not exempt the reply data from that requirement.

Add a same-ID refit Loadout step to a world and dump its `ship_status` and `engineering_fleet` replies through `preview_fixtures.rs`. Have this browser case switch between those dumped replies on the virtual tick. Keep the regression assertion so the case still fails without the cache invalidation fix.
