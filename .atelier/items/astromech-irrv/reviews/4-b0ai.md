---
schema: atelier.review
schema_version: 3
round: 4
commit: 3ac1237666acb56caff22a08917d67130d1f9baa
reviewers:
- thread:delegated-task:command%3Amcp%3Ae478f112-2c4c-44fd-a8da-92a33922b225%3Adelegate-task%3Aa349fa44-b574-4ca3-9417-8a53e2b0562d
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T11:23:55.441531889Z
---

## Summary

Complexity: high. The complete change adds a shared reactive plot setup, coordinates asynchronous reads and plotting between the title and header, and introduces three UI primitives.
Value: high. The Route page gains the agreed plotting workflow while sharing its settings with the header.
Risk of merging: medium. Regressions affect the visible plotting workflow, but the change is reversible and changes no stored format or core plotting algorithm.
The value warrants the shared setup. The merge preserves the approved plotting work and the timeline from main; I found no problems requiring changes.

Reviewed `main...3ac1237666acb56caff22a08917d67130d1f9baa` and the changes since the approved round 3 commit, `3d331f695d05acb36b1d02bfb0c24c6db6f2a74c`. Checked each Done when line and the Report against the title, shared setup, header integration, dropdown/menu/dialog primitives, import and clear actions, preview handlers, tests and guide. In particular, checked the merged View retains the timeline and its action/error handling, removes the superseded import/help/clear blocks, and places the title's help and menu beside its controls.

Ran `pnpm lint`, `pnpm check`, `pnpm test` (627 tests in 84 files), `pnpm build`, `python3 scripts/check-guide.py`, `python3 scripts/check-dist.py app/dist` and `git diff --check`; all passed. The default browser-test port was available. The browser suite covers plotting/replotting, busy/error/retry states, download and engine changes, range changes, shared setup, dismissed autocomplete replies, menu/open/import/clear actions, and the merged timeline's copy, Make next and parked-route behavior.

Exercised the full Vite preview in headless Chromium at 1100 x 700 and 2560 x 1440 with the recorded timeline beneath the new title. Inspected captures of the combined page, open Worked out dropdown, actual header panel and import dialog. The title's help and menu follow its controls; the dropdowns remain usable over the timeline. Independently changed Plotter in the header and verified both locations agree. Real Up/Down keys change Efficiency and retain focus in both locations; Escape closes the picker. The header has no horizontal overflow at either size, and its content remains accessible in its scrolling panel. The native import dialog keeps focus inside on Ctrl+K and closes on Escape. No page errors occurred. Captures include `/tmp/irrv-r4-title-1100.png`, `/tmp/irrv-r4-worked-1100.png`, `/tmp/irrv-r4-header-1100.png`, `/tmp/irrv-r4-import-1100.png` and their 2560-width counterparts.

Did not exercise Windows/Tauri, Elite itself, live Spansh requests or a real neutron-file download. No Rust code differs from main in this item.
