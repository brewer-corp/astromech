---
schema: atelier.review
schema_version: 3
round: 2
commit: 94e0018b7ab93ee87e6e5ed8475ef6b42708e660
reviewers:
- thread:delegated-task:command%3Amcp%3Acb0f7063-35a6-4af1-b38e-316ee4e03908%3Adelegate-task%3Aef935660-5b06-465c-8635-c9f6d8437c23
- thread:delegated-task:command%3Amcp%3Acb0f7063-35a6-4af1-b38e-316ee4e03908%3Adelegate-task%3Aa5dc59e4-a6e3-4d69-a188-f14387aa8536
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T11:33:30.089578566Z
---

## Summary

Complexity: medium. The change adds loadout and panel selection and a small helper for item-specific saved picks, using the existing transport, recipes and layout components.

Value: high. Showing each item's equipment and plan is the required outcome, and the old blueprint picker remains reachable.

Risk of merging it: medium. It changes persisted local plans and the displayed shopping costs, but does not change game execution or the core's recipe calculations. The previously identified plan-loss and double-counting paths now pass regression checks.

The value warrants the scope. No substantially smaller change would provide the requested loadout detail and per-item planning.

Reviewed `main...94e0018b7ab93ee87e6e5ed8475ef6b42708e660` against every Done when line, and the changes since round 1. Read the page, matching and consolidation helper, tests, sizing enrollment, guide, capture plan and report. Compared the reference mockup and before/after captures, including the narrow stacked panel.

Validation passed: `pnpm check`, `pnpm lint`, `pnpm test` (91 files, 779 tests), `pnpm build`, `python3 scripts/check-dist.py`, `pnpm exec tsc -p journeys`, all seven journeys, `python3 scripts/check-guide.py`, and `git diff --check main...HEAD`. Ran one full test suite at a time. Journeys used the existing debug core in the `t3-1c2d7220-03c6cb5e4a0b1bdb` worktree; verified that worktree is clean and its Rust sources, manifests, lockfile, toolchain and Cargo configuration match this branch. No cold Rust build was needed.

Independently exercised all three original reproductions in Chromium. The page used the recorded Odyssey view and the real core's `odyssey_shopping` on `worlds/preview/odyssey`. Adding grade 3 with an existing tied 1-to-2 upgrade and a loose 1-to-3 upgrade saved one tied pick; the core returned 2,850,000 credits and 7 Carbon Fibre Plating. Tying the lower loose upgrade to a grade-5 plan preserved that single grade-5 pick. Actual browser Back from Spare to Explore closed Spare's panel; selecting Raid after Explore retained the shared Maverick's panel. These checks produced no page errors. The new table cases and hash-navigation browser case also passed in the full suite.

Repeated all twelve states/sizes in `docs/design/suits/plan.json`: 1100 x 700 at 100 percent, 1920 x 1080 at 100, 2560 x 1440 at 145, and 3840 x 2160 at 180. Every capture had empty `sideways`, `clipped`, `overlaps`, `small`, `contrast` and `far` lists, with the sidebar strip at 1100 and docked at the larger sizes. Used a temporary copy of the audit with only repository-root handling adjusted so the plan, captures and report stayed outside the checkout; measurement code was unchanged. Independently spot-measured the panel's Remove and Add to plan, Plan other gear, engineer destination and shopping-column gaps at the three larger sizes; all were below 360 layout px. The captures and tokens support the reported brightness hierarchy.

Loading, empty, initial failure, unknown-recipe and legacy-pick behavior pass the browser suite. Inspected refresh, stale-answer and retry handling in code. The Windows/Tauri window and live game journals were not exercised.

## Finding 1: Adding a higher grade can retain two overlapping upgrades for the same item
Recommendation: fix now
Was: 1-l936.1
Response: fixed in 4e078e97: addPick uses up the matching untied pick and combines it with the item's own upgrade into one pick, with a table case for that combination.
Now: fixed

## Finding 2: Tying a lower upgrade silently reduces the item's existing target
Recommendation: fix now
Was: 1-l936.2
Response: fixed in 4e078e97: combining upgrades keeps the higher target; a table case ties a 1 to 3 pick to an item planned to grade 5 and keeps one 1 to 5 pick.
Now: fixed

## Finding 3: Browser Back changes the loadout but leaves the previous item's panel active
Recommendation: fix now
Was: 1-l936.3
Response: fixed in 4e078e97: the panel is reconciled in an effect on the loadout shown (click, Back, or a new worn loadout), keeping only an item that loadout has or one in no loadout; a browser case goes Back from Spare's Maverick to Explore.
Now: fixed
