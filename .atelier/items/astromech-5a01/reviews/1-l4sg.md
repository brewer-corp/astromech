---
schema: atelier.review
schema_version: 3
round: 1
commit: 44c0636cd2bbf09c73898fb340cd0304f0740902
reviewers:
- thread:delegated-task:command%3Amcp%3Aee52845e-d5a7-466d-9986-172eca7785bd%3Adelegate-task%3A996179ab-42a3-42b1-82bb-1fdad61aa911
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T09:11:26.592636696Z
---

## Summary

Complexity: medium. The change redistributes one page across three views and changes the shell, manifests, guide and navigation test, without adding a backend contract.

Value: high. The requested navigation split gives ships and on-foot features the homes the approved design calls for.

Risk of merging it: medium. It changes navigation and displayed inventory and engineer information used by commanders, but can be reverted without a data migration.

The value warrants the size. The move and reuse of existing components are appropriate; a materially smaller change would not satisfy the requested split.

Reviewed `git diff main...44c0636cd2bbf09c73898fb340cd0304f0740902`, the previous On foot view, the destination views, manifests, palette collection, hash routing, token exemptions, tab-row detection, Presets semantics and the changed guide. Compared the engineer display with both Rust implementations and their catalogues. Inspected every committed before/after capture and the narrow header capture. Ran the branch in headless Chromium to reproduce the inventory failure, inspect the remaining referral instructions and confirm that Ships actually loads. Independently bisected header stacking at all six Text size steps with the font loaded; the thresholds are approximately the documented 95.5 rem (small variations with the active tab and rounding).

Validation passed: `pnpm lint`, `pnpm check` (zero errors or warnings), `pnpm build`, `pnpm test` (86 files, 723 tests), `python3 scripts/check-dist.py` and `python3 scripts/check-guide.py` (22 pages, zero problems). The route cases and Two Tab Rows cases pass. The Suits diff preserves the picker, saved picks, shopping list and bartender trade. The inventory tiles preserve the locker/backpack/mission/barter/sale values when the read succeeds. The findings below prevent the claims that nothing was lost and every moved page was captured from holding.

Did not run Rust builds/tests because no Rust changed, or the native Windows/Tauri window and live game. Browser verification used the recorded preview data and its explicit failure scenario. Changed only this review before recording the verdict.

## Finding 1: Keep the Raw grid out of On foot's loading and failure states
Recommendation: fix now

`app/src/modules/materials/View.svelte:192` enters the on-foot branch only when both `onFoot` and `foot` are true. When `odyssey_view` has not answered or its first read fails, `foot` remains null and rendering falls through to `{:else if kind}` at line 221. `kind` falls back to the first ship-material kind at line 61, so the page shows Raw's grid under the selected On foot choice.

Reproduced at `/?fail=odyssey_view#/materials?kind=onfoot`: On foot and Held only remain pressed, the header reports the inventory failure, and the board contains Carbon, Vanadium, Niobium and the other Raw materials. Clicking one navigates out of On foot. This also flashes Raw during a slow first on-foot read after the ship inventory has arrived. The existing failure tests check the message but do not check the displayed inventory.

Branch on `onFoot` independently of whether its data has arrived, and render its tiles only inside that branch once `foot` exists. Add a case with only `odyssey_view` failing or pending and `materials_inventory` succeeding, asserting that ship-material tiles never appear in the on-foot view.

## Finding 2: Preserve referral tasks that the unlock tree does not yet display
Recommendation: fix now

`app/src/modules/unlocks/View.svelte:125-132` moves the mod list and Colonia flag but drops the old engineer card's `refers` and `referral_task` information. The Report says the referred engineer's card already shows each task. That is only true once the introducing engineer is unlocked: `crates/astromech-engineering/src/unlocks.rs:144-151` otherwise substitutes “Unlock <engineer> first.” The old On foot card displayed its referral task regardless of unlock status.

For example, the old Hero Ferrari card (also visible in `docs/design/tabs-on-foot-engineers-2560-before.png`) says “Refers Wellington Beck: Provide 5 Settlement Defence Plans.” In the new preview, Hero's card shows his invitation requirement and Wellington's says only “Unlock Hero Ferrari first.” “Settlement Defence Plans” appears nowhere on the page. The same loss occurs for later referral tasks such as Kit Fowler's 5 Surveillance Equipment. Commanders can no longer look ahead and collect those items, although the item requires the parts to move whole.

Keep these referral requirements available on Engineers even before the introducing engineer is unlocked, using the already loaded Odyssey engineer information or extending the tree's displayed requirements. Preserve the useful existing unlock guidance, and cover a referral whose introducing engineer is not yet unlocked.

## Finding 3: Replace the blank Ships captures and capture Stored modules
Recommendation: fix now

`docs/design/tabs-ships-2560-before.png` and `docs/design/tabs-ships-2560-after.png` contain the frame and tabs but an entirely blank page body. They do not show the Ships page, its rail or fleet. There is also no 2560 × 1440 capture of Stored modules, which moves from Engineering to Ships. The last Done when line requires captures of each moved page, so the committed set does not yet meet it.

The Ships page does load in the branch: waiting for `main nav` yields the ship rail and Heavy Lifter's details. Regenerate the before/after Ships images after the actual page content and web font have loaded, and add a populated Stored modules capture under its new top tab. Wait on those observable conditions rather than elapsed time.
