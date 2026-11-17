---
schema: atelier.review
schema_version: 3
round: 2
commit: 8bde95b8ccf2f63275e197e4a60d6e2fd363cef5
reviewers:
- thread:delegated-task:command%3Amcp%3Ab5d62477-e0e3-4a53-bba4-112ad4e876b3%3Adelegate-task%3Aeec90b1a-4ee6-4ede-967d-5ed74d8d84b1
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T12:32:08.788846165Z
---

## Summary

Complexity: medium. The cycle arithmetic is small, but the change also adds a global preview clock and passes it through capture plans.

Value: medium. Now and Powerplay are in use, and their countdown and cycle merits must remain correct across Thursday turnover.

Risk of merging it: medium. Both pages show changed cycle figures, but the change is reversible and alters no persisted data.

The value justifies the scope. Both carried findings are fixed, and I found no new problem requiring changes.

Reviewed the full diff from main to `8bde95b8ccf2f63275e197e4a60d6e2fd363cef5` and the changes since round 1. Read the shared cycle helpers, both views, Now's Poll, preview clock/transport/fixture handling, capture plans and reports, and the core's overview and standing calculation. The core does calculate the current cycle on successful reads. Rolling an expired end by whole UTC weeks targets the next end; leaving merits unknown instead of calling them last cycle's is correct for answers several weeks old.

Ran `pnpm exec vitest run src/lib/powerplay.test.ts src/modules/now/View.browser.test.ts src/modules/powerplay/View.browser.test.ts --maxWorkers=1`: all 3 files and 35 tests passed. The helper table covers one minute ahead, exactly at the end, one minute past, and three weeks past. Both view regressions now start with a successful read at 06:59, reject subsequent reads, advance fake Date and interval timers to 07:01, and verify the next countdown and removal of 1,829 cycle merits. Powerplay also checks the Ends line. The updated Poll timestamp is reactive and changes after successful and failed reads, so Now's mounted block no longer caches its opening time. Powerplay's failed-read handler also advances its clock.

`pnpm lint` and `pnpm check` passed with no Svelte errors or warnings. Heavy commands ran sequentially. A standalone Node check confirmed that the preview clock sets Date.now() and new Date() to the configured moment while preserving explicit ISO and date-component construction. Inspected the saved Now/docked and Powerplay/pledged 2560x1440 PNGs: they show 1 d 19 h and 1,829 merits, and Powerplay shows Ends Thursday 07:00 UTC. All 9 Now and 22 Powerplay recorded capture URLs carry their plan's fixed clock, including shots with their own query; the audit implementation appends it after either query choice.

I did not rerun the full 857-test suite, regenerate all captures, run Windows/Tauri, or independently establish that tests were executed before implementation. The changed tests, saved captures and clock implementation support the Done when claims. The Ends label intentionally omits the date, so its Thursday 07:00 wording remains the same after a weekly rollover.

## Finding 1: Now freezes the cycle clock for the lifetime of its Powerplay block
Recommendation: fix now
Was: 1-q0io.1
Response: fixed in c0d3f518: Now reads `Poll.at`, set whenever a read settles, instead of a `Date.now()` cached for the block's life.
Now: fixed

`Poll.at` is reactive and is updated whenever a read succeeds or fails. The Now block reads it for both countdown and merits. The new controlled-timer view test passes across the boundary while failed reads retain the old standing.

## Finding 2: Powerplay still shows ended-cycle merits when reads fail across turnover
Recommendation: fix now
Was: 1-q0io.2
Response: fixed in c0d3f518: Powerplay moves `now` on a failed read too, and both view tests now cross 07:00 with failing reads.
Now: fixed

The failed-read handler now updates `now`. The controlled-timer Powerplay test passes after retaining the 06:59 standing through two minutes of failed reads: it shows 6 d 23 h, keeps the next end's label, and replaces 1,829 cycle merits with an en dash.
