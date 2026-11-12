---
schema: atelier.review
schema_version: 3
round: 2
commit: e3354c8954656c9c231e48f3fc76dce1eb1b2c5c
reviewers:
- thread:delegated-task:command%3Amcp%3Aadc1aaff-29c4-454f-8038-0b4c4e7be7ba%3Adelegate-task%3A793380dd-5131-42d1-8539-cff9438bfe73
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T10:07:09.247021012Z
---

## Summary

Complexity: high. The change introduces a selection/context contract between SearchKit and ListDetail, four shared layouts, a lint plugin and four page conversions.

Value: high. These layouts are required for the revision 4 window design and its migration away from page width caps.

Risk of merging: medium. Layout and interaction failures reach users, but are visible and reversible.

The value warrants the scope. The small layout wrappers and shared split avoid duplicating the same sizing behavior across pages; I found no smaller approach that delivers most of the outcome.

I read the complete diff against main, the round 2 changes, DESIGN.md and CONTRIBUTING.md, and checked each Done when line against the implementation and captures. I checked that all 24 before/after captures exist at their stated dimensions and inspected representative captures. In headless Chromium against this checkout's preview, I exercised Materials (raw, manufactured and encoded), Sell, Outfitting, Commodities with a result, and General settings at 1100 x 700, 1920 x 1080, 2560 x 1440 and 3840 x 2160. These pages had no horizontal overflow. I also checked the 1410 x 900 panel threshold, panel action failures and recovery controls, and selected-row hover and keyboard-focus colors.

`pnpm check` passes with zero errors and warnings; `pnpm lint`, `pnpm build`, `python3 scripts/check-guide.py` and `python3 scripts/check-dist.py` pass. `pnpm test` passes: 79 files, 590 tests. Its first run stopped on a port collision at 63315; the subsequent run completed normally without a source change.

The three carried findings are fixed. One additional fix-now finding concerns the first selection in a stacked ListDetail. I did not exercise the Windows Tauri window, live core/network data or native microphone calibration; runtime checks used the browser preview.

## Finding 1: Show panel-action feedback beside the panel's buttons
Recommendation: fix now
Was: 1-3fjf.1
Response: fixed in a9508970. A panel action's note, failure and TRY AGAIN now show under the panel's buttons; places/Market.browser.test.ts finds the note inside the panel, and failed before the fix.
Now: fixed

Verified in SearchKit's action handlers and both render locations. In Chromium at 1100 x 700 with `?core=ready&fail=route_set#/places/sell/painite`, a panel Set destination failure and TRY AGAIN render inside Details beside its buttons. The selected-panel browser test passes.

## Finding 2: Keep selected-row action icons and keyboard focus visible
Recommendation: fix now
Was: 1-3fjf.2
Response: fixed in a9508970. A hovered selected row's icons are On Orange (rgb(12, 13, 15) in Chromium), and the row's icon buttons focus with an On Orange outline inset 2 px.
Now: fixed

Verified in Chromium: hovered selected-row action icons compute to `rgb(12, 13, 15)`. Keyboard selection followed by Tab to Add stop produces a solid outline in that color with `outline-offset: -2px`. The selected CSS now wins over row hover.

## Finding 3: Include the panel's padding and border in the stacking threshold
Recommendation: fix now
Was: 1-3fjf.3
Response: fixed in a9508970. stacks() uses the panel's full track, PANEL_TRACK = 320 + 24 + 1; the new lib/ui/layout/layout.test.ts failed before the fix. On Sell at 1410 x 900 the panel stacks and the list keeps all nine columns.
Now: fixed

Verified that stacks() budgets PANEL_TRACK = 345, matching the panel's content, padding and border. The threshold test passes. In Chromium at 1410 x 900, selecting a Sell result stacks the panel and retains all nine column heads.


## Finding 4: Scroll the first selected result's panel into view after it mounts
Recommendation: fix now

`app/src/lib/ui/layout/Split.svelte:31-35` reads `aside` inside `untrack`. On the first selection in a ListDetail, that effect runs before the conditional aside's `bind:this` has supplied its element. The optional call does nothing, and the later binding cannot rerun the effect because it is untracked. A later selection works because the aside already exists.

Reproduced in Chromium at 1100 x 700 on `/?core=ready#/places/sell/painite`: load the finder and click the first result's name. The row becomes selected, but main's scrollTop remains 0 and Details starts at y=1009, completely below the main viewport (y=188 through 700). Instrumenting scrollIntoView records no call. Click the second result's name and it does call scrollIntoView: scrollTop becomes 737 and the panel appears at y=272. The first selection therefore fails to reveal the details that the new interaction is meant to show, contrary to the Report's claim that a new stacked selection scrolls the panel into view.

Wait for the new panel to be rendered and bound before scrolling it, keeping the once-per-selection behavior. Verify the transition from no selection to the first selection at the minimum window size, as well as changing an existing selection.
