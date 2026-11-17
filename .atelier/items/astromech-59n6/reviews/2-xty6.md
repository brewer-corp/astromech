---
schema: atelier.review
schema_version: 3
round: 2
commit: 19df75c3f6950e2a07528fed15b15527c366925e
reviewers:
- thread:delegated-task:command%3Amcp%3Ae33afcc8-cc43-4f7f-a9ef-dbefd4262c68%3Adelegate-task%3Ad21fc0da-dc95-45bc-8bfc-0f972bffceda
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T12:43:43.170121108Z
---

## Summary

Complexity: low. One shared component owns four visual states; its callers pass counts, with one derived SVG and no new application state or layout.

Value: medium. Fitted versus planned grades appear on the current Ships and Suits paths; offered rings clarify what an engineer could provide.

Risk of merging: medium. The component affects several user-facing pages, but failures are visible and the change is readily reverted.

The value warrants the implementation. A smaller change would not cover the required states and all callers equally well.

Reviewed the full diff against main at `19df75c3f6950e2a07528fed15b15527c366925e` and the changes since round 1. Checked DESIGN.md's state definitions against Hexes.svelte and every caller in Loadout, ModulePanel, Suits, Engineers search, panel and tree. Reached marks use White, plan additions Orange, offered grades Label Grey rings, and remaining grades Mark Off. No page CSS targets the hexagon states. Counts and accessible names are preserved. Verified that access-ring.svg contains exactly access.svg's outer path, without its centre path, and that NOTICE.md credits the derivative. The unrelated page-sizing test correction agrees with the current ESLint enrollment list.

Inspected the committed Loadout before/after images at 100 and 145 percent: the Frame Shift Drive's fitted 3/planned 5 reads distinctly beside the Power Plant's fitted 5. Also inspected Suits rows, selected panel and loadout strip, Engineers tree and panel, and search captures with rank-2 and locked engineers. The missing centre distinguishes offered rings from unreached marks. Compared the capture plans with the reports and PNG changes: all 12 Suits and 14 Engineers images were updated, as were 18 of 19 Loadout images. The unchanged empty-fleet image contains no hexagons. The pre-existing no-fleet capture-plan fault remains deferred in round 1 Finding 2; it does not block this change. I did not regenerate the full capture suites, independently reproduce that timeout, or check the Windows game runtime.

Ran `pnpm check` (zero errors or warnings), `pnpm lint`, `pnpm test` (96 files, 885 tests passed), and `python3 scripts/check-guide.py` (23 guide pages, zero problems). The Report's required check results hold. The guide correction resolves the carried finding; no new fix-now findings.

## Finding 1: Update the Engineers guide to describe the new grade marks
Recommendation: fix now
Was: 1-5f6q.1
Response: fixed in f9e327dd
Now: fixed

Verified `docs/guide/engineering.md:22` and `:63`: available grades are now described as white, with grey outlines for additional offered grades. `docs/guide/ships.md:59` and `:95` describe White fitted grades followed by Orange planned grades for Suits and ship Loadout. These descriptions match the shared component, its callers and the captures.
