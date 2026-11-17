---
schema: atelier.item
schema_version: 3
id: astromech-unak
title: Powerplay cycle time reads '0 h 0 min left' once the recorded cycle end has passed
status: approved
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A8886d38c-8090-446d-9a27-b64ec70a6289
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T11:48:22.652698126Z
trivial: null
caused_by: astromech-rj6i
created_at: 2026-11-17T11:48:22.548657026Z
---

## Goal

The cycle's time left is right after the cycle has turned over. `timeLeft(st.week_ends)` in `app/src/lib/powerplay.ts` counts down to the cycle end the journal last gave. Once that moment has passed and the commander has not yet been sent a new one, Now and Powerplay read "0 h 0 min left" (seen in `docs/design/now/docked-2560x1440.png`, captured after the preview's recorded cycle end; `docs/design/powerplay/pledged-2560x1440.png`, captured before it, reads "1 d 19 h"). Powerplay cycles end every Thursday at 07:00 UTC, so the next end is known without the journal.

## Done when

- [x] A failing test first: with `week_ends` in the past, the time left counts to the next Thursday 07:00 UTC after now, on Now and on Powerplay, and "Ends ..." names that moment. A table test covers an end one minute ahead, one minute past, and three weeks past.
- [x] Merits "this cycle" from a cycle that has ended are not shown as this cycle's: the page says they are last cycle's, or leaves them out, and the Report says which and why.
- [x] The browser preview and the capture tool run on a fixed clock for these pages, so a capture does not change with the day it is taken; the Now and Powerplay captures are retaken.

## Scope

The cycle clock on Now and Powerplay, and the preview's clock for their captures.

## Report

Now and Powerplay no longer read "0 min left" once the cycle end they were given has passed. They count to the next Thursday 07:00 UTC instead, and they leave out the ended cycle's merits.

**Where the bug actually was.** The real core never sends a past end: `powerplay_overview` reckons `week_ends` and `week_merits` from the wall clock (`crates/astromech-api/src/commands.rs:180`), so its own answer is always for the current cycle. A past end reaches the window in two ways. One is an answer kept after the core stops answering (both pages keep the last answer on a failed read). The other is the preview fixture, which was recorded in the cycle ending 2026-10-22. The capture cited in the Goal came from the second: `docs/design/now/docked-2560x1440.png` was taken on the astromech-coq6 branch, before astromech-rj6i added a roll-forward to `shell.ts`. That roll moved the fixture's end on by whole weeks but kept its 1,829 merits, so the preview showed an old cycle's merits as this cycle's.

**What changed**
- `cycleAt(week_ends, now)` in `app/src/lib/powerplay.ts` holds the rule. An end still ahead is this cycle's. An end at or before `now` moves on by whole weeks to the first end after `now`, and the answer counts as `over`. `timeLeft` and `endsAt` read the end from it. Rolling the given end by whole weeks keeps the weekday and hour the core sent, so the Thursday 07:00 rule is not written a second time in TypeScript.
- **Merits: left out, not relabelled.** Once the answer's cycle is over, `cycleMerits` returns null and both pages show "–" for this cycle's merits. I chose this over "last cycle" because a kept answer can be several cycles old: "last cycle" would be wrong from the second week on, and this cycle's merits are unknown until the core answers again (its next answer resets them to the right figure).
- **Fixed clock.** The preview takes `?clock=<RFC 3339>` (`app/src/preview/clock.ts`). It shifts `Date` to start at that moment and run on from it, and leaves timers alone. A plan's new `clock` field makes `scripts/browser/audit.mjs` add it to every shot, including shots that set their own `query`. Both plans use 2026-10-20 11:23 UTC, which is inside the fixtures' cycle and after every Powerplay target report. The clock is opt-in: without it, `pnpm dev` runs on the wall clock and now shows the fixture as the old answer it is. I removed the `shell.ts` roll-forward. It is documented at the top of `fakecore.ts` and in CONTRIBUTING.md.
- The captures were retaken. Now shows "1,829 merits this cycle" and "1 d 19 h left in the cycle", and Powerplay shows "1 d 19 h left" and "Ends Thursday 07:00 UTC". After merging main I retook them again. Apart from `report.json`'s commit, the second take differed in one PNG (`powerplay/not-pledged-1100x700.png`), so captures no longer depend on the day they are taken. Most of the Powerplay `report.json` diff from before is the Now tab, which main added after those captures were taken.

**Tests**
- `app/src/lib/powerplay.test.ts` replaces the two rows that asserted the bug ("0 min" at and after the end). The table now covers 1 min ahead ("1 min"), exactly at the end ("7 d 0 h"), 1 min past ("6 d 23 h") and 3 weeks and 2 h past ("6 d 22 h"), plus `over` for the three cases.
- There is one case in each view test for the path a real window takes: a good read at 06:59, then failed reads across 07:00. After two minutes, Powerplay shows "6 d 23 h", "Ends ..." for the next end, and "–" in place of 1,829. Now, on the preview fixture, shows "6 d 23 h left in the cycle" and no 1,829. Both failed before the round 1 fix. The round 0 versions, which handed the page an answer already past its end, failed before the first fix.
- A Now test ("a failed ask fails that block only") depended on the day it ran: it asserted 1,829, which passed only because of the roll-forward. It now checks the block's power and rank line.
- `pnpm lint`, `pnpm check` and the full `vitest run` pass (95 files, 857 tests).

**Limits**
- "Ends Thursday 07:00 UTC" names only a weekday and time, so it reads the same before and after the turnover. The view test checks it against the rolled end, but the string cannot show the difference.
- Both pages tell the cycle from the moment their last read settled, answered or failed (Powerplay's `now`, Now's `Poll.at`), so the clock moves every 2 s on Powerplay and every 5 s on Now, even while the core is silent.

### Responses

1-q0io.1: fixed in c0d3f518: Now reads `Poll.at`, set whenever a read settles, instead of a `Date.now()` cached for the block's life.
1-q0io.2: fixed in c0d3f518: Powerplay moves `now` on a failed read too, and both view tests now cross 07:00 with failing reads.

