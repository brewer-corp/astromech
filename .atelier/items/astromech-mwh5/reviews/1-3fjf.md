---
schema: atelier.review
schema_version: 3
round: 1
commit: 1f2526df467882155a0045412c71083b52123a58
reviewers:
- thread:delegated-task:command%3Amcp%3Aadc1aaff-29c4-454f-8038-0b4c4e7be7ba%3Adelegate-task%3A4b5a0b9e-194e-4277-943d-0d949c2c9756
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T09:58:59.329258677Z
---

## Summary

Complexity: medium. Four shared layouts, a selection context and a two-part ESLint rule change several page owners, but the implementation fits in one review.
Value: high. Shared sizing and adjacent details are needed for this mission's window layouts.
Risk of merging: medium. Layout and action-feedback failures reach users, are visible and can be reversed.

The value warrants the scope. I found no substantially smaller change that delivers the four layouts and their enforcement.

I reviewed `main...1f2526df467882155a0045412c71083b52123a58` against each Done when line and DESIGN.md revision 4. I read the layouts, SearchKit selection and actions, converted pages, lint implementation, enrollment and tests. I inspected representative before-and-after captures and verified all 24 PNGs have their promised dimensions.

I independently ran `pnpm check` (0 errors, 0 warnings), `pnpm lint`, `pnpm test` (78 files, 589 tests) and `python3 scripts/check-guide.py` (0 problems). All passed. In headless Chromium on this branch's Vite preview, I checked Materials (Manufactured and Encoded), Sell, Outfitting, Services, Commodities with a route result, and General settings at widths from 1100 through 3840. The checked pages did not overflow horizontally at 1100 x 700. I exercised selection, resizing, row hover, keyboard focus and a failed destination action. The minimum-window and large-window layout claims hold for these preview inputs, subject to the findings below.

I did not run the Windows/Tauri application or use live game/network data. The browser checks use the repository's recorded preview inputs. I changed no implementation file.

## Finding 1: Show panel-action feedback beside the panel's buttons
Recommendation: fix now

`app/src/lib/ui/SearchKit.svelte:296-299` adds Set destination and Add stop to the detail panel, but their handlers write `goNote`, whose only renderer remains under the result list at line 372. The panel contains no confirmation, error, retry or Resume control. When the layout stacks and brings the panel into view, that feedback can be clipped above the scroll viewport.

Reproduced at 1100 x 700 with `/?core=ready&fail=route_set#/places/sell/painite`: select the first result and click the panel's Set destination. The panel and its buttons remain visible, while the failure and Try again are at y=163-197 above a main viewport beginning at y=188; their text is not readable. The screen looks as though the button did nothing. Successful replacement notices and Resume use the same misplaced renderer.

Render feedback from a panel action adjacent to its buttons, including failures and recovery actions. Preserve the existing feedback for actions initiated from the list.

## Finding 2: Keep selected-row action icons and keyboard focus visible
Recommendation: fix now

At `app/src/lib/ui/SearchKit.svelte:423-426`, the new selected-state icon rule loses to the more specific existing `.result:hover .acts :global(.ibtn)` rule at line 401. Hovering a selected row therefore paints both action icons Orange Lit (`rgb(255, 160, 74)`) on its solid Orange background (`rgb(250, 110, 0)`), rather than On Orange. This occurs immediately after clicking a row name and makes the actions difficult to distinguish precisely when the pointer approaches them.

I reproduced this in Chromium on the Sell finder and checked the computed colors, both while hovering the name and while hovering Add stop. Keyboard navigation also leaves the selected row's Add stop focus outline Orange on the same Orange fill: after selecting its name, move the pointer away and press Tab twice. Only the name button has a selected-state focus override; the icon buttons retain their Orange outline with a 2 px outward offset.

Make the selected styling win for hovered action icons, and give the selected row's icon buttons the visible On Orange focus treatment required by DESIGN.md States.

## Finding 3: Include the panel's padding and border in the stacking threshold
Recommendation: fix now

`app/src/lib/ui/layout/layout.ts:21-22` compares available width against `min + 48 + 320`, but `Split.svelte:50-52` reserves `320 + 24 + 1` px for the actual panel track. The missing 25 px lets the layout stay side by side even when its content is narrower than the minimum it was given.

Reproduced on Sell at 1410 x 900: before selection the 1320 px split shows all nine column heads. Selecting the first result keeps the panel beside the list, allocates 345 px to the panel and 48 px to the gap, and leaves only 927 px for a list whose `fits` minimum is 952 px. SearchKit switches to its three-column compact mode, hiding Demand, Distance, Jumps, Pad, Arrival and Updated. At this width the promised fallback is to place the panel below the content instead.

Make the stacking calculation match the panel's complete CSS track so a side panel never forces content below the supplied minimum.
