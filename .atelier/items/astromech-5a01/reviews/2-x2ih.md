---
schema: atelier.review
schema_version: 3
round: 2
commit: 6caada8a31047c84d68c37bf1e64b292aea2f6fe
reviewers:
- thread:delegated-task:command%3Amcp%3Aee52845e-d5a7-466d-9986-172eca7785bd%3Adelegate-task%3Af408262b-8f7d-4b4d-ad41-9279d4c1c52a
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T09:18:43.010394566Z
---

## Summary

Complexity: medium. The navigation change splits one page across three owners and adds a separate inventory read to Materials.

Value: high. The change directly addresses the confusing page grouping and keeps the existing inventory, planning and engineer information available.

Risk of merging it: medium. It changes navigation and inventory presentation used by commanders, but failures are visible and the change can be reverted.

The value justifies the scope. The manifest changes keep the old page ids, Suits retains the existing planner, and the moved inventory uses Materials' existing board and panel. I found no smaller change that would satisfy the requested split.

I reviewed `git diff main...6caada8a31047c84d68c37bf1e64b292aea2f6fe`, the changes since round 1, all Done when lines, and the Report. I checked the route redirects, manifest ordering and palette collection; compared Suits against the original Odyssey page; checked Materials' selection, polling, empty and failure branches; checked engineer referral rendering against the core's unlock guidance; and inspected the role exclusions in the tab-row test and the Presets table markup. I read the guide and design changes and inspected the committed after captures, plus the replacement Ships and Stored modules before captures.

`pnpm lint`, `pnpm check`, `pnpm build` and `pnpm test` pass: 86 files and 724 tests. This includes the pending/failed ShipLocker regression, referral coverage, every registered page's tab rows, old links, stale-data states and Suits' planner and bartender behavior. `python3 scripts/check-dist.py` and `python3 scripts/check-guide.py` pass. I also measured the header in headless Chromium with loaded fonts at all six Text size steps. The one-row thresholds are within 9 px of the documented measurements; the stated starting-size and stacked-header behavior holds.

All three earlier findings are fixed, and I found no new merge blockers. I did not run Rust checks because no Rust changed, test against live game files on Windows, or reproduce the unfinished-setup header measurement. The Report explicitly leaves that last measurement unclaimed. The before captures use a fallback font as disclosed, but now show the page content needed to assess the move.

## Finding 1: Keep the Raw grid out of On foot's loading and failure states
Recommendation: fix now
Was: 1-l4sg.1
Response: fixed in 2242b5dd, with a test that fails without it (ShipLocker pending, then failed, with the ship materials answered).
Now: fixed

Confirmed: the outer render branch now depends on `onFoot` alone, with the `foot` check nested inside it. The regression test passed with ship materials available while the ShipLocker read is pending and after it rejects; ship-material tiles stay absent.

## Finding 2: Preserve referral tasks that the unlock tree does not yet display
Recommendation: fix now
Was: 1-l4sg.2
Response: fixed in 8937386b, with a check that Hero Ferrari's card says "Refers Wellington Beck: Provide 5 Settlement Defence Plans" before Hero is unlocked.
Now: fixed

Confirmed: suit cards now render `refers` and `referral_task` independently of unlock status. The passing test checks Hero Ferrari is not unlocked and still shows the Wellington Beck requirement; the updated capture also shows it.

## Finding 3: Replace the blank Ships captures and capture Stored modules
Recommendation: fix now
Was: 1-l4sg.3
Response: fixed in 6cc109e7; the before captures from `main` draw in a fallback font, but the content and tabs are the same.
Now: fixed

Confirmed by opening the PNGs: both Ships captures now contain Heavy Lifter and the fleet rail, and both Stored modules captures show all three stored modules. The after images show the new Ships tab and page grouping.
