---
schema: atelier.review
schema_version: 3
round: 1
commit: 906a1f4b9b45724e401c53454ccf9ed9e87e77a3
reviewers:
- thread:delegated-task:command%3Amcp%3A5dc072bd-93dd-4632-9a54-e362846aa285%3Adelegate-task%3A42cb0e58-f1fb-4f48-9797-249fab9133df
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-12T11:04:02.877934942Z
---

## Summary

Complexity: medium. The timeline adds a row model and clipboard feedback, and Make next crosses the core command registry, generated bindings, preview and window tests.

Value: high. This delivers the agreed Route timeline and makes the available jump data visible while flying.

Risk of merging: medium. Incorrect position and missing jump rows affect the route guidance a player reads, but the change is reversible and the new command checks the selected system before moving the route.

The value warrants the scope. The two position/rendering faults below need fixing before merge; no smaller replacement is needed.

I reviewed `git diff main...906a1f4b9b45724e401c53454ccf9ed9e87e77a3`, the item and parent Design, DESIGN.md and CONTRIBUTING.md, and traced the timeline against `route.rs`'s actual view contract. I checked the new command, bindings, preview handler, test changes and guide. All 617 frontend tests passed, as did all 30 Rust route tests and `every_window_command`. The page files passed ESLint (both sizing restrictions enabled), `pnpm check` reported no errors or warnings, and `scripts/check-guide.py` passed.

I captured and inspected the timeline preview at 1100 x 700 and 2560 x 1440. Its document width equalled the viewport at both sizes; the large preview used zoom 1.44. The shown layout follows the written design, with content below the small viewport reachable by scrolling. I did not compare against the external artifact's original pixels or run the Windows app in Elite.

For the edge cases, I ran the real headless core against temporary copies of `worlds/preview/route`, sent `route_set`, and stepped the recorded NavRoute and journal input. I rendered those core replies through the page's transport boundary in Chromium. For the replot case I replayed the recorded NavRoute starting at its third system after the recorded two jumps. For the dry case I changed only the raw NavRoute star classes to TTS. These exposed gaps the passing tests do not cover. No implementation file was changed.

## Finding 1: Render the flown leg when its destination is the first waypoint
Recommendation: fix now

`app/src/modules/route/Timeline.svelte:53` puts all leg rendering inside `if (i > 0)`. A route imported without the current system, an engineer trip, or a Set destination route can have `route.next === 0` while `view.stops[0].hops` contains the complete galaxy-map leg. The core explicitly supports this: it uses NavRoute's starting system to determine position independently of whether a previous waypoint exists.

I reproduced this by setting the route to `[PSR J1752-2806, Colonia]` in `worlds/preview/route` at `running`, then stepping to `flying`. The real reply has `next: 0`, eleven listed hops and `position: 2`, with `here: Alrai Sector LH-V b2-4`. Rendering it produces zero jump rows and no You are here marker, although the next-stop heading says nine jumps away. This fails the first Done when line's requirement to open the flown leg whenever its jumps are listed. The same guard also hides the unlisted-leg explanation before a first waypoint.

Allow the flown leg before waypoint zero to render, without requiring an earlier stop row, and cover that route shape in the browser test using a core-recorded scenario.

## Finding 2: Do not interpret NavRoute position zero as the previous waypoint
Recommendation: fix now

`app/src/modules/route/Timeline.svelte:45` unconditionally chooses `next - 1` when `view.position === 0`. In `crates/astromech-core/src/route.rs:576-602`, position zero means the first system in the current NavRoute file. That system can be anywhere on the leg if the player plots again after travelling partway.

I reproduced this with the recorded Sol to PSR J1752-2806 leg. After the two recorded jumps into Alrai Sector LH-V b2-4, I replayed the same NavRoute from Alrai onward. The real core returns `here: Alrai Sector LH-V b2-4`, `next: 1`, `position: 0`, and the preceding waypoint remains Sol. The page puts the pin and You are here on Sol and omits Alrai entirely. This is incorrect location guidance during an ordinary replot.

Only mark the previous waypoint when it matches `view.here`. When position zero refers to another system, render the actual current system at the start of the expanded leg. Add coverage for a NavRoute whose starting system differs from the preceding waypoint.

## Finding 3: Keep a known dry leg's warning when it expands
Recommendation: defer

`app/src/modules/route/Timeline.svelte:55-62` computes `dry` only for collapsed leg rows. The expanded flown-leg branch produces jump rows, none of which renders the warning at line 188. Consequently, a known dry leg loses its warning when it becomes the leg being flown, when the warning is most useful.

With the recorded NavRoute's raw star classes changed to TTS and its waypoint jump count set to eleven, the real core lists all eleven jumps as not scoopable. `noScoop` returns true, but the rendered timeline contains no No scoopable star warning. The existing browser test checks dry legs ahead only. Since this item's warning acceptance line is specifically about legs ahead, this is deferred rather than a merge blocker; retain the warning for an expanded leg in follow-up work.
