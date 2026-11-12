---
schema: atelier.review
schema_version: 3
round: 4
commit: 1336f9ac09d7e497d7cf8cd458acdfe12db9bad7
reviewers:
- thread:delegated-task:command%3Amcp%3A9f4927b5-7446-4540-87a5-04f4cae0b321%3Adelegate-task%3A745424ff-2d83-4edc-af7b-dce431b7c40c
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T10:08:31.883104634Z
---

## Summary

Complexity: medium. The bar adds a window command, generated bindings, polling and a fleet cache across Rust and Svelte.

Value: high. Keeping the current ship and app health on every page delivers the requested outcome.

Risk of merging: medium. Every page's frame and displayed ship data change, but failures are visible and the change is straightforward to undo.

The value warrants the scope. I found no substantially smaller change that delivers the outcome and no problems requiring changes.

Reviewed the full diff against main at 1336f9ac09d7e497d7cf8cd458acdfe12db9bad7 and the changes since round 3, including the integration with main's root scaling and Tip attachments. Checked each Done when line and the Report against the command registration, session/game-state sources, generated bindings, fleet refresh logic, preview handler, recorded world, fixture dumper, browser tests, design specification, guide and both committed captures. The same-ID refit regression uses dumped ship_status and engineering_fleet replies from the world's refit step. The verifier and browser regression pass; the earlier fixture finding remains resolved.

Ran pnpm check, pnpm lint and pnpm test: all passed, with 584 frontend tests. cargo test -p astromech-core -p astromech-app passed, including every_window_command and every_preview_fixture_is_the_cores_reply_or_says_why_not; the two existing helper/voice tests remain ignored. cargo clippy -p astromech-core -p astromech-app --all-targets -- -D warnings, cargo fmt --all -- --check and python3 scripts/check-guide.py passed. git diff --check reports trailing whitespace only in the generated ShipStatus.ts binding; excluding that file passes.

Independently measured the current Materials frame in headless Chromium after fonts and layout settled. At 1100 x 700 and zoom 1, header 108 px, route strip 36 px, page tabs 44 px and bar 32 px leave main exactly 480 px. At 2560 x 1440, the merged height scale is now active at zoom 1.44 and the bar measures about 46 px on screen, preserving its 32 px layout height. A binary search found the first single-row header widths at 1455 px normally and 1605 px with Finish setup, consistent with DESIGN.md's approximate 1456/1606 px. The footer had no horizontal overflow at either required size.

The Report's note that height scaling has not landed predates the merge from main; the current code scales the bar correctly. The committed captures likewise predate that integration. I did not run the Windows/Tauri window or live Elite. No implementation files were changed during review.
