---
schema: atelier.review
schema_version: 3
round: 1
commit: 015e19cdcf716abcbad0b5aca47c61f2b990fa5a
reviewers:
- thread:delegated-task:command%3Amcp%3Ab5d62477-e0e3-4a53-bba4-112ad4e876b3%3Adelegate-task%3Abe35cdbb-07a4-41fd-9960-9e77950cfb11
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T12:26:15.150295502Z
---

## Summary

Complexity: medium. The shared cycle calculation is small, but the change also installs a global preview clock and threads it through capture plans.

Value: medium. Now and Powerplay are current pages, and their cycle countdown and merits need to remain correct across Thursday turnover.

Risk of merging it: medium. The change affects visible cycle figures on both pages; it is reversible and changes no persisted data.

The value justifies the scope, but the two clock-lifetime problems below prevent approval.

Reviewed `git diff main...015e19cdcf716abcbad0b5aca47c61f2b990fa5a`, both views and their polling, the shared helpers, preview transport and fixtures, capture plans/reports, and the core's `Standing::view` and `powerplay_overview` command. The core does compute a current end and merits on every successful read, as the Report says. Inspected the retaken Now/docked and Powerplay/pledged 2560x1440 PNGs: both show the reported countdown and merits. Every recorded capture URL has the fixed clock, including scenarios with their own query. A standalone controlled-clock check confirmed the preview's fixed start, advancing `Date.now()` and `new Date()`, and unchanged explicit-date parsing.

Ran `pnpm exec vitest run src/lib/powerplay.test.ts src/modules/now/View.browser.test.ts src/modules/powerplay/View.browser.test.ts --maxWorkers=1 --no-file-parallelism`: all 35 tests passed. `pnpm lint` and `pnpm check` passed. Heavy commands ran sequentially. Then ran two additional browser reproductions by appending diagnostic tests through an in-memory Vite transform, without changing source or test files. Both failed as described below. Also compiled Now with Svelte to confirm its clock becomes a derived value without reactive dependencies.

The submitted tests establish rollover for an answer that is already expired when the view opens, but do not cover a mounted view crossing the boundary. The helper's table covers the requested offsets, and hiding merits rather than calling them last cycle's is sound for multi-week-old answers. I did not rerun the full 857-test suite, regenerate all captures, run Windows/Tauri, or independently establish that the tests were executed before implementation. The supplied capture files and clock implementation support the capture claim; the live rollover and merits requirements remain incomplete.

## Finding 1: Now freezes the cycle clock for the lifetime of its Powerplay block
Recommendation: fix now

In `app/src/modules/now/View.svelte:156`, `{@const now = Date.now()}` has no reactive dependency. Svelte compiles it as `derived(() => Date.now())`, so it is evaluated once and remains cached while the pledged block stays mounted. Even successful polls that replace `power.value.standing` keep using that first timestamp. This regresses the previous expression, where `timeLeft(st.week_ends)` read the current time when a new standing arrived.

Reproduced with fake Date and interval timers: open Now at 2026-10-22 06:59 UTC using the recorded standing ending at 07:00, then have subsequent successful overview polls return the next end, 2026-10-29 07:00, with zero cycle merits. Advance two minutes. The block correctly receives the new zero merits, but displays `7 d 0 h left in the cycle` instead of `6 d 23 h`, because its timestamp is still 06:59. With a retained expired answer it also continues treating the ended cycle's merits as current.

Use a reactive clock that advances with the existing polling schedule, including when a read fails, and use the same current timestamp for countdown and merits. Add a controlled-timer case that crosses the boundary while the view remains mounted.

## Finding 2: Powerplay still shows ended-cycle merits when reads fail across turnover
Recommendation: fix now

The new `cycleMerits(st, now)` in `app/src/modules/powerplay/View.svelte:105` uses the timestamp updated only in the successful-read handler at lines 19–30. If the last successful answer is before Thursday 07:00 and subsequent reads fail, `now` stays before the end indefinitely. Consequently `cycleAt` never sees the expired answer, the countdown never rolls, and the ended cycle's merits remain labelled `this cycle`.

Reproduced with fake Date and interval timers: load the recorded 1,829 merits with an end of 2026-10-22 07:00 at 06:59; reject later overview calls; advance two minutes. The page still contains the 1,829 cycle figure at 07:01. This is the real stale-answer path identified by the Report, not an additional hypothetical feature. Its acknowledged frozen-clock limitation defeats the second Done when line and leaves rollover ineffective on that path. The new test instead successfully returns an already-expired standing, which the real core does not normally return.

Advance the display clock independently of successful overview responses, while retaining the old standing and its stale warning. Cover a successful read followed by failed reads across the end; the old merits must disappear and the countdown must target the next end.
