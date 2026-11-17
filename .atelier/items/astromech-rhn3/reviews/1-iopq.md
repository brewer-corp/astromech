---
schema: atelier.review
schema_version: 3
round: 1
commit: 20422ea58105be1c61b2f6b360e77b9723735455
reviewers:
- thread:delegated-task:command%3Amcp%3A1c1678ee-aa3d-49c1-aced-716462eff510%3Adelegate-task%3Af76e3131-e556-4069-b69a-d96221eaafb1
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T09:55:44.418079489Z
---

## Summary

Complexity: medium. The available width now flows from Split through ListDetail's context to SearchKit, with a parent observer for standalone lists.

Value: high. Widening the window restores the result groups in the finders and standalone lists instead of leaving information collapsed.

Risk of merging: low. This changes visible, reversible layout state and does not persist data or change backend calls.

The value warrants the implementation size. Measuring the allocation rather than the shrink-to-fit rows fixes the feedback loop; the shared panel-width calculation keeps the compact and stacking decisions consistent.

I reviewed the complete diff at 20422ea58105be1c61b2f6b360e77b9723735455, the shared layout and result-row code, the standalone callers' parent layouts, DESIGN.md and the test rules. Both new regression cases pass, as do both dock-width tests across all Text size steps. The targeted run passed 55 tests; the full `pnpm test` run passed 90 files and 755 tests. `pnpm lint`, `pnpm check` (zero errors and warnings) and `git diff --check main...HEAD` passed. Git history confirms the regression tests were committed before the fix. I did not independently rerun the historical red commit or exercise native Windows WebView2.

I found no production-code correctness issue, but the new and changed component cases need to follow the repository's test boundary and fixture rules.

## Finding 1: Use the recorded lookup replies in the component regressions
Recommendation: fix now

`app/src/lib/ui/SearchKit.browser.test.ts:151` constructs a successful `Results` response by hand and injects it through `run: vi.fn().mockResolvedValue(sell)` at line 162. The changed narrow-column case at line 138 likewise injects the handwritten `results` response. These bypass the real lookup conversion and the preview transport; they are not failure or empty-state setups.

CONTRIBUTING.md's Tests section explicitly applies its rules to new and changed tests (lines 243–244), requires displayed data to come from recorded fixtures (lines 246–251), and requires UI tests to fake only the transport. The existing older cases do not exempt these touched cases. The new response also differs from the recorded Parekh offer in `app/src/preview/fixtures/trade.json`, for example in its jump count, exactness and report date, despite the Report calling it the Sell fixture.

Keep the real-width resize assertions, but obtain the results through the real lookup functions and their result converters, using the preview transport's recorded trade and nearby replies. This can be confined to the new regression and the changed narrow-column case; it does not require migrating unrelated cases in this file.
