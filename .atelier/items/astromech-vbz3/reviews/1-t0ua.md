---
schema: atelier.review
schema_version: 3
round: 1
commit: 617577aa7054e96455790460d290c6ac38929a1c
reviewers:
- thread:delegated-task:command%3Amcp%3A86e3ce16-4f5d-4af6-9fb7-eb065b16ff26%3Adelegate-task%3Ab8aa0376-2327-4b76-adfe-78c0f5b3aa07
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T10:17:19.144539309Z
---

## Summary

Complexity: low. One selector overrides the shared nowrap rule for unlock steps, and the now-redundant step Tip is removed.

Value: high. The row now shows the complete instruction the commander needs to unlock the engineer.

Risk of merging: low. This is a local presentation change with no state or data changes, and it is easy to revert.

The value warrants the change; there is no smaller useful correction. No findings. Both Done when lines are met.

Reviewed `git diff main...617577aa7054e96455790460d290c6ac38929a1c`, the complete Search component and its shared `who` snippet, DESIGN.md's alignment and action-placement rules, CONTRIBUTING.md's visual-change rules, and the committed before/after captures at 1100x700 and 2560x1440. The item explicitly permits the unlock instruction to wrap, so the general truncation rule does not block this change. The other cells keep their existing truncation, and other uses of Tip remain intact.

Ran scratch copies of `scripts/browser/audit.mjs` and `docs/design/increment2/plan.json` from astromech-8e22 against this branch's `pnpm dev` at http://127.0.0.1:5178, with the browser node_modules symlinked and GIT_DIR set to this worktree. The unmodified audit with `--only engineers-laser` reports zero clipped text, overlaps, small text, contrast problems, sideways overflow and far actions at all four checkpoints: 1100x700 and 1920x1080 at 100%, 2560x1440 at 145%, and 3840x2160 at 180%. A separate scratch copy restoring only the old nowrap styling in the browser reproduces exactly one clipped step at each size, confirming the reported cause.

Measured row geometry at all four sizes: every group's left edge equals its header's left edge, and the actions remain directly after Where with the existing 1.5rem gap. Mel Brandon's complete instruction occupies two lines without horizontal overflow. The existing row height accommodates the extra text line; the row itself does not need to grow. The committed captures and fresh audit captures agree on the visible correction.

`pnpm lint`, `pnpm check` (zero errors and warnings), `pnpm build`, and `python3 scripts/check-dist.py` pass. No tests were added, as required for this visual change. Validation used headless Chrome and the browser preview's recorded data; Windows WebView2 and the live game were not exercised. Scratch scripts and audit captures stayed outside the checkout.
