---
schema: atelier.review
schema_version: 3
round: 3
commit: 4a6ccc8226cf079d0d860635304231344db026e3
reviewers:
- thread:delegated-task:command%3Amcp%3A9f4927b5-7446-4540-87a5-04f4cae0b321%3Adelegate-task%3A0a6b20a4-bd8e-46a7-a17f-9c38753141d0
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T10:01:28.101993365Z
---

## Summary

Complexity: medium. The bar adds a window command, generated bindings, polling and a fleet cache across Rust and Svelte.

Value: high. Keeping the current ship and app health on every page delivers the requested outcome.

Risk of merging: medium. The change affects every page's frame and displayed ship data, but failures are visible and the change is straightforward to undo.

The value justifies the size. I found no substantially smaller change that delivers the outcome, and no new problems requiring a finding.

I reviewed the full diff against main at 4a6ccc8226cf079d0d860635304231344db026e3 and the changes since round 2. I checked every Done when line, the command's session/game-state sources, generated bindings, fleet refresh logic, fake handler, browser tests, guide, design specification and both committed captures. The same-ID refit case now switches between dumped replies, rather than constructing successful replies in the test. The ship world's refit Loadout produces both the new ship_status and engineering_fleet replies through preview_fixtures.rs. The fixture verifier and browser regression both pass, resolving the carried finding.

I ran pnpm check, pnpm lint and pnpm test: all passed, with 566 frontend tests. cargo test -p astromech-core -p astromech-app passed, including every_window_command and every_preview_fixture_is_the_cores_reply_or_says_why_not; the two existing ignored helper/voice tests remain ignored. cargo clippy -p astromech-core -p astromech-app --all-targets -- -D warnings, cargo fmt --all -- --check and python3 scripts/check-guide.py also passed. The unchanged exploration fixture's intermittent failure noted in round 2 did not occur in this run.

In headless Chromium, after fonts and layout settled, I measured the materials frame at 1100 x 700: header 108 px, route strip 36 px, page tabs 44 px, status bar 32 px and main 480 px. At 2560 x 1440 the header is 64 px, main 1264 px and bar 32 px. A binary search found the first single-row header width at 1455 px, consistent with DESIGN.md's approximate 1456 px.

I did not run the Windows/Tauri window or live Elite, or independently repeat the Finish setup breakpoint measurement. No implementation files were changed during review.

## Finding 1: Dump the refit replies from a world instead of inventing successful test data
Recommendation: fix now
Was: 2-x4aq.3
Response: fixed in d3c7ef9c: worlds/preview/ship has a refit step (ship 3's next Loadout at 21.4 ly, mark "refitted"); ship-status.json's flying, refitted and refitted_fleet parts are dumped by preview_fixtures.rs, and the refit test switches to those (it fails with the old id-only re-read).
Now: fixed
