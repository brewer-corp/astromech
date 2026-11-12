---
schema: atelier.review
schema_version: 3
round: 4
commit: 2776964d65191373b62ad572d3fc95dd6f9c22d0
reviewers:
- thread:delegated-task:command%3Amcp%3A129e272b-7e6c-4dde-a93c-54841696d631%3Adelegate-task%3A97080e97-72d4-47a8-812f-2e210457aab5
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T10:11:06.980414331Z
---

## Summary

Complexity: medium. One shared disclosure owns open state and focus, with summaries and consumer-specific controls supplied by live-search finders and the explicit Find route page.

Value: medium. These existing finders put results closer to their main input while keeping their constraints visible and editable.

Risk of merging it: medium. Hidden controls make accurate summaries necessary, but failures are visible and the frontend change is reversible.

The value warrants the scope. SearchFilters is the appropriate shared owner; a substantially smaller change would not cover both kinds of consumer and their extra controls.

Reviewed the full diff from main to `2776964d65191373b62ad572d3fc95dd6f9c22d0` against each Done when line and the Report. Also inspected the integrated changes since round 3, including Choice's tooltip contract, Escape handling and Route's container queries. Read SearchFilters, SearchKit, the summary builders, filter persistence, all consumer call sites, the changed tests and guide edits against DESIGN.md and CONTRIBUTING.md. The shared disclosure remains mounted through searches, saves and errors. Mining hotspots and Powerplay supply their extra filters inside it; the main query inputs remain visible. Route updates its summary and pending inputs independently of Find route. No unresolved earlier findings remain.

Validation passed: `pnpm check` (zero errors or warnings), `pnpm lint`, `pnpm exec vitest run --browser.api.port 63457` (80 files, 587 tests), `pnpm build`, `python3 scripts/check-dist.py`, `python3 scripts/check-guide.py` and `git diff --check main...2776964d65191373b62ad572d3fc95dd6f9c22d0`. The alternate browser port avoids the reported worktree collision. The browser suite covers two live filter edits and recorded results, search and save failures, disclosure closing actions, the pending-radius summary and explicit Find route with controls open and closed.

Inspected all three committed Sell captures at 1920 x 1080. Also checked the integrated Sell preview with `scripts/ui` in Chrome at that size: the resting panel has no filter selects; changing the radius to 20 ly and demand to 5,000 t updates the summary and results; selecting a large pad then pressing Escape closes the controls and returns focus to CHANGE. Inspected the resulting capture. The probe browser and Vite server were closed. Changed only this review file. Windows/Tauri, the game and live external services were not checked; no Rust or journey tests were run for this frontend-only change.

No findings. Approved.
