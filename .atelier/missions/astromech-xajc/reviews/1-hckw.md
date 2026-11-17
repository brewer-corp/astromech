---
schema: atelier.review
schema_version: 3
round: 1
commit: 9b1ae4495558ff5427a4cd2c07514ae1bfb1e982
reviewers:
- 01a1e7f6-1d6f-7043-a836-c2c57424d5f1
model: grok-4.6
verdict: changes
created_at: 2026-11-17T05:25:07.611788697Z
---

## Summary

- **Complexity:** high. Increment 2 rewrites the frame, the colour contract and the tab tree, and every later screen is specified only provisionally.
- **Value:** high. The pages are in daily use, increment 1 already proved the foundation, and Sumeet asked for these five changes from mockups he saw.
- **Risk of merging it:** medium. Failures show in the preview and revert per page, but two ready items share one navigation change, and the glance-test criterion does not say how to score a page once the route sits on every screenshot.

The five changes Sumeet asked for are in ready items with a checkpoint, and the screens stay blocked on that checkpoint. The plan is worth running once the increment-2 collisions and the glance-test rule are fixed. A much smaller change would not get the sidebar, the calmer pages or the tabs.

I read `mission.md`, `design.md`, `notes.md`, every mission item (ready, draft and done), DESIGN.md's Height Scale Rule and Frame, `app/src/lib/modules.ts`, `app/src/App.svelte`'s default page, every `app/src/modules/*/manifest.ts`, and `app/eslint.config.js`'s `PAGE_SIZING` lists. I checked `spike/nav-rework` at `dea5f0bb` still holds the captures the items cite, and `mockups/glow-up` still holds the `glow-up-*.png` files `n41x` is to bring to main. I did not run the app, the spike mockups, or Windows WebView2.

## Finding 1: astromech-uc7i and astromech-5a01 both rename Galaxy to Places and can start together
Recommendation: fix now

`design.md` (lines 82–88) runs the route sidebar and the tabs in parallel after `astromech-3kf5`. Both ready items then own the same rename.

- `astromech-uc7i` Done when: "GALAXY becomes PLACES, one page with no page tab row." Scope: "The frame, the route module and the Places top tab."
- `astromech-5a01` Done when: "Top tabs in order: SHIPS, ENGINEERING, PLACES, …"

Today `app/src/lib/modules.ts` has a `galaxy` group whose pages are Route and Places (`app/src/modules/route/manifest.ts`, `app/src/modules/places/manifest.ts`). Both items are blocked only on `astromech-3kf5`, so two implementers will edit `groups` in `modules.ts` at once. If `5a01` lands first, the top tab is already named Places while Route is still a page under it, so the page tabs read Places > Route | Places.

Give the rename to one item. Either block `5a01` on `uc7i`, or leave the Galaxy title on `uc7i` and have `5a01` list that tab as whatever `uc7i` named it.

## Finding 2: astromech-5a01 measures ten tabs but ships nine
Recommendation: fix now

`astromech-5a01` Done when first bullet ships nine top tabs and postpones NOW: "SHIPS, ENGINEERING, PLACES, TRADE, EXPLORATION, POWERPLAY, LOGBOOK, CONTROLS, SETTINGS (NOW joins at the front with astromech-coq6)."

The fourth bullet then says "The header's second tab row comes in at the width the ten tabs need, measured at each Text size step, and DESIGN.md's header widths are updated with the measurement."

Those widths go into DESIGN.md and `astromech-8e22` checks that they match the captures. Nine tabs wrap later than ten. `coq6` is draft and blocked on `8e22`, so the checkpoint will bless a nine-tab header, then NOW will move the breakpoint.

Measure the nine tabs this item actually ships, and say `coq6` remeasures when NOW joins. If the intent is to reserve NOW's width now, the item must say how (a placeholder tab, a cited width from the spike) rather than "ten tabs."

## Finding 3: The route sidebar keeps Title and Timeline on the page-sizing list
Recommendation: fix now

Mission success criterion `fills-the-window` (`mission.md` lines 66–69) turns the check on for every page file "except the Controls ones and the frame's two parts in the route module." `app/eslint.config.js` lines 36–39 name those two parts: `Strip.svelte` and `PlotBar.svelte`. `gvy0`'s Done when repeats the same pair.

`astromech-uc7i` moves the Route page into the frame: the plotter title and the timeline sit in a 20rem sidebar. Those files are currently enrolled as pages (`eslint.config.js` lines 26–28: `Title`, `PlotSetup`, `ImportDialog`, `View`, `Timeline`, both `width` and `font-size`). A 20rem sidebar width in `Title.svelte` or `View.svelte` will fail the check this item does not mention.

`uc7i` must say where the sidebar width lives (the frame, not a page file) and which route files leave `PAGE_SIZING` for `PAGE_SIZING_EXEMPT`. Update the mission criterion and `gvy0` to match, so increment 4 does not "enrol whatever is left" back onto frame files.

## Finding 4: Glance-test has no rule for the route that sits on every screenshot
Recommendation: fix now

`glance-test` (`mission.md` lines 75–80) is DESIGN.md's three-pointing test on every page outside Controls, shown by `astromech-crxa` from the 2560 × 1440 screenshots alone. `crxa` writes one expected main value, one changeable thing and one thing that needs the commander per page.

After `uc7i`, every one of those screenshots includes the route sidebar: an editable destination, Plotter / Replot, and the next stop raised. A fresh agent can point at the sidebar for every page. The criterion then either fails across the board or accepts "the destination" as the changeable thing on Materials, Powerplay and Settings.

`crxa` (and the criterion) must say the three answers are for the page's own job, and that the route is scored once as frame — or that a page whose job is not the route answers "none" for a change that lives only in the sidebar. `8e22` already asks "on each page the brightest things are the main value, what can be changed and what needs the commander"; without the same rule it will file the sidebar as outshining the page.

## Finding 5: The plan still describes increment 1 as the work ahead
Recommendation: fix now

The amendment is in `mission.md` Why and Authority, and in `design.md`'s revision 5 and increment-2 list. The rest of the plan still reads as if that work had not happened.

- `mission.md` lines 33–34: "The Route page redesign agreed on 6 Nov (astromech-ok29) is the model for the fix and has not been built." `ok29`, `irrv`, `qwvs` and `irtr` are done.
- `mission.md` lines 58–59: "Worth shipping on its own: the foundation … and the Route page." That slice is already on main. Increment 2's shippable slice is the sidebar, the calmer pages and the tabs.
- `mission.md` lines 63–64: Controls "get only the frame, the scale and Tip." Revision 5 removes the scale.
- `design.md` lines 150–155: "**Now page (increment 2).**" `coq6` is draft and blocked on `8e22`; lines 101–103 already put the screens in increments 3 and 4. A lead splitting increment 2 can assign Now from the later heading.
- `design.md` Derisking point 3 (lines 230–232): increment 1 "is its one planned stop" and "the screens in increments 2 to 4 are planned from what he says." Authority (`mission.md` lines 111–115) already took that stop and says there is no second one; screens start when `8e22` merges.
- `design.md` Derisking "third" risk (lines 237–241) and Risks (lines 251–253) still treat the height zoom as the WebView2 lever. `b2cz` removes it; `3340`'s Goal already says the zoom is gone.

An increment-2 implementer or a lead using Derisking will wait for a stop that is not planned, rebuild a Route page that exists, or keep zoom workarounds. Update these sentences so they describe what is done and what increment 2 actually ships. The screen items' "Preliminary: planned in detail after increment 1" lines are the same class of stale text; `blocked_by: astromech-8e22` is the real gate, so those can wait until the screens are readied.

## Finding 6: astromech-3340's Done when still checks the zoom this plan removes
Recommendation: defer

`design.md` Derisking still points at `astromech-3340` as the Windows check. That item is not on the mission, but its Goal was rewritten for text size and colours. Its Done when is still "in a release that has astromech-k6pl: text is comfortably readable at his usual window size … and the orange no longer looks washed out." `k6pl` is already on main; the check that matters after this amendment is the first-run step, the six sizes from the seat, and whether Windows display scaling adds to the root size (the Goal's own words). Rewrite the Done when when `3340` is next touched, so the human check does not sign off the old zoom.
