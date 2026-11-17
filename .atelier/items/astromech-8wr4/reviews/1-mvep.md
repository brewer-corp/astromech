---
schema: atelier.review
schema_version: 3
round: 1
commit: f1b588492bbb139a3c4a466a472edfbf9f56f077
reviewers:
- thread:delegated-task:command%3Amcp%3Af547e1d1-af7b-4267-b2d0-b889d0455ab6%3Adelegate-task%3Ae24a137c-42ba-4a10-a90a-d85d4d441525
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T11:19:55.919086340Z
---

## Summary

Complexity: medium. The shared ShipPlan owner and the new loadout and module panel span several components, using the existing core commands.

Value: high. Reading the fleet's fitted modules and planning their engineering are central Ships workflows.

Risk of merging: medium. The change reaches persistent plan editing and arrival shopping lists, but keeps the core contracts and is reversible.

The value warrants the change. I found no substantially smaller implementation that delivers the loadout, module panel and preserved Plan view. Two display and error-state regressions need fixing before merge.

I reviewed the full diff against main at f1b588492bbb139a3c4a466a472edfbf9f56f077, the Done when lines, plan persistence and evaluation, slot grouping, engineer search matching, palette routes, retained stats and Plan actions, guide edits and lint enrollment. I checked the recorded fixtures: stats are on ship 1 and the sample plan is on ship 7, as the Report corrects. Keeping military slots under Optional and showing planned hexagons for an unengineered target are reasonable. The supplied captures support keeping one column at the standard sizes and two where both groups fit.

Validation: pnpm lint, pnpm check, pnpm test (90 files, 780 tests), scripts/check-guide.py (22 pages) and all seven journeys passed. The first typecheck was killed with exit 137 while the tests ran; it passed separately with zero errors or warnings. The journeys used the existing debug core in this task's scratch directory through ASTROMECH_CORE, not a fresh core build; this change modifies no core code. I did not check Windows/WebView2 or live game journals.

I inspected the committed captures and report, then independently ran the audit's measure function in headless Chromium over all 19 entries in plan.json at their stated sizes and text steps. Each produced empty sideways, clipped, overlaps, small, far and contrast lists, and the measured Open in gap was 24 layout px. However, the no-fleet entry and its committed PNG show onboarding, not Ships. I separately skipped setup, navigated back to Ships, and exercised the actual empty-fleet state and a subsequent failed poll. I also reproduced the blueprint/grade mismatch below through the real page and dropdowns. No application code was changed during review.

## Finding 1: Keep the fitted grade consistent with the fitted blueprint in a loadout row
Recommendation: fix now

In app/src/modules/engineering/Loadout.svelte:46–53, grade() sets now to zero whenever the target blueprint differs from the fitted blueprint. The row still prints m.engineering.blueprint_name and experimental_name at lines 71–72, so it combines the planned blueprint's starting progress with the fitted blueprint's name.

Reproduction: open #/engineering/1, select the Power distributor, and choose Charge enhanced (PowerDistributor_HighFrequency). Before the edit its row reports Grade 5 of 5 beside Weapon focused / Super Conduits. Afterward the same fitted text has no reached-grade hexagons and the accessible grade becomes “Grade 0 of 5, grade 5 planned.” The panel correctly still reports Weapon focused, grade 5 at 75%. The saved plan has not changed the fitted module, yet the loadout now misstates its engineering progress.

Keep the fitted grade attached to the fitted blueprint. If a different target needs separate progress, distinguish its blueprint and grade rather than presenting its zero starting grade as the fitted module's grade. Protect this case with a behavior test covering a planned blueprint different from the fitted one.

## Finding 2: Show refresh failures after an empty fleet answer
Recommendation: fix now

In app/src/modules/engineering/View.svelte:209–210, a cached fleet with no ships renders only the empty-state line. The Stale component is inside the nonempty-ship branch at line 254, so fleetErr is never shown when the last successful fleet answer was empty.

I reproduced this at ?core=empty after skipping setup and returning to #/engineering. After adding fail=engineering_fleet and waiting for that poll to reject, the transport returned “preview: engineering_fleet failed on purpose (?fail=)” but main still contained only “No ships in the journal yet.” A real missing or unreadable journal after an empty read is therefore silently presented as a current empty answer. This regresses the previous unconditional Stale line and fails the Done when requirement for a failed fleet poll.

Render the refresh failure for any cached fleet reply, including an empty one, and test an empty success followed by a failed poll. Update the no-fleet audit entry to bypass setup and wait for the Ships empty state; its current screenshot, docs/design/ship-loadout/no-fleet-1100x700.png, captures Find Elite instead and cannot verify this state.
