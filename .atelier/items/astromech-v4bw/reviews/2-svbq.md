---
schema: atelier.review
schema_version: 3
round: 2
commit: 8431cac061fe7ac17e617283f422a6d34a559c08
reviewers:
- thread:delegated-task:command%3Amcp%3Ac73ca5a3-78ac-4dfd-8cc7-9e888add1097%3Adelegate-task%3A20dc34e3-8ddc-4fdc-9979-f0a728b6cbe6
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T10:17:21.676323533Z
---

## Summary

Complexity: low. The eight pages reuse the existing Tip attachment and attribute forwarding; no new state or subsystem is introduced.

Value: medium. Pages in current use become easier to scan while explanations remain beside the relevant figures and tags.

Risk of merging: low. The changes affect visible presentation and are straightforward to undo; calculations and commands are unchanged.

The value is worth the size and risk. Reusing Tip is an appropriately small implementation.

Reviewed the full diff from `main` to `8431cac061fe7ac17e617283f422a6d34a559c08` and the changes since round 1. Checked the eight pages and their child components, Tip's accessibility behavior, Badge/Input/Choice attachment forwarding, source labels, the guide, and all six committed Powerplay/Exploration captures against the Done when lines and Report. The moved explanations and retained source/age facts match the intended behavior. The excluded Route page is untouched. Both carried findings are fixed; one changed static-render test remains outside CONTRIBUTING.md's rules, as Finding 3 describes.

Ran `python3 scripts/check-guide.py` (22 pages, no problems), `pnpm check` (no errors or warnings), `pnpm lint`, `pnpm build`, `python3 scripts/check-dist.py` (no browser preview or dev bridge), and `git diff --check main...8431cac061fe7ac17e617283f422a6d34a559c08`. All passed. The first test invocation, `pnpm test --browser.api.port 0`, hit the occupied default port 63315; `pnpm test --api.port 63427` passed all 79 files and 577 tests.

I inspected the committed captures rather than retaking them. I did not exercise Windows/Tauri or the game, regenerate the live game-data catalog, or run Rust tests; Rust code is unchanged.

## Finding 1: Keep Biology's five-times payout explanation in the guide
Recommendation: fix now
Was: 1-bvka.1
Response: fixed in bb269601: `docs/guide/exploration.md` says a species nobody had logged pays five times.
Now: fixed

Verified `docs/guide/exploration.md:63-64` now preserves the five-times rule and points to the Value heading's Tip.

## Finding 2: Remove the rewritten copy and static-markup tests
Recommendation: fix now
Was: 1-bvka.2
Response: fixed in bb269601: `Suggestions.browser.test.ts` deleted; Tip's own tests cover the description.
Now: fixed

Verified `app/src/modules/powerplay/Suggestions.browser.test.ts` is deleted. The shared Tip browser tests still cover hover, keyboard focus and accessible descriptions, and pass.

## Finding 3: Remove the remaining Powerplay badge-count render test
Recommendation: fix now

`app/src/modules/powerplay/View.browser.test.ts:98-104` changes the “shows what earns merits in the current system” case to assert one Ethos badge after the explanatory key was removed. This case only renders the page with invented `standing` and `undermine` replies and checks literal activity/heading text and badge count; it performs no interaction or state change. `CONTRIBUTING.md:236-254` explicitly applies its rules to changed tests, excludes copy, static markup and smoke renders, and requires displayed data to come from recorded fixtures. Removing Suggestions' tests fixes the carried finding, but leaves this changed case with the same problem. Remove this case rather than updating the badge-count assertion; the committed captures prove the visual change and the shared Tip tests cover its behavior. No cleanup of the unchanged cases is required here.
