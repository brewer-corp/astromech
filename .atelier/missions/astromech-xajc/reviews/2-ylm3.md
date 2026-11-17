---
schema: atelier.review
schema_version: 3
round: 2
commit: 9cd6474856860ffb18a2353ae99cbfa3f691f057
reviewers:
- 01a1e7fd-f1d2-7103-b907-cd1e297e8d76
model: grok-4.6
verdict: changes
created_at: 2026-11-17T05:32:39.947821832Z
---

## Summary

- **Complexity:** high. Increment 2 still rewrites the frame, the colour contract, text size and the tab tree, and adds engineer-search logic.
- **Value:** high. The pages are in daily use, increment 1 is on main, and the five changes match Sumeet's recorded answer at the stop.
- **Risk of merging it:** medium. Visual failures show in the preview and revert per page. Grade advice and demand colours can be wrong without looking wrong; those two now name tests. Text size now has a migration contract. The remaining hole is navigation: `#/route` is specified through a mechanism that cannot keep the commander on the page they were on.

The value is worth the rest. Round 1's nine fix-now findings are addressed in the items they named. Increment 2 can start once `#/route` is specified as "open the sidebar" rather than a `MOVED` rewrite, and DESIGN.md's empty-rail rule matches the Import entry the first round added.

I read `mission.md`, `design.md`, `notes.md`, every mission item (ready, draft, done and dropped), DESIGN.md Typography and Frame, `app/src/lib/modules.ts`, `App.svelte`'s `MOVED` map and default page, `eslint.config.js` `PAGE_SIZING` lists, `scale.svelte.ts` and its tests, `config.rs` `TextSize`, `window_settings`, `ShipStatus`, `cargo.rs` `sellable`, `engineers::offers`, and the strip / Find or plot `#/route` links. I checked `spike/nav-rework` at `dea5f0bb` still holds the `tone5-*` / `hier-B-*` captures, and `mockups/glow-up` still holds `docs/design/glow-up-*.png`. I did not run the app, the spike mockups, or Windows WebView2.

## Finding 1: Keep Import reachable before a route exists
Recommendation: fix now
Was: 1-0weo.1
Now: fixed

`astromech-uc7i` now puts Import on the empty 48 px rail and on the strip below the dock width, and requires a browser test that imports a first route from the empty state, docked and narrow. That matches today's menu entry in `route/Title.svelte`.

## Finding 2: Define migration and the unset state for Text size
Recommendation: fix now
Was: 1-0weo.2
Now: fixed

`astromech-b2cz` now stores a percentage, reports an absent field as not chosen, migrates `smaller` / `normal` / `larger` to the nearest old zoom for this display, and requires config tests for the three old strings, the missing field, and a restart that keeps the result. A 1440p `normal` install is specified to land on 145.

## Finding 3: Make the first-run formula agree with the 1080p default
Recommendation: fix now
Was: 1-0weo.3
Now: fixed

`astromech-b2cz` replaces nearest-to-height/1000 with explicit bands: below 1200 px → 100, below 1600 → 145, otherwise 180. The tests assert 100 for 1080, 145 for 1440 and 180 for 2160, which matches the mission capture matrix.

## Finding 4: Use the selected commodity's cargo for Sell demand, and separate Buy's empty-hold rule
Recommendation: fix now
Was: 1-0weo.4
Now: fixed

`astromech-3ca6` now measures Sell against sellable tonnes of the searched commodity (`cargo.rs` rules), Buy against free space including an empty hold, tells a known zero from unknown cargo, names `ship_status` as the reply, and adds the mixed-hold and empty-hold Buy cases. `GameState` already has `cargo_inventory`.

## Finding 5: astromech-uc7i and astromech-5a01 both rename Galaxy to Places and can start together
Recommendation: fix now
Was: 1-hckw.1
Now: fixed

`astromech-5a01` is blocked on `astromech-uc7i`. `uc7i` owns the GALAXY → PLACES rename; `5a01` follows it. `design.md` runs the tabs after the sidebar.

## Finding 6: astromech-5a01 measures ten tabs but ships nine
Recommendation: fix now
Was: 1-hckw.2
Now: fixed

`astromech-5a01` measures the nine tabs it ships. `astromech-coq6` remeasures when NOW joins.

## Finding 7: The route sidebar keeps Title and Timeline on the page-sizing list
Recommendation: fix now
Was: 1-hckw.3
Now: fixed

`astromech-uc7i` puts the sidebar width on the frame and moves `Title`, `PlotSetup`, `Timeline`, `NeutronFile` and the `View` replacement to `PAGE_SIZING_EXEMPT`. The mission criterion and `gvy0` now point at that list.

## Finding 8: Glance-test has no rule for the route that sits on every screenshot
Recommendation: fix now
Was: 1-hckw.4
Now: fixed

The mission criterion, `astromech-crxa` and `astromech-8e22` now score the sidebar once as frame. An answer that points into it for a page is a fail for that page.

## Finding 9: The plan still describes increment 1 as the work ahead
Recommendation: fix now
Was: 1-hckw.5
Now: fixed

The cited sentences in `mission.md` Why / Outcome / Authority and `design.md` Now page, Derisking and Risks now describe increment 1 as done, increment 2 as the next shippable slice, and `8e22` as a checkpoint that does not stop the mission. `astromech-3340`'s Done when now checks text size and colours.

## Finding 10: `#/route` cannot land on the current page through the MOVED map
Recommendation: fix now

`.atelier/items/astromech-uc7i/item.md:35` removes the Route page and says `#/route` and its links (the strip, palette entries, the guide) "land on the page the commander was on with the sidebar open, through the `MOVED` map."

`MOVED` cannot do that. In `app/src/App.svelte:24-35` it is a static `Record<string, string>`: `activities` becomes `controller`, a finder slug becomes `places/...`. Following a hash replaces the location; the previous page is not in the map. Today's in-app entry points are hash links (`route/Strip.svelte:98` destination, `route/Plot.svelte:83` "Show the route"). A `MOVED` entry of `route → controller` would yank the commander off Sell or Materials onto My controller. A bookmark or guide link to `#/route` has no "page they were on"; `resolve` would need a fallback, and `MOVED` still cannot open the sidebar, which is frame state rather than a module id.

Say what the links do: an in-app Show-the-route control opens the sidebar and does not change the page; a cold `#/route` (bookmark, guide) opens the default page (My controller until Now exists) with the sidebar open. Drop `MOVED` as the mechanism, or say the strip and Find or plot stop using `#/route` at all.

## Finding 11: DESIGN.md's empty rail is still the spike's, without Import
Recommendation: fix now

Round 1 added Import to `astromech-uc7i`'s empty rail. The rulebook item did not move with it.

`astromech-n41x` Done when Frame (`item.md:33`) still says "no route is a 48 px rail". That is the spike (`4-no-route-2560.png`): one action, Find or plot. `n41x` is unblocked and writes DESIGN.md first. `uc7i` does not say it will amend that Frame bullet; `8e22` checks dock width, header width and rows-visible, not the empty-rail actions.

Put Find or plot and Import on `n41x`'s empty-rail sentence, so revision 5 matches `uc7i` instead of the capture it will replace.
