---
schema: atelier.review
schema_version: 3
round: 1
commit: b683e58013c2b700741ed694fb76c92a072a6931
reviewers:
- thread:delegated-task:command%3Amcp%3Aef563057-ac92-410c-aab1-67840d8b977b%3Adelegate-task%3Ae50d8167-f803-470b-a209-ab49894ec36b
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T12:05:17.544977512Z
---

## Summary

- Complexity: medium. The change joins a new API field, tree construction, keyboard selection and a responsive detail panel.
- Value: high. A readable system hierarchy is the item's outcome and replaces the wide flat table on a page in current use.
- Risk of merging: medium. Layout and accessibility faults reach the commander, but they are visible and reversible; this change does not write game or commander data.

The value warrants the implementation's size. I found no substantially smaller change that delivers the hierarchy and selected-body facts. Two design regressions need fixing before merge.

I reviewed `git diff main...b683e58013c2b700741ed694fb76c92a072a6931`, the Done when lines, DESIGN.md, the core/journal boundary, tree and icon functions, selection and polling, the guide, world/fixture changes, lint enrollment, and the saved captures, plan and audit report. The parents are copied in journal order, and the tree handles missing ancestors and barycentres as specified. The saved audit's six fault lists are empty at every captured size. Its brightness explanation is consistent with the prescribed White titles and Orange Lit main values.

Independent checks passed: all 25 exploration crate tests; the preview fixture drift tests; 66 focused frontend tests covering the tree/icons, system view, empty history and stale polling; `pnpm check`; `pnpm lint`; `cargo fmt --all -- --check`; and `cargo clippy --workspace --all-targets -- -D warnings`. The full frontend run passed 799/801: the existing transport module-loading test and ESLint configuration test each exceeded their 5-second limit while the host load was about 45. Both files subsequently passed in isolation (36 tests, 2.16 seconds), without a code or timeout change. I did not obtain a fully green full-suite run or independently rerun the journeys, generated-binding or API tests.

I also drove Chromium against the local preview, using the real page and transport-boundary replies, to measure the selected moon at the 1659 px dock width, examine keyboard access to badge Tips, and exercise a system change followed by a return. The longer gas-giant class I checked at 1100 px stayed within the page. I inspected the saved selected-body captures at 1100 and 2560; I did not recreate the complete capture/audit matrix or check the Windows webview.

## Finding 1: Keep badge explanations reachable from the keyboard
Recommendation: fix now

`app/src/modules/exploration/View.svelte:193` applies `stop: false` to every badge Tip. These Tips contain information absent from the badge's name, such as the 500 k cr threshold and the uncertainty behind “First footfall likely”. DESIGN.md's Tip access rule exempts only repeated marks whose explanatory words are already their accessible name. It does not exempt these explanations.

In Chromium, every moon badge had `tabIndex === -1`; focusing the selected row opened no Tip, the row had no `aria-describedby`, and Tab went directly to OPEN IN BIOLOGY. Attaching a description to an unfocusable badge does not give a keyboard-only user a way to open its Tip. This also removes the previous keyboard path to the Worth mapping explanation, as the Report acknowledges. Exposing the same explanation in the guide is not equivalent to reaching it on this page.

Provide a keyboard path to the nonredundant badge explanations. This can preserve the tree's single row tab stop by exposing the relevant explanation from the focused row or its detail panel; it need not add a tab stop to every repeated badge.

## Finding 2: Stack the panel before the tree's badges have to wrap
Recommendation: fix now

`app/src/modules/exploration/View.svelte:133` declares that the tree fits beside the panel at 800 px, although the Report measures its natural width at about 947 px. The `.tags` rule at line 305 then wraps the badges to compensate. At the supported dock width, 1659 x 900 at 100 percent Text size with the route followed and A 4 a selected, I measured a 350 px by 52 px tag cell: the first five badges started at y=443.58 and “Landed 2 times” at y=467.58, while the panel remained beside the tree. This reproduces the Report's stated trade-off.

That contradicts DESIGN.md's Aligned Columns Rule, which explicitly treats a wrapped fact as a grouping problem. The audit checks do not detect wrapping, so their empty lists do not establish compliance with that rule. The item's required badge list does not require keeping the panel alongside at 1659 px: it requires that at 1920 and 2560, where the preview's full row fits.

Let the space required by the tree determine when the panel stacks, or regroup the facts so they remain aligned without wrapping. Verify the dock width and a longer supported body class as well as the required capture sizes.

## Finding 3: Clear the stored selection when the current system changes
Recommendation: defer

`app/src/modules/exploration/View.svelte:63` retains `picked` for the lifetime of the page. Line 64 merely makes `chosen` undefined when that body is absent; no code clears the stored name on a jump. I selected A 4 a, supplied the empty Next system through the transport, and then restored the original system reply. The panel disappeared in Next, but returned with A 4 a already selected, without another selection action.

This is a rare revisit case, so it need not block the tree redesign, but the Report's claim that selection “drops on a jump” does not hold. Reset selection when the system identity changes while retaining it across polls of the same system, or document explicitly that revisiting a system restores the remembered selection.
