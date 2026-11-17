---
schema: atelier.review
schema_version: 3
round: 2
commit: e8a65c0c5e8a4be0ade6e402436be2695d90f771
reviewers:
- thread:delegated-task:command%3Amcp%3Ac6ad92d8-d76e-4058-bbd3-0d44677a0cb3%3Adelegate-task%3A7396e844-6ad2-4342-b448-7257de88b7ad
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T12:01:34.397020189Z
---

## Summary

Complexity: medium. The resource/shopping join, shared Meter's second segment and SearchKit's ListDetail context are several interacting parts, but the change fits one review.

Value: high. Delivered, aboard and still-to-buy quantities together deliver the progress view the item requires.

Risk of merging: medium. Incorrect progress or purchase advice reaches the commander, but the UI is visible and the change is easy to revert.

The value justifies the size. I found no substantially smaller change that delivers the required progress rows and station details.

I reviewed the full diff from main to e8a65c0c5e8a4be0ade6e402436be2695d90f771 against the Done-when lines, then examined the changes since round 1. I checked progressOf against the Rust shopping-list producer and commodity_id, followed needsOf through the page's search branch, and read the Meter, SearchKit and ListDetail contracts. Both carried findings are fixed; no new findings.

I inspected the committed before-and-after captures at 1100 and 2560, the 1920 capture, the selected station panel and the complete state, and checked both audit reports against the Report. Their measurements match its claims: only the documented Stock ellipses remain, and the panel exposes the full stock. The layout, type roles, progress and aboard segments, row ordering, site selection, search origin, sizing lint activation and guide match the item. Colonies remains outside ListDetail so its search cannot take over the purchase panel. Its pre-existing paragraph is the scope exception already identified in the Report; changing Colonies is explicitly excluded by the item.

Independent checks passed: all 13 Colonisation cases (including the nine browser cases and both regression tests); pnpm check (zero errors or warnings); pnpm lint; pnpm test --maxWorkers=1 (92 files, 790 tests); all seven journeys; node --check scripts/browser/audit.mjs; python3 scripts/check-guide.py (22 pages, zero problems); and git diff --check. I stopped the first default-concurrency full-suite run after it showed failures across unrelated pages while several other worktrees were also running full suites. The subsequent one-worker run completed without failures; no test timeout, retry or code was changed.

The journeys used the existing /home/supadava/projects/astromech/target/debug/astromech-core through ASTROMECH_CORE. I did not rebuild the core or run the Rust suite because no Rust changes here. I inspected the supplied captures rather than regenerating them, and did not exercise the Windows WebView or live journals.

## Finding 1: Suppress the purchase search when a construction project has failed
Recommendation: fix now
Was: 1-aj8y.1
Response: fixed in cb35a7d9: `needsOf` asks for nothing once a project is complete or has failed, so the page shows "Nothing left to deliver." and mounts no search; covered at the lowest layer that failed, `colonisation.test.ts` ("asks for nothing once a project has failed").
Now: fixed
Verified: app/src/lib/colonisation.ts:82-84 now returns no needs for complete or failed projects, independent of outstanding shopping lines. View.svelte:132 therefore cannot mount the purchase SearchKit for them and reaches the existing terminal message at line 147. The regression at colonisation.test.ts:19-22 retains the fixture's unfinished resources, sets failed and passes.

## Finding 2: Preserve a readable commodity fallback when localised names are absent
Recommendation: fix now
Was: 1-aj8y.2
Response: fixed in cb35a7d9: a row's name falls back to the shopping line's symbol, else the resource's journal name normalised as the core's `commodity_id` does; covered in `colonisation.test.ts` ("names a commodity by its market symbol").
Now: fixed
Verified: app/src/lib/colonisation.ts:58 preserves localised names first, then uses the shopping symbol or commodityId for a resource absent from the shopping list. commodityId at lines 75-78 matches crates/astromech-colonisation/src/shopping.rs:112-115. The regression at colonisation.test.ts:24-31 clears both sets of localised names and verifies symbols for incomplete commodities and completed Aluminium. It passes.
