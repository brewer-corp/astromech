---
schema: atelier.review
schema_version: 3
round: 3
commit: e8ebfa76025e29b49a2a4cabc1102a43a8931a2a
reviewers:
- thread:delegated-task:command%3Amcp%3Af547e1d1-af7b-4267-b2d0-b889d0455ab6%3Adelegate-task%3A935f08da-5868-4b74-9cb3-eaea7b2c5f23
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T11:40:48.787387366Z
---

## Summary

Complexity: medium. The loadout and module panel share a plan owner with the retained Plan view across several components, using existing core commands.

Value: high. Seeing fitted modules and planning their engineering are central Ships workflows.

Risk of merging: medium. The page edits persistent plans and supplies arrival shopping lists, but retains the existing core contracts and is reversible.

The value warrants the change. I found no substantially smaller implementation that delivers the loadout, editable module panel and retained Plan workflow. The carried finding is fixed, and I found no new problem requiring a change.

I reviewed the full code diff against main at e8ebfa76025e29b49a2a4cabc1102a43a8931a2a and the delta since round 2. I checked the Done when lines, shared plan restoration and evaluation, dropdown and ADD TO PLAN behavior, fitted versus planned grades, slot grouping, retained Plan sections and stats, fleet and error states, routes, guide changes, sizing and token checks, and the audit's gap measurement. The current code preserves both earlier production fixes. The fixtures confirm that recorded stats and the sample plan belong to different ships, as the Report explains. The documented choices about military slots, planned hexagons on unengineered modules and columns at standard sizes are reasonable.

Validation: the four relevant test files passed (103 tests), followed separately by the full frontend suite (90 files, 782 tests). pnpm check passed with zero errors or warnings; pnpm lint, scripts/check-guide.py (22 pages) and git diff --check passed. The fleet-refresh regression now explicitly advances its fake interval by 1000 ms and verifies both the failure message and retained empty answer.

I inspected the committed narrow and wide module captures and corrected no-fleet capture, and checked the saved audit plan and report. Its 19 entries have empty sideways, clipped, overlaps, small, far and contrast lists; all recorded Open in gaps are 24 layout px. I did not rerun the visual audit or journeys in this round, and did not check live journals or Windows/WebView2. No application files were changed during review.

## Finding 1: Drive the new fleet-refresh test with fake timers
Recommendation: fix now
Was: 2-iq82.3
Response: fixed in ff87a726: the test installs fake setInterval and advances the poll 1000 ms, as the Settings status-poll test does.
Now: fixed

View.browser.test.ts installs vi.useFakeTimers with only setInterval and clearInterval before rendering, then advances the clock by 1000 ms after making fleet reads fail. The real-time 2500 ms assertion override is gone. The focused and full test runs pass, and the case retains its assertions about the empty cached fleet and failed refresh. This meets the repository's determinism rule and the earlier finding.
