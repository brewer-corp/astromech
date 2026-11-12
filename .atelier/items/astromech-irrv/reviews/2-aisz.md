---
schema: atelier.review
schema_version: 3
round: 2
commit: ee933209c55edece37441cbdbc512639c2c9b89e
reviewers:
- thread:delegated-task:command%3Amcp%3Ae478f112-2c4c-44fd-a8da-92a33922b225%3Adelegate-task%3Aeb020524-bdd8-48fa-9857-cb0ea4f89d74
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T11:13:12.323669258Z
---

## Summary

Complexity: high. A shared reactive plot setup coordinates the page and header, asynchronous reads and plotting, alongside three new UI primitives.
Value: high. The change implements the agreed Route-page plotting workflow and shares its settings with the header.
Risk of merging: medium. Regressions affect the visible plotting workflow, but the change is reversible and changes no stored format or core plotting algorithm.
The value warrants the shared setup. Both previous findings are fixed; the keyboard regression below should be fixed before merge.

Reviewed `main...ee933209c55edece37441cbdbc512639c2c9b89e` and the round 2 diff, checking each Done when line and the Report against the title, shared setup, header integration, import dialog, UI primitives, preview, tests and guide. Ran `pnpm lint`, `pnpm check`, `pnpm test` (620 tests in 83 files), `pnpm build`, `python3 scripts/check-guide.py` and `python3 scripts/check-dist.py app/dist`; all passed. The default browser-test port was available.

Exercised the full Vite preview in headless Chromium at 1100 x 700 and 2560 x 1440, inspecting captures of the title, import dialog and the actual header's open Worked out dropdown. Rechecked pending autocomplete replies at the transport boundary, holding and then releasing the preview's original answers after Escape, clearing, blur (after its click grace period) and submission. Checked Efficiency with real keyboard input in both plotting locations. The passing browser suite covers plotting/replotting, shared settings, busy/error/retry, download/engine changes, range changes and menu/import/clear actions. Did not exercise Windows/Tauri, Elite itself, live Spansh requests or a real neutron-file download.

## Finding 1: Keep the header's plot dropdown visible inside its scrolling panel
Recommendation: fix now
Was: 1-rw9s.1
Response: fixed in abce8aae
Now: fixed

`Plot` now passes `inline` through `PlotSetup` to `Picker`, which opens the options and extra controls in the panel's flow. At both target window sizes, the header has no horizontal overflow and the dropdown, Download and Efficiency fit inside it. Captures: `/tmp/irrv-r2-header-open.png` and `/tmp/irrv-r2-header-large.png`.

## Finding 2: Invalidate pending destination suggestions when dismissing the edit
Recommendation: fix now
Was: 1-rw9s.2
Response: fixed in abce8aae
Now: fixed

`dropHits` clears the debounce and advances the request generation on every edit, including clearing, and on dismissal, pick and submission. The new deferred-reply test passes. Independently releasing held preview replies after Escape, clearing, blur and Enter left suggestions closed.

## Finding 3: Preserve arrow-key editing inside the Efficiency field
Recommendation: fix now

`app/src/lib/ui/Picker.svelte:61-70` handles ArrowUp and ArrowDown from every descendant while the picker is open, including the numeric Efficiency input in its `extra` content. It moves focus to an engine option and prevents the input's native stepping. This regresses the previous plain numeric field.

Reproduced in the full preview at 1100 x 700: open Worked out, focus Efficiency at 60, and press ArrowUp. The value stays 60 and focus moves to the By Spansh option; a native number input becomes 61 and retains focus. ArrowDown has the same problem in the header panel. The mouse and text-entry paths in the passing suite do not cover this.

Restrict option-navigation keys to the trigger/options so controls in `extra` retain their native keyboard behavior, while preserving Escape to close. Add a browser regression case that presses an arrow in Efficiency and verifies its value and focus.
