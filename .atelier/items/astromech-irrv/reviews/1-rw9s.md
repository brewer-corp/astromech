---
schema: atelier.review
schema_version: 3
round: 1
commit: bf43815e91a2c16c6aec2195abeb2fdca8529558
reviewers:
- thread:delegated-task:command%3Amcp%3Ae478f112-2c4c-44fd-a8da-92a33922b225%3Adelegate-task%3A6247a2bf-5d95-48be-a36e-289d12fdfd44
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T11:04:56.084953629Z
---

## Summary

Complexity: high. A new shared reactive setup coordinates the page, header, asynchronous reads and plotting, alongside three new UI primitives.
Value: high. This implements the agreed Route-page plotting workflow and keeps its settings shared with the header.
Risk of merging: medium. Regressions affect the visible plotting workflow, but the change is reversible and changes no stored format or core plotting algorithm.
The shared setup is justified by the workflow; the two problems below need fixing before merge.

Reviewed `main...bf43815e91a2c16c6aec2195abeb2fdca8529558`, including every Done when line, the title, shared setup, header integration, import dialog, dropdown/menu primitives, preview changes, tests and guide. Ran `pnpm lint`, `pnpm check`, `pnpm test` (619 tests in 83 files), `pnpm build`, `python3 scripts/check-guide.py` and `python3 scripts/check-dist.py app/dist`; all passed. The default browser-test port was available.

Exercised the full Vite preview in headless Chromium at 1100 x 700 and 2560 x 1440 with the app's height scale, inspecting captures of the title, dropdown and import dialog. Also held a real preview autocomplete reply at the transport boundary to check Escape deterministically. The title's ordinary plotting/replot, engine/download, busy/error/retry, shared-settings and menu paths are covered by the passing browser tests. The header integration has the clipping regression below, which component-only rendering of Plot does not expose. Did not test Windows/Tauri, Elite itself, a real neutron-file download or live Spansh requests.

## Finding 1: Keep the header's plot dropdown visible inside its scrolling panel
Recommendation: fix now

`app/src/lib/ui/Picker.svelte:111` positions the new dropdown absolutely below its cell with a viewport-based maximum width. In the header, PlotSetup renders inside `PlotBar.svelte`'s `.panel`, which has `overflow: auto`. The dropdown neither grows that panel nor fits its remaining width, so the panel clips its right and bottom edges. This is a regression from the previous in-flow controls.

Reproduce in the full preview at 1100 x 700: enter Colonia in Find or plot, press Enter, then open Worked out. The panel ends at x=1015.5 and y=303.7, while the dropdown extends to x=1185.5 and y=461.5. Download and Efficiency are initially hidden below the panel; the file explanation is cut off horizontally. The same clipping occurs at 2560 x 1440. The dropdown requires scrolling in both directions inside the small panel to reach its contents. Capture: `/tmp/irrv-header-open.png`.

Make Picker and the header panel work together so the open options, file controls and efficiency field are visible and usable without this clipping. Verify the actual PlotBar container at both target sizes, rather than only rendering Plot by itself.

## Finding 2: Invalidate pending destination suggestions when dismissing the edit
Recommendation: fix now

`app/src/modules/route/Title.svelte:112-118` only rejects a reply when `asked` changes, but Escape at lines 146-151 neither changes that counter nor cancels the pending timer. The early return for a short/empty query at lines 107-110 also leaves the previous request current. Consequently a dismissed or cleared edit can reopen its stale suggestions when Spansh answers.

Reproduced with the existing exact-route preview: the destination starts as Maia; type Col, hold the `galaxy_systems` reply, then press Escape. The field correctly returns to Maia. Release the unchanged preview reply and the field now has `aria-expanded="true"` with a visible Colonia suggestion under Maia. No new edit occurred. Blurring or submitting before a pending lookup completes can likewise be followed by that reply reopening the list. Capture: `/tmp/irrv-autocomplete-escape.png`.

Cancel the debounce and invalidate outstanding replies on dismissal/submission, and advance the request generation for every edit, including clearing the input. Add a deterministic deferred-reply case proving that Escape and clearing leave suggestions closed after the old reply arrives.
