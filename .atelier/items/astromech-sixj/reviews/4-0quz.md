---
schema: atelier.review
schema_version: 3
round: 4
commit: 49637362351f44495fda7be2e76f5471b9d6a510
reviewers:
- thread:delegated-task:command%3Amcp%3A685172b7-45c1-415c-bdbf-c75790f5874c%3Adelegate-task%3A89b44256-d224-44c4-b235-bb9addfb4005
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T09:57:14.494356155Z
---

## Summary

- Complexity: medium. One shared attachment owns tooltip placement, dismissal, descriptions and focus across many components.
- Value: medium. Instant explanations improve controls and values used throughout the current app.
- Risk of merging: medium. A failure affects visible explanations or keyboard use across the window, but shows and is reversible.

The value warrants the implementation's size. I reviewed the full change at `49637362351f44495fda7be2e76f5471b9d6a510`, the changes since round 3, each Done when line, the Report, all remaining native title attributes, the design and guide updates, and both evidence captures. The earlier fixes remain present. The title migration, guard against new interactive native titles, keyboard behavior and accessible descriptions meet their claims. No new findings.

Validation passed: `pnpm lint`, `pnpm check`, all three Tip node tests, all 56 browser tests across Tip, SearchFilters, Engineering Build and View, Colonisation, Station and Powerplay, and `python3 scripts/check-guide.py`. The browser project's API used port 63429 to avoid the other running server.

I also ran the actual Vite app in headless Chromium at 1200 x 800 and 2560 x 1440 with `?core=ready#/engineering`, hovered Help and a jump value, and measured their drawn rectangles. At root zoom 1.44, Help's Tip is at x=2181..2548 in the 2560 px window, about 11.5 px below its target; the jump-value Tip is at x=1188..1414, about 11.5 px above its target at x=1295..1307. Both stay beside their targets and inside the window. The zoom-1 checks pass too.

I did not repeat the entire Vitest suite, production build, Rust checks or journeys, test Windows WebView2 or use a physical screen reader. Accessible descriptions were checked through the Chromium component tests. Only this review file was edited.

## Finding 1: Convert Tip placement to the root zoom's coordinate system
Recommendation: fix now
Was: 3-buf7.1
Response: fixed in fe1dc319: placement measures target and Tip in screen px and writes back divided by the Tip's `currentCSSZoom`; the width cap uses `--win-w`; a browser test at zoom 1.44 checks both placements, and the real app at 2560x1440 puts Help's Tip 12 px below it, inside the window.
Now: fixed

Verified fixed: `place()` measures both rectangles in viewport coordinates, scales the gap and edge consistently, and divides the output position and pointer offset by the tooltip's effective CSS zoom. The width cap uses `--win-w`. The new non-unit-zoom browser regression passes, and measurements in the running app confirm the previously offscreen Help Tip and displaced jump-value Tip now sit beside their targets.
