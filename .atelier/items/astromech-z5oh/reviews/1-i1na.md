---
schema: atelier.review
schema_version: 3
round: 1
commit: af338116e8badb9cf3158474b9316e5ede12a1a6
reviewers:
- thread:delegated-task:command%3Amcp%3Ad93aacaa-09c0-4330-95bd-c4fbe9c70fcc%3Adelegate-task%3Af1e8ab9d-de35-491f-bf33-0d2c9bb7602c
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T10:41:45.132228449Z
---

## Summary

Complexity: medium. Shared typography classes and a new Stat component are applied across seven page families, with matching lint and test changes.

Value: high. Making each tile's main value immediately visible is the item's purpose and improves pages used during play.

Risk of merging: low. The changes are presentation changes whose failures are visible and inexpensive to undo.

The scope and shared component are proportionate to the readability goal. Two small fixes remain before the Done when claims hold.

Reviewed main...af338116e8badb9cf3158474b9316e5ede12a1a6 against every Done when line, DESIGN.md's Hierarchy and component specifications, the page-sizing rule and CONTRIBUTING.md. Read every changed source and test file, inspected all sixteen committed before/after PNGs and confirmed their 2560 x 1440 dimensions. Compared Materials with the reference PNG from mockups/glow-up. The role definitions, material tile structure, stat tiles, Places roles and lint enrollment otherwise match their specifications; Places correctly uses Value L under its specific component specification.

Ran pnpm check, pnpm lint, pnpm test (82 files, 612 tests), pnpm build, scripts/check-guide.py and scripts/check-dist.py; all passed. Independently rendered Materials in headless Chrome at 2560 x 1440 and confirmed the counts compute to 28 px / 500 with no compiler overlay. Exercised the two Settings warning states through the transport boundary and inspected their computed typography. Did not run the Windows/Tauri app, connect to the game or rerun the unchanged Rust workspace.

## Finding 1: Give the two conditional Settings warnings their Label role
Recommendation: fix now

In app/src/modules/settings/View.svelte:258 and :270, the update-installation warning and config warning still have only `hint warn-text`. Removing font-size and line-height from `.hint` at :293 leaves these two branches inheriting the page's 15 px / 500 / normal text instead of the Label role used by the other hints. This is an incomplete conversion under the Done when requirement that Settings > General use roles throughout, and it changes the warning typography unintentionally. With `status.update.apply_error` and `status.config_note` supplied through the transport, Chrome reports 15 px / 500 / normal for both, versus 13 px / 400 / 16.9 px for the adjacent `hint type-label` paragraph. Add `type-label` to both conditional warning paragraphs.

## Finding 2: Replace the Materials after capture that contains a compiler overlay
Recommendation: fix now

docs/design/type-roles-materials-after.png contains a large Vite error overlay for exploration/View.svelte's invalid `{@const}` placement. It dims and covers the Materials page, so it is not the required usable after capture and does not show the submitted code's successful render. The selected-material after capture is clean, and the current source passes compilation and renders without the overlay in a fresh browser. Recapture the unselected Materials page at 2560 x 1440 from the submitted source and replace this PNG.
