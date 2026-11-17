---
schema: atelier.review
schema_version: 3
round: 1
commit: 460618e905d5fbda02932365d5890ee9c30395bb
reviewers:
- thread:delegated-task:command%3Amcp%3A8cdcc4c8-6a59-47cf-b305-ea17018c24a2%3Adelegate-task%3A4b54a634-46b1-4af1-b1ae-ea206ed1f97c
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T09:32:13.019788906Z
---

## Summary

Complexity: low. The runtime change is one shared dock threshold; the remaining changes measure it, test the existing layout, and document the result.

Value: high. This restores the failing main-branch test and lets 1920 px windows dock at 115 percent Text size while keeping every finder's full rows beside its panel.

Risk of merging: low. A wrong threshold produces a visible layout failure, and the constant is cheap to revert.

The value warrants the change, and its scope is appropriate. No findings.

Reviewed the full diff against main, the frame's docking and gutters, Split's stacking rule, SearchKit's row-width calculation, and the updated browser tests. Checked all Done when lines and the Report's window margins against the new threshold. DESIGN.md and the runtime constant agree.

Ran `pnpm test` (89 files, 752 tests passed), `pnpm lint` (passed), and `pnpm check` (zero errors or warnings) in app/.

Independently measured Mining hotspots in headless Chromium with a fresh page for every probe. A temporary in-memory Vite transform forced the sidebar to stay docked below the threshold without editing repository files. Binary searches found minimum window widths of 1659, 1907, 2154, 2402, 2649, and 2979 px at Text sizes 100, 115, 130, 145, 160, and 180 percent. At each boundary the full row and panel were geometrically beside each other; one pixel below it the panel stacked. These reproduce the Report's normalized measurements and support 1659 as the narrowest shared threshold. The passing browser tests cover all ten finders at all six Text sizes.

Did not run the native Windows WebView2 window. The separate compact-row feedback bug described in the Report was not independently reproduced in this review; the author has reported it to the lead for a follow-up item.
