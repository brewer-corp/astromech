---
schema: atelier.review
schema_version: 3
round: 1
commit: 86f23f511322693ba8537e9c7bf90809fbc44225
reviewers:
- thread:delegated-task:command%3Amcp%3Aa2ff8ecd-9bd5-430d-a2ff-147ae09eea93%3Adelegate-task%3A19fa0738-6483-468c-9d7c-3458f87c822b
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T11:20:08.148691166Z
---

## Summary

Complexity: high. The page adds shared frame polling, several independent reads, a keyed asynchronous cargo answer, and navigation and responsive layout changes.

Value: high. A home page that follows the game is the item's intended outcome, and the change delivers its main states.

Risk of merging: high. Failed reads can silently leave old game, location and hold information looking current, including an actionable cargo recommendation.

The value warrants the change's scope, once the failure reporting below is fixed. I found no materially smaller implementation that would preserve the requested outcome.

I reviewed the diff against main at 86f23f511322693ba8537e9c7bf90809fbc44225, checked the Done when lines against the implementation, and followed the cargo lookup through its core origin, ship and hold reads. I inspected the navigation and manifests, shared station components, sale key and takes tables, preview changes, fixture dump change, lint enrollment, guide changes, and committed capture plans and audit reports. I also viewed the docked minimum-size and in-space large-size captures.

Independently passed: pnpm check, pnpm lint, pnpm test (93 files, 788 tests), pnpm exec tsc -p journeys, all seven journeys, python3 scripts/check-guide.py, and cargo test -p astromech-app --test preview_fixtures (three tests, including fixture drift). Additional headless Chromium probes used the real page and preview transport with failed boundary calls and a controlled clock; they reproduced the finding below and confirmed that Session's equivalent failure does show the shared stale line.

I did not run the Windows app or contact live Ardent. I checked the submitted layout evidence rather than independently repeating every capture and the six header-width measurements. The exploration-at-risk branch has no recorded risky fixture, so its appearance was checked from the code rather than reproduced with recorded data.

## Finding 1: Report failed game, location and risk reads beside retained answers
Recommendation: fix now

`app/src/modules/now/View.svelte:81` only renders `here.failed` in the running-without-a-place branch at line 89. Once a station or system has loaded, its retained value takes an earlier branch, hiding every subsequent failure. `unsold.failed` is never rendered. The frame retention at lines 39–46 likewise discards failed status and ship reads without giving the affected blocks a failure or stale state; Mining at line 150 receives neither. `app/src/lib/frame.svelte.ts:17` calls wrappers that reduce those errors to null, so the page also lacks their reasons.

Reproduction: open `?core=ready&game=running#/now`, wait for the cargo answer, then reject just `docked_here`, `exploration_unsold`, `ship_status`, or `status` at the transport and advance the page clock by 15 seconds. Each produces failed reads but no warning on Now; Test Orbital remains the title and the cargo destination action remains enabled. In the in-space state, rejecting status leaves the old 18 t/hour highlighted without a stale line. A cold load with `?core=ready&game=running&fail=status#/now` even says "Elite isn't running" rather than reporting that its status could not be read. Rejecting session_summary instead correctly shows "Could not refresh", which confirms this is missing wiring in these paths.

This violates the Done when requirement that a failed poll retain its last answer with the shared Stale line, or show the initial failure in words. It can also hide that the exploration-risk check is unavailable. Preserve failure metadata for the frame reads and render each affected read's initial error or stale line beside the data it qualifies, including the head and risk check. Keep the existing retention and unchanged-key behavior; restoring failures must not turn them into repeated trade_cargo requests. Cover these failure paths with boundary-failure browser cases.
