---
schema: atelier.review
schema_version: 3
round: 2
commit: 9cd6474856860ffb18a2353ae99cbfa3f691f057
reviewers:
- thread:delegated-task:command%3Amcp%3A40e7bd6a-75d7-481f-bc07-e18b62a688e1%3Adelegate-task%3Ac80eeae4-d073-438d-85c7-b7582ddb942a
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T05:32:30.693911667Z
---

## Summary

- Complexity: high. The plan changes the frame and shared styling, migrates a persisted setting, adds engineer advice, and then rebuilds screens across the app.
- Value: high. It addresses the owner's stated problems with pages already in daily use and follows his approved mockups.
- Risk of merging it: high. Incorrect engineer eligibility or demand baselines can silently give bad advice; Text size also upgrades saved configuration. The plan now names tests for those contracts, while visual changes remain easy to revert per item.

The value warrants the work. The shared foundation followed by individual screens is a reasonable split; a smaller colour-only change would leave the navigation and page-shape problems unresolved.

I read the mission, design, notes and all listed items at 9cd6474856860ffb18a2353ae99cbfa3f691f057. I reassessed all nine carried findings against the amendment and checked the relevant current code: route and navigation ownership, Text size storage and scaling, cargo eligibility and ship_status, engineer offers, page-sizing enrollment, and the shared search kit and its consumers. The earlier findings are fixed. One ready item still needs a contract for its consumers without side panels. The final capture coverage can be assigned when the later screen items are readied.

This is a plan review. I ran no app tests or builds, did not inspect the rendered mockups, and did not validate the proposed appearance or Windows/WebView2 behaviour. Screens in increments 3 and 4 still have provisional acceptance criteria and need their planned detailed shaping before implementation.

## Finding 1: Keep Import reachable before a route exists
Recommendation: fix now
Was: 1-0weo.1
Now: fixed

astromech-uc7i/item.md:32 now gives both the empty rail and the narrow strip an Import action and requires a first-route import journey in both states. Import no longer depends on an active route.

## Finding 2: Define migration and the unset state for Text size
Recommendation: fix now
Was: 1-0weo.2
Now: fixed

astromech-b2cz/item.md:31-32 distinguishes an absent choice, accepts the three old strings, specifies conversion using the previous scale and multiplier, and saves the percentage once through set_window. It requires old-string, missing-field and restart coverage. The previous implicit-normal reply is explicitly replaced.

## Finding 3: Make the first-run formula agree with the 1080p default
Recommendation: fix now
Was: 1-0weo.3
Now: fixed

astromech-b2cz/item.md:31 replaces nearest-step rounding with explicit height bands and asserts 1080 -> 100, 1440 -> 145 and 2160 -> 180. Those values agree with the mission and checkpoint capture matrix.

## Finding 4: Use the selected commodity's cargo for Sell demand, and separate Buy's empty-hold rule
Recommendation: fix now
Was: 1-0weo.4
Now: fixed

astromech-3ca6/item.md:30 now names the selected commodity's sellable tonnes, reuses cargo.rs eligibility, gives Buy its free-space baseline even with an empty hold, and distinguishes known zero from unknown inventory. It assigns the added data to ship_status and requires the mixed-hold and empty-hold cases.

## Finding 5: astromech-uc7i and astromech-5a01 both rename Galaxy to Places and can start together
Recommendation: fix now
Was: 1-hckw.1
Now: fixed

astromech-uc7i/item.md:35 owns the rename. astromech-5a01/item.md:14 now depends on uc7i, and its scope at line 39 agrees. The design's increment-2 ordering also puts the tabs after the sidebar.

## Finding 6: astromech-5a01 measures ten tabs but ships nine
Recommendation: fix now
Was: 1-hckw.2
Now: fixed

astromech-5a01/item.md:33 measures the nine tabs it ships; astromech-coq6/item.md:27 explicitly remeasures with ten when NOW joins.

## Finding 7: The route sidebar keeps Title and Timeline on the page-sizing list
Recommendation: fix now
Was: 1-hckw.3
Now: fixed

astromech-uc7i/item.md:36 puts the sidebar width in the frame, names the route components moving to PAGE_SIZING_EXEMPT, and keeps ImportDialog enrolled. The current mission criterion and astromech-gvy0/item.md:28 preserve that exemption at mission close.

## Finding 8: Glance-test has no rule for the route that sits on every screenshot
Recommendation: fix now
Was: 1-hckw.4
Now: fixed

The mission criterion and astromech-crxa/item.md:25 score the sidebar once as frame and reject sidebar answers for another page's job. astromech-8e22/item.md:35 applies the same separation at the checkpoint.

## Finding 9: The plan still describes increment 1 as the work ahead
Recommendation: fix now
Was: 1-hckw.5
Now: fixed

The mission now records increment 1 as shipped and increment 2 as the next useful slice. design.md:153 places Now in increment 3; its Derisking and Risks sections record the completed stop, the checkpoint without another stop, and Text size replacing the zoom. The provisional screen items' remaining old context is bounded by their explicit 8e22 dependency and can be refreshed when they are readied.

## Finding 10: Keep report details reachable in shared lookups without a side panel
Recommendation: fix now

.atelier/items/astromech-3ca6/item.md:29 changes the shared result row and moves facts such as Updated into the side panel. That destination exists only for a SearchKit inside ListDetail: app/src/lib/ui/SearchKit.svelte:273-281 obtains the optional context, and lines 298-303 make names selectable only when it exists. Powerplay's Targets.svelte:37 and Colonisation's View.svelte:111 use SearchKit without that context. Neither the current frame nor their page supplies a panel. These consumers are not covered by the Places-finder acceptance list at 3ca6/item.md:32-33, and their later screen rewrites cannot preserve information lost in increment 2.

Specify what happens to the moved facts for these consumers. Removing a report's age from the row must keep it reachable through a disclosure, preserve it for the panel-less variant, or supply a detail surface there. A newly fetched answer can contain a weeks-old report; SearchKit's answer-fetch age at line 404 is not its report age, and the planned over-30-days mark does not cover a younger report. Add the standalone consumers to the item's acceptance check so the shared redesign does not make their existing report details inaccessible. This does not require redesigning their pages in increment 2.

## Finding 11: Assign the final four-size capture pass when the screen items are readied
Recommendation: defer

The mission's fills-the-window criterion (mission.md:71-75) requires every final page at four sizes. The later screen items currently ask for only 1100 x 700 and 2560 x 1440 captures (for example astromech-dpaz/item.md:28 and astromech-nnn0/item.md:28). astromech-gvy0/item.md:29 also asks for two, and crxa checks the final screenshots only at 2560 x 1440. b2cz's all-page pass and 8e22's four-size checkpoint precede those screen rewrites, so their captures cannot prove the final pages at 1920 x 1080 and 3840 x 2160.

This belongs to the already-planned detailed shaping of increments 3 and 4, so it need not block the ready foundation work. Before readying those items, assign the remaining sizes per screen or give a final sweep explicit ownership of all-page four-size captures and the fills-the-window evidence receipt. No additional approval gate is needed.
