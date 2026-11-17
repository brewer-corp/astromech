---
schema: atelier.review
schema_version: 3
round: 2
commit: 2d55eada1a3b54af5ecb24a3467fd9da7f2e56ad
reviewers:
- thread:delegated-task:command%3Amcp%3Aef563057-ac92-410c-aab1-67840d8b977b%3Adelegate-task%3A5a6c63ad-0891-4484-8f5e-2be37a880734
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T12:16:37.741586965Z
---

## Summary

- Complexity: medium. The change joins the journal's parents, tree construction, icons, keyboard selection and a measured layout threshold across the core and window.
- Value: high. The hierarchy and selected-body facts deliver the item's outcome on the current System page.
- Risk of merging: medium. A layout or keyboard regression reaches the commander but is visible and reversible; the change writes no game or commander data.

The value warrants the implementation's size. I found no substantially smaller change that delivers the tree and its detail panel. Both carried findings are fixed, and I found no new problem requiring action.

I reviewed the whole diff from main to 2d55eada, the changes since round 1, every Done when line, the Report, DESIGN.md and CONTRIBUTING.md. I checked the parents at the journal/core/type boundary; tree ordering, missing ancestors, barycentres and loop handling; icon mapping; selection and polling; badge Tips; the responsive panel; unchanged unsold-data content and sell links; loading, failure and empty states; preview world and fixture changes; lint enrollment; and the guide. The previous deferred selection finding is also fixed: a system identity change clears the stored selection, and the jump-and-return browser test passes.

Independent checks passed:

- All 25 exploration crate tests, all 3 preview fixture tests, all 41 API tests, both core binding tests and all 12 copy tests.
- The full frontend suite: 92 files, 802 tests. The focused tree/icon, System view, stale-poll and empty-history checks passed too.
- `pnpm check`, `pnpm lint`, `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings`.
- `pnpm exec tsc -p journeys` and all 7 journeys.

I drove Chromium against the local browser preview, with the route followed and A 4 a selected, at 1100 x 700 and 1659 x 900 at 100 percent Text size, 1920 x 1080 at 100 percent, 2560 x 1440 at 145 percent and 3840 x 2160 at 180 percent. The panel stacks at 1100 and 1659 and sits beside the tree at the larger sizes. No tree badge wrapped and no main-column sideways overflow appeared. At 1920 the tree measured 946 px wide, followed by the 48 px gap and the panel, leaving spare width after it. I also supplied the supported longer class, Gas giant with ammonia-based life, through the preview transport at 1659 and 1920: the tree grew to 1003 px without wrapping or overflow, and panel placement remained correct.

I inspected the saved selected capture at 1659 and the unselected capture at 2560, the plan and the saved audit report. All six fault lists are empty for every saved capture; the reported brightness explanation matches the prescribed White titles and Orange Lit main values. I independently measured the selected layouts but did not regenerate the complete screenshot/audit matrix. I did not run the whole Rust workspace test suite, a release build or the Windows webview.

## Finding 1: Keep badge explanations reachable from the keyboard
Recommendation: fix now
Was: 1-flbt.1
Response: fixed in 9258322e. The panel repeats the selected body's tags with Tips that keep their tab stops, and Tab from the selected row reaches them (tested in the moon case).
Now: fixed

`app/src/modules/exploration/View.svelte:267` repeats the selected body's tags using the same `tags` function as the tree, with focusable Tips. In Chromium, Tab from the selected moon reached Landable, then Bio 2, Geo 1, First discovery, First footfall likely and Landed 2 times. Each opened its visible explanatory Tip, including the uncertainty behind First footfall likely. The browser test also verifies the row-to-panel keyboard path. The tree keeps its single row tab stop while the panel provides access to the explanations.

## Finding 2: Stack the panel before the tree's badges have to wrap
Recommendation: fix now
Was: 1-flbt.2
Response: fixed in 9258322e. The panel's threshold is the tree's measured natural width, so at 1659 x 900 it stacks and no tag wraps (captures and audit in 8c3f8b47).
Now: fixed

`app/src/modules/exploration/View.svelte:144` measures the three fixed tracks plus each row's unwrapped badges, gaps and padding, normalizes that width for Text size and passes it to ListDetail. The fixed 800 px threshold is gone. Independent Chromium measurements reproduce a stacked panel and unwrapped badges at the 1659 px dock width, with the panel alongside at 1920 and at 2560 with 145 percent Text size. The longer supported gas-giant class also remained unwrapped at 1659 and 1920. The revised capture and Report agree with this behavior.
