---
schema: atelier.review
schema_version: 3
round: 1
commit: 546022ca552f0ebcc706378cf9e883ab7f8eec0a
reviewers:
- thread:delegated-task:command%3Amcp%3Ac73ca5a3-78ac-4dfd-8cc7-9e888add1097%3Adelegate-task%3A2f1b5b5d-38bf-4be9-91d7-d812d6436500
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T10:13:36.180329827Z
---

## Summary

Complexity: low. The eight pages reuse the existing Tip attachment; no new state or subsystem is introduced.

Value: medium. Removing repeated explanations makes pages in daily use easier to scan while retaining help beside the relevant value.

Risk of merging: low. The changes affect visible presentation and are straightforward to undo; calculations and commands are unchanged.

The value is worth the size and risk. The implementation already uses the existing shared component, so a smaller infrastructure change is not needed.

Reviewed `git diff main...546022ca552f0ebcc706378cf9e883ab7f8eec0a`, the eight pages and their child components, Tip's attachment and accessibility behavior, Badge/Input/Choice attribute forwarding, the source-label changes, the guide, and all six committed Powerplay/Exploration captures. Checked each Done when line against the code and artifacts. The main explanation moves and retained source/age lines match the Report. The Route page is untouched.

Ran `python3 scripts/check-guide.py` (22 pages, no problems), `pnpm check` (no errors or warnings), `pnpm lint`, `pnpm build`, `python3 scripts/check-dist.py` (no browser preview or dev bridge), and `git diff --check`. All passed. Plain `pnpm test` could not start its browser project because port 63315 was occupied; `pnpm test --browser.api.port 0` passed all 80 files and 580 tests.

I inspected the committed captures rather than retaking them. I did not exercise the Windows/Tauri app or the game, regenerate the live game-data catalog, or run Rust tests; the Rust implementation is unchanged.

## Finding 1: Keep Biology's five-times payout explanation in the guide
Recommendation: fix now

`app/src/modules/exobiology/View.svelte:130` removes the footer's explanation that a first logged species pays five times the Vista Genomics base value. The new `VALUE` Tip at line 37 preserves it in the app, but `docs/guide/exploration.md:63` only says the figures are before the first-logged bonus. Searching all of `docs/guide/` finds no explanation of the multiplier. The third Done when explicitly requires the guide to explain what was removed from the pages; the link checker cannot verify this content. Add the five-times rule to the Biology guide so the same payout condition remains available there.

## Finding 2: Remove the rewritten copy and static-markup tests
Recommendation: fix now

`app/src/modules/powerplay/Suggestions.browser.test.ts:23-59` rewrites all three remaining cases around copy and static markup: activity names and order, literal description strings, badge presence, `.unverified`, and the text referenced by `aria-describedby`. They render invented Suggestion objects and inspect the resulting markup without an interaction or state change. `CONTRIBUTING.md:236-254` explicitly applies its rules to changed tests and excludes copy and static-markup tests; lines 239-244 also require displayed data to come from recorded fixtures. These cases keep the change outside those rules despite passing. Remove the copy/markup cases rather than rewriting them for the new presentation. The committed captures already prove the page change, and `app/src/lib/ui/tip.browser.test.ts` covers Tip's hover, keyboard and description behavior at the shared layer.
