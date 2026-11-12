---
schema: atelier.review
schema_version: 3
round: 3
commit: ec6866b201d19657f32316e25acba13e4e4c55c1
reviewers:
- thread:delegated-task:command%3Amcp%3Aadc1aaff-29c4-454f-8038-0b4c4e7be7ba%3Adelegate-task%3A1879ebc6-f908-4ca3-aebc-2c3c1fb4aefd
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T10:14:24.785849826Z
---

## Summary

Complexity: high. Four shared layouts, a selection/context contract, a lint plugin and conversions across several page owners must work together.

Value: high. Shared sizing and adjacent details are required for the revision 4 window design and its migration away from page width caps.

Risk of merging: medium. Layout and interaction failures reach users, but are visible and reversible.

The value warrants the scope. The shared split and small layout wrappers avoid repeating sizing behavior; I found no substantially smaller change that delivers the required layouts, conversions and enforcement.

I read the full code diff against main and the changes since round 2, checked each Done when line against DESIGN.md revision 4 and CONTRIBUTING.md, and reviewed layout sizing, SearchKit selection and panel actions, the page conversions, lint enrollment and rule tests. All 24 before/after PNGs exist at their stated dimensions; I inspected representative captures, including the minimum-window Materials and General pages and the large-window Sell and Commodities pages.

I independently ran `pnpm check` (0 errors and warnings), `pnpm lint`, `pnpm test` (79 files, 590 tests), `pnpm build`, `python3 scripts/check-guide.py` and `python3 scripts/check-dist.py`. All passed.

In headless Chromium against this checkout's Vite preview, I exercised Materials (raw, manufactured and encoded), Sell, Outfitting, Services, Commodities with a route result, and General settings at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160. The checked pages had no horizontal overflow. The narrow layouts stack their panels/results, while wide layouts keep panels beside their content and keep row actions at the end of their columns. I separately checked the first and second Sell selections at 1100 x 700 by instrumenting scrollIntoView and measuring the panel's position. The carried finding is fixed; I found no additional problems.

I did not exercise the Windows/Tauri window, live game/network data or native microphone calibration. Runtime checks used the repository's browser preview inputs. I changed no implementation file.

## Finding 1: Scroll the first selected result's panel into view after it mounts
Recommendation: fix now
Was: 2-ryk4.4
Response: fixed in c223ace2. Split now waits on the panel element and scrolls once per selection; the Sell finder test selects a result in a 1100 x 500 window and checks the panel is on screen (failed before the fix). In the app at 1100 x 700 the first and second selections both bring the panel into view.
Now: fixed

`Split.svelte` now tracks the bound aside and waits for it before scrolling, with a separate last-shown selection to prevent repeated scrolling for the same selection. The selected-panel browser test passes as part of the full suite. In the full app at 1100 x 700, the first selection produced one ASIDE scrollIntoView call and moved main's scrollTop from 0 to 737; Details occupied y=272 through 699.8, inside the viewport. Selecting the second row produced exactly one further call and updated the visible panel to Cousteau Vista. This verifies both the initial mount and an existing selection changing.
