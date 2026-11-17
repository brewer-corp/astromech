---
schema: atelier.item
schema_version: 3
id: astromech-unak
title: Powerplay cycle time reads '0 h 0 min left' once the recorded cycle end has passed
status: building
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

- [ ] A failing test first: with `week_ends` in the past, the time left counts to the next Thursday 07:00 UTC after now, on Now and on Powerplay, and "Ends ..." names that moment. A table test covers an end one minute ahead, one minute past, and three weeks past.
- [ ] Merits "this cycle" from a cycle that has ended are not shown as this cycle's: the page says they are last cycle's, or leaves them out, and the Report says which and why.
- [ ] The browser preview and the capture tool run on a fixed clock for these pages, so a capture does not change with the day it is taken; the Now and Powerplay captures are retaken.

## Scope

The cycle clock on Now and Powerplay, and the preview's clock for their captures.
