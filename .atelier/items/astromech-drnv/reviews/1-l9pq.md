---
schema: atelier.review
schema_version: 3
round: 1
commit: a252db3888e4063683a98b5f75d178b52d887cd3
reviewers:
- thread:delegated-task:command%3Amcp%3A129e272b-7e6c-4dde-a93c-54841696d631%3Adelegate-task%3A38e9923a-8f55-4a46-a99b-26a532a061a2
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T09:55:38.552208634Z
---

## Summary

Complexity: medium. The shared disclosure, focus handling and summary now serve both live-search finders and the explicit Find route consumer.

Value: medium. Every finder benefits from putting results ahead of rarely changed controls.

Risk of merging: medium. These are reversible UI changes, but hidden controls make an accurate summary necessary for understanding a search.

The value warrants the implementation's size. The shared component is an appropriate place for the disclosure; no substantially smaller implementation would deliver the same coverage.

Reviewed `git diff main...a252db3888e4063683a98b5f75d178b52d887cd3`, the item criteria and Report, DESIGN.md's collapsed-filter rule, the SearchKit consumers, filter persistence, summary builders, browser tests and guide edits. Confirmed that SearchFilters owns its open state, the normal controls remain mounted across searches and saves, and Route leaves the disclosure independent of Find route. Inspected all three committed Sell captures at 1920 x 1080.

Validation: `pnpm check` passed with zero errors or warnings; `pnpm lint` passed; `pnpm exec vitest run --browser.api.port 63427` passed all 77 files and 568 tests; `python3 scripts/check-guide.py` passed. The alternate browser port avoids concurrent worktrees' fixed-port collision. Also ran a temporary Chromium probe against the actual Mining hotspots page, using the preview transport and a partial-answer fault followed by a held request, to confirm Findings 1 and 2. Its browser and Vite server were closed. No implementation or test files were changed. The Windows/Tauri runtime was not checked; this frontend review used Chromium on Linux.

## Finding 1: Collapse Mining hotspots' Ring and Reserve controls too
Recommendation: fix now

`app/src/modules/places/Hotspots.svelte:78-83` still passes Ring type and Reserve choices through SearchKit's always-visible `lead` snippet (`app/src/lib/ui/SearchKit.svelte:269`). Only the radius enters the new disclosure. On `#/places/mining-hotspots/painite`, Chromium confirms that both choice groups are visible while Change filters has `aria-expanded="false"`; DONE likewise leaves them visible. These constrain the mineral search rather than identify the mineral, which already has its own always-visible input. This misses the first Done when line's requirement that a finder at rest have no filter controls. Route these additional controls through the same disclosure while keeping the mineral input visible, and cover this consumer.

## Finding 2: Keep a newly selected radius visible while its search runs
Recommendation: fix now

`app/src/lib/ui/SearchKit.svelte:251` combines current filters with the previous answer's `reach` and notes, and `app/src/lib/search.ts:247` gives that reach precedence over the selected radius. Since this change keeps the summary visible during loading, an earlier partial Mining hotspots answer can hide the newly selected non-default radius for the entire next request. Reproduced with a partial answer covering 32 ly from a 50 ly request: change Within to 200 ly, then press DONE while that request is pending. The closed summary remains `Within 32 ly of Sol · 1,101 bodies within 50 ly, too many to rank`; it never names 200 ly even though that is the request in flight. This contradicts both the criterion that a non-default filter is always named and the Report's claim that the summary follows each edit at once. Build the current constraint line from the current radius; omit obsolete coverage metadata during the next request or clearly separate it from the current constraints. Preserve the existing disclosure state across that change.

## Finding 3: Use recorded results for the new two-filter browser test
Recommendation: fix now

`app/src/lib/ui/SearchKit.browser.test.ts:236-248` fabricates successful result rows whose system names encode the selected filters (`Within 50, 7 days`) and supplies them through a mocked `run` callback. Consequently the new required scenario asserts that its own fake echoed those values, rather than exercising a real consumer's search adapter and recorded results. CONTRIBUTING.md's Tests rules explicitly require displayed data to come from recorded fixtures and UI doubles to sit at the transport boundary; those rules apply to new tests. Exercise the two edits through a real finder using the recorded preview data and transport helpers, assert the outgoing filters and resulting rows, and inject the search failure at the transport boundary. This can stay scoped to the new scenarios rather than expanding into a rewrite of all older SearchKit tests.
