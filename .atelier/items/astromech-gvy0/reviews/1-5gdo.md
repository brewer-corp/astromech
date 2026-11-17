---
schema: atelier.review
schema_version: 3
round: 1
commit: 26372c6c3e0cd3a15c38a5793d6e1572c7d69643
reviewers:
- thread:delegated-task:command%3Amcp%3Aabc62642-9c85-452a-b67b-a35aa26f98df%3Adelegate-task%3A4a5204bf-d2d7-4a1e-8ca7-1486388f0bd3
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T14:20:20.393104714Z
---

## Summary

Complexity: medium. Several pages adopt shared layouts and type roles; Stored modules adds grouping and selection, and shared controls and lint configuration change.

Value: high. This completes the remaining page conversions and supplies the mission's final sizing evidence.

Risk of merging it: medium. The changes affect visible UI and shared controls, but failures are observable and the change is reversible.

The value justifies the implementation's size. Most of the diff is evidence; no substantially smaller implementation delivers the page conversions and final audit.

Reviewed `main...26372c6c3e0cd3a15c38a5793d6e1572c7d69643`, the Done when lines, DESIGN.md, the guide changes, the layout components, and the stored-module data contract. Independently passed `pnpm check`, `pnpm lint`, all 915 tests, all seven journeys using the existing main-checkout core binary, and `python3 scripts/check-guide.py`. Resolved ESLint configuration confirms both sizing rules on 52 module files, neither on 13, and no partial enrolments. The token exemptions contain only the stated Controls-related files.

Checked the two capture plans against their reports and files: all 60 remaining-page and 120 final captures are present, with the required sizes and Text size steps. Report findings match the stated ellipsis exceptions; the final evidence line records the lint state and capture commit. Inspected representative saved captures for Stored, Places home, Mining, Data sharing and Voice tester. Independently ran the audit's measurement function in headless Chromium against the converted pages: all six at 1100 x 700, and Data sharing, Diagnostics and Voice tester also at 1920 x 1080 and 2560 x 1440 with the prescribed Text sizes. Scrolled EDSM and Diagnostics' logs into view. Those checks produced no sideways, clipped, overlaps, small, contrast or far entries. Injecting a keyring failure at the transport boundary reproduced Finding 2.

The 16 px button padding is justified. Using the shared Board tile sizing for Places home, with the documented DESIGN.md update, is a reasonable simplification. The Data sharing exception remains explicitly unresolved, as Finding 1 explains.

Did not independently rerun all 180 saved captures or the 4K capture run, exercise the actual Windows shell, or capture the fixture-limited states listed in the Report. Their omissions are disclosed as the item requires. The journeys exercise the real core through the browser bridge, using the prebuilt binary rather than a fresh Rust build; this branch changes no Rust.

## Finding 1: Settle the Data sharing disclosure exception before marking the requirement complete
Recommendation: fix now

`app/src/modules/datasharing/View.svelte:72` retains the printed “What gets sent” explanation and list, and `app/src/modules/datasharing/Edsm.svelte:130` retains its explanatory paragraph. The fifth Done when line explicitly requires printed explanations to move into Tips, yet it is checked complete. The Report itself says retaining these blocks is a deviation requiring the lead to obtain Sumeet's decision, and that the requested `needs-human` follow-up has not been filed. There is no recorded resolution in this change or the mission notes.

Keeping the disclosure visible has a sound rationale, so do not hide it merely to clear this finding. Return this scope question to the lead and record its resolution: either the accepted requirement explicitly retains the disclosures, or the implementation follows the agreed alternative and updates its evidence. A report that asks for a future decision does not establish that this acceptance condition is complete.

## Finding 2: Keep the keyring error's marker beside its text
Recommendation: defer

`app/src/modules/datasharing/Edsm.svelte:157` adds `.failed::before`, but the keyring error at line 103 already has `class="warn failed"` and its child receives another marker from `dd.warn > :first-child::before` at line 153. With `edsm_status.key_error` set to a fault, Chromium computes `"! "` for both pseudo-elements. Since the `dd` uses `justify-content: space-between`, one isolated exclamation mark appears at the left of the value column and the actual error, with another mark, moves to the far right. The saved plans do not include this fault.

Scope the new paragraph-error prefix so it does not add a second flex item to the keyring row, or make that row use one shared error treatment. This is a minor presentation regression, so it can be deferred; the keyring message remains readable.
