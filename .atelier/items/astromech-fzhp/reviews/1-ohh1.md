---
schema: atelier.review
schema_version: 3
round: 1
commit: 7f9323ab91bfc972bc10a719bfa67c56a0e32cbe
reviewers:
- thread:delegated-task:command%3Amcp%3Abfa67aea-4f37-4552-bca8-53dbcf101075%3Adelegate-task%3Ad5e2757c-5284-475f-aa41-01f75f75b2b9
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T12:17:27.368325507Z
---

## Summary

Complexity: medium. The change connects a Rust history query, generated API types, two page layouts, day grouping and the browser audit tool.

Value: medium. Earnings over time and travel by day make two existing history pages easier to read.

Risk of merging: medium. These are user-facing history displays and a shared session reply; errors are reversible but can hide payment details.

The value warrants the scope. The query reuses the totals' commander and session bounds, and the page changes fit the requested outcome. No substantially smaller change would provide both views. One payment-band interaction needs fixing before merge.

Reviewed `main...7f9323ab91bfc972bc10a719bfa67c56a0e32cbe` against each Done when line, including the SQL window and ordering, generated types, fixtures, UTC grouping, retained states and facts, type roles, sizing enforcement, audit plans, and guide changes. Inspected the before/after captures and the saved audit report. Also exercised the Session page in headless Chromium with the recorded fixture and with a real core reply from a temporary copy of the history world containing two same-second trade payments. No application files or committed fixtures were changed.

Independent checks passed: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test -p astromech-history` (37 tests); the core bindings tests (2); the preview fixture drift tests (3); `pnpm check`; `pnpm lint`; focused Session, Logbook, Now and grouping tests (18); the full `pnpm test` run (96 files, 848 tests); and all 7 journeys. Full test suites ran sequentially. I did not rerun every saved audit capture or test in the Windows WebView/game environment.

## Finding 1: Preserve access to payments whose marks occupy the same position
Recommendation: fix now

In `app/src/modules/session/View.svelte:105-111`, each payment gets an independent mark positioned only by timestamp. The absolute positioning at lines 154-156 gives same-kind payments at the same timestamp identical hit areas, so the later mark completely covers the earlier one. With `stop: false`, there is no keyboard focus fallback. The page therefore cannot show the earlier payment's amount through its advertised hover interaction.

Reproduced using a temporary copy of `worlds/preview/history`: change the 3,000 cr MarketSell timestamp from `10:26:00Z` to `10:18:00Z`, import it with the real core and request `session_summary`. The core correctly returns both the 1,500 cr and 3,000 cr trade payments at `10:18:00Z`. Render that reply at 1100 x 700: both marks have the same rectangle (`x=863.0625`, `y=515.46875`, `width=10`, `height=15`). Hovering their shared hit area shows only “3,000 cr / Trade / 10:18 UTC”; the 1,500 cr payment is inaccessible. The By activity row still counts all three trade payments, while the band appears to have only two.

Handle colliding marks without losing the underlying payments, for example by having the shared hover target enumerate every payment at that position. Cover the collision through raw journal input and verify that the resulting interaction exposes both amounts. This is needed for the Done when requirement that each payment's time, kind and amount be available on the band.
