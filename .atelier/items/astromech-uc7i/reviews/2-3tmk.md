---
schema: atelier.review
schema_version: 3
round: 2
commit: 763bfed0f2bb00c020b0e0e05f8b1111d66ea0f3
reviewers:
- thread:delegated-task:command%3Amcp%3Aee65c5e2-2014-49d7-aa45-41a730a2d1b3%3Adelegate-task%3A0d9ca5ca-8ebb-46f2-b240-01b717a39940
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T08:28:41.492154216Z
---


## Summary

Complexity: high. The change introduces shared route-follow state and changes the frame's layout and navigation contracts across every page.

Value: high. It delivers the requested persistent route while returning the Route page's space to the active page.

Risk of merging: medium. Frame layout and route navigation reach every user, but failures are visible and the change can be reverted.

The value justifies the scope. A substantially smaller change would not deliver the requested docked, collapsed, overlay and empty states.

I reviewed `main...763bfed0f2bb00c020b0e0e05f8b1111d66ea0f3` and the delta since round 1 against the Done when lines, DESIGN.md and CONTRIBUTING.md. I checked the shared poll, sidebar and strip state, navigation, timeline changes, page container queries, sizing exemptions, guide changes and journeys. I inspected the committed 1920 px docked and 1100 px overlay captures. Both carried findings are fixed, but checking the new explicit-open state exposed Finding 3.

Validation passed: `pnpm lint`, `pnpm check`, `pnpm build`, `check-dist.py app/dist`, `check-guide.py`, all 648 tests, `pnpm exec tsc -p journeys`, and all 7 journeys. The journeys used `/home/supadava/projects/astromech/target/debug/astromech-core` through `ASTROMECH_CORE`; this diff changes no Rust, so I did not rebuild the core or rerun Rust tests. In additional Chromium checks I opened the menu for neutron and exact routes at 1920 x 1080 and 1100 x 700, opened and cancelled Import from each, followed cold empty `#/route` links at both widths, and reproduced the explicit-open/clear regression below. I did not exercise the Windows Tauri window or a live game, or manually capture all ten page files at every switching width; the existing browser test checks Sell's dock boundary at every Text size step.

## Finding 1: Keep the neutron route's actions menu inside the window
Recommendation: fix now
Was: 1-l239.1
Response: fixed in dfe596b9
Now: fixed

`Menu.svelte` now opens from the trigger's left edge when right alignment would place it outside the window. Independently checked both plotters at 1920 x 1080 and 1100 x 700: the neutron menu is 204 px wide at x = 40 and x = 33 respectively, and the exact menu is at x = 83 and x = 76. All four menus fit inside the window, and Import can be opened and cancelled. The added browser regression test passes.

## Finding 2: Honor explicit sidebar opens when there is no route at dock width
Recommendation: fix now
Was: 1-l239.2
Response: fixed in 59959ec1
Now: fixed

The new `asked` state lets explicit opens render the empty sidebar instead of the rail. Cold `/?core=ready#/route` links show the empty plotter at both 1920 x 1080 and 1100 x 700 and rewrite the address to `#/controller`. The added browser test also checks folding back to the rail and reopening through the palette's Route entry, and passes. The separate lifecycle regression introduced by this state is Finding 3.

## Finding 3: Return to the rail after clearing an explicitly opened route
Recommendation: fix now

`app/src/modules/route/follow.svelte.ts:83` sets `asked = true` for every docked `show()`, even when a route is already followed. The clearing effect in `app/src/App.svelte:67-69` only depends on the derived boolean `any`. If a route already exists, setting `asked` does not rerun that effect; when the route is cleared, `any` becomes false and the effect leaves `asked` true. The conditions at lines 64-65 consequently keep the empty 320 px sidebar and suppress the 48 px no-route rail.

Reproduced in Chromium at 1920 x 1080: open `/?core=ready&route=exact#/materials`, wait for the route timeline, follow `#/route` in the same window, then choose More route actions > Clear route. The address correctly stays `#/materials`, but the empty sidebar remains (one Route complementary region, zero No route navigation regions). `/tmp/uc7i-r2-explicit-open-clear.png` captures the result. This violates the no-route rail requirement and the Report's claim that a route ending returns to the rail. The existing route-appearing/clearing test passes because it never explicitly opens an already-followed route. Limit the empty-sidebar request to the empty state or ensure it is consumed while a route exists, and cover opening an existing route before clearing it while preserving the new explicit empty-state behavior.
