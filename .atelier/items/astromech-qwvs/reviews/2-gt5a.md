---
schema: atelier.review
schema_version: 3
round: 2
commit: 5b37e5aefab3a44a9dc4d573ee211b5359b4aa33
reviewers:
- thread:delegated-task:command%3Amcp%3A5dc072bd-93dd-4632-9a54-e362846aa285%3Adelegate-task%3A25741ebd-6b7a-4f33-829a-ad356e263048
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-12T11:12:22.195116616Z
---

## Summary

Complexity: medium. The timeline introduces a row model and clipboard feedback, and Make next adds a core command with bindings and preview support.

Value: high. The agreed Route timeline exposes the available jump data and the commander's position while flying.

Risk of merging: medium. Position errors would affect route guidance shown to players, but failures are visible and the change is reversible.

The value warrants the scope. Both carried findings are fixed, and I found no new merge blockers or reason to shrink the implementation.

I read the full change against `main` at `5b37e5aefab3a44a9dc4d573ee211b5359b4aa33`, the changes since round 1, the item and parent Design, DESIGN.md and CONTRIBUTING.md. I traced the timeline's rows and position calculations against the core's `timeline` contract, and checked `route_go`, its stale-stop guard, bindings, preview handling, clipboard feedback, details lookup and guide changes.

Validation passed: all 619 frontend tests across 83 files; all 30 Rust route tests; `every_window_command`; all three `preview_fixtures` tests, including regeneration comparison against the real core; `pnpm lint`; `pnpm check` with zero errors or warnings; `git diff --check`; and `scripts/check-guide.py`. The first targeted frontend run encountered an occupied Vitest port; the subsequent complete run finished cleanly. Both page-sizing restrictions are enabled for View.svelte and Timeline.svelte and pass.

I captured and inspected the timeline at 1100 x 700 and 2560 x 1440, and the first-waypoint, replotted and exact scenarios at 1100 x 700. Document width equalled viewport width in every capture; root zoom was 1.44 at the larger size. The timeline follows the written design, and the smaller window scrolls to the remaining rows. The new recorded scenarios show the pin on Alrai, including before waypoint zero, without marking Sol after a replot. I did not compare pixels against the external mockup artifact or run the Windows app in Elite.

The earlier deferred finding `1-mga3.3` is also fixed: the expanded-leg branch inserts a warning row when `noScoop(stop)` holds, and the browser test now asserts the flown leg retains it. The warning's complete-list and known-dry guards remain intact.

## Finding 1: Render the flown leg when its destination is the first waypoint
Recommendation: fix now
Was: 1-mga3.1
Response: fixed in 1b8c5af4
Now: fixed

`Timeline.svelte:59` now builds a leg when the stop is next, including index zero. The recorded `/first` reply has `route.next === 0` and nine listed hops; the browser test and preview show the current-system row followed by the eight intermediate jumps and the destination stop. Count-only first legs also enter the leg-rendering branch.

## Finding 2: Do not interpret NavRoute position zero as the previous waypoint
Recommendation: fix now
Was: 1-mga3.2
Response: fixed in 1b8c5af4
Now: fixed

`Timeline.svelte:48-50` checks the actual current system before marking the previous waypoint. When position is zero elsewhere, lines 64-66 insert the current system at the start of the expanded leg. The `/replotted` fixture is verified against the real core, and its browser test and preview put You are here on Alrai Sector LH-V b2-4 while leaving Sol unmarked.
