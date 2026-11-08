---
created_at: "2026-11-07T22:55:23.422386221+00:00"
id: "astromech-11vu"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Journeys: the kill switch shown on and re-armed in the page; the voice tester runs gear down live and shows it confirmed"
updated_at: "2026-11-08T00:01:28.334374757+00:00"
---

## Goal

The suite holds two journeys about what matters most while playing (strategy, Shape: copilot reliability wins over window features):
- The kill switch: the page shows it on, then off after a re-arm.
- The voice tester: running "gear down" live from the page presses the gear key, the game confirms it, and the page shows the confirmation.

The Rust world tests already cover the core's half of both (crates/astromech-core/tests/world_voice.rs:45, :93). These journeys add what the commander sees.

## Done when

- [x] Journey: the kill switch.
  - The world is worlds/kill-switch, or a copy under worlds/journeys/. Elite is running and the page is open.
  - A kill-switch press and release (world steps) makes the page's header show "Kill switch on" (app/src/lib/core.ts:92) once the page clock runs the status poll.
  - A hold past `rearm_hold_ms` (a world `advance` of 1000 ms) re-arms the kill switch, and the header no longer shows it.
  - Mutation: a 999 ms hold leaves the kill switch on, and the journey fails.
  - The denied command and the short hold stay in world_voice.rs:93's test.
- [x] Journey: the voice tester runs gear down live.
  - The world is worlds/gear-down, or a copy: Elite is in front, and a reaction sets the gear flag when its key is pressed.
  - On the Voice tester page (app/src/modules/voicetester) the commander runs "gear down" live (`voice_test_live`, app/src/lib/voice.ts:62).
  - The core presses the gear key (a `key` observation), the world's reaction sets the flag, and the page shows that the game confirmed it.
  - Mutation: with the reaction removed, the page never shows the confirmation, and the journey fails.
- [ ] Both pass in the journey suite (`node --test 'journeys/*.journey.ts'` in app/) locally, 20 runs in a row with no failure, and in CI's Linux job.

## Scope

In: the two journeys and their worlds.

Out:
- Setting the kill switch or push-to-talk button from the page: press capture needs presses in the page, which is a backlog item.
- Speech recognition from audio, which world_voice.rs covers.

## Context

- **Order.** Start once astromech-8qsx (the journey harness) is on yis2's branch. This child and the route journeys can run in parallel.
- **Kill switch display.** The page reads the kill switch from `status`, through App.svelte's 2 s poll (app/src/App.svelte:99). No shell push topic is involved, so this works through the dev bridge now.
- **World inputs.** In world mode the fake raw input, the clock and the key reactions run on every OS (astromech-1hh9's Report).
- **Voice tester elsewhere.** The release onboarding smoke runs the voice tester on Windows (scripts/onboarding_smoke.py, step 5), but only at release. This journey runs it on every push.

## Unknowns

- Whether `voice_test_live`'s verify wait needs the world clock to move. If the core waits for the gear flag on its clock (a `clock` wait with a timeout), the world needs an `advance` step after the key press for the verify to finish. Otherwise it settles on the reaction alone, as gllh's `voice_test_live` lights.on case did in every_window_command.rs. Find out first, and add the `advance` to the world if it's needed.

## Notes

- 2026-11-07 [decision] The Unknown: voice_test_live's verify needs no advance under the world clock when the game answers. On worlds/gear-down, with no step after 'running', the live run of landing_gear.down replies 'Pressed; the game confirmed it in 0 ms.' The world's reaction rewrites Status.json before the key send returns, so the verify sees the flag at once. No advance is added. With the reaction removed, the verify waits on the clock for the flag, and with no advance it never ends: the page shows nothing, which is the mutation's failure.
  Reason: Probed directly with app/dev/core.ts before writing the journey, and confirmed by the journey and its mutation.
- 2026-11-07 [decision] The kill-switch journey uses a copy, worlds/journeys/kill-switch: worlds/kill-switch's header, game folder, bindings and controls, with only the kill (press, release) and re-arm (press, a 1000 ms advance, release) steps. The voice-tester journey uses worlds/gear-down as it is, stepping only to 'running'.
  Reason: Stepping worlds/kill-switch to its re-arm mark would also run the denied command and the short hold, which the spec keeps in world_voice.rs:93's test. gear-down's later marks are never reached, so it needs no copy.
- 2026-11-07 [decision] The re-arm part waits for the page (runFor(2000), then the header without 'Kill switch on') before the core's 'Re-armed.' line. The kill part waits for 'Kill switch on. All actions stopped.' first, then checks the header hasn't changed before its poll.
  Reason: A press or release step returns once the push-to-talk loop has handled it (world.rs), so the next poll reads the new state. Waiting on the page first makes the 999 ms mutation fail where the commander would see it: the header still says Kill switch on.

## Report

Lines 1 and 2 hold and are ticked. Line 3 holds locally (20 runs in a row); its CI half waits on the PR's Linux job, so it is left unticked. Commit ad1f348b on astromech-yis2, app/journeys/voice.journey.ts.
- The Unknown: no advance is needed. On worlds/gear-down, stepped only to "running", `voice_test_live` landing_gear.down replied "Pressed; the game confirmed it in 0 ms." with no further step. The key send returns only after the world's reaction has rewritten Status.json, so the verify sees the gear flag at once. Probed with app/dev/core.ts before the journey was written (decision note).
- Line 1, the kill switch (worlds/journeys/kill-switch: worlds/kill-switch with only its kill and re-arm steps).
  - With Elite running, the page's header reads "Elite running · Voice ready".
  - The press and release step brings the said "Kill switch on. All actions stopped.". The header is unchanged until `runFor(2000)` runs the status poll, then reads "Elite running · Voice ready · Kill switch on".
  - The 1000 ms hold step, then `runFor(2000)`: the header is back to "Elite running · Voice ready", and the core said "Re-armed.".
  - Mutation, reverted: a 999 ms hold. The header wait for "Elite running · Voice ready" times out, because the header still shows "Kill switch on".
  - The denied command and the short hold aren't stepped; they stay in world_voice.rs.
- Line 2, the voice tester (worlds/gear-down as it is: Elite in front, a reaction to Key_L that sets the gear flag).
  - On #/voicetester the journey types "gear down" and clicks Test. Run live is enabled once the page's first voice poll reports test mode, and the journey clicks it.
  - The page shows "Pressed; the game confirmed it in 0 ms.", and the core's key observations are exactly Key_L down, then up.
  - Mutation, reverted: the Key_L reaction removed from worlds/gear-down's header. The verify waits on the world clock for the flag, and with no advance it never ends. The wait for the confirmation times out. The harness still quit the core with exit 0 and its temp world gone.
- Line 3, locally: `node --test 'journeys/*.journey.ts'` runs all seven tests.
  - Node 26: 20/20 in a row, 3.8-6.0 s a run by node:test.
  - Node 22.23 (`npx node@22`): 10/10, 4.4-5.4 s a run.
  - After the loops: no world core, world home or playwright-tracing folder left.
- Other checks: `pnpm -s lint` clean, `pnpm -s check` 0 errors, `pnpm -s test` 563 passed, `pnpm exec tsc -p journeys` 0, `cargo test -p astromech-core --test committed_worlds` 2 passed (the new world). No harness change.

### Decisions
Three decision notes:
- the Unknown, with no advance needed;
- the kill-switch copy with only its kill and re-arm steps, and gear-down used as it is;
- the page waited on first for the re-arm, so the mutation fails where the commander would see it.

### Surprises
- With no reaction, a live run never ends under the world's manual clock: the verify's timeout is on that clock. So "Pressed, but the game didn't show the change." can only be reached in a world with an `advance` past the verify's timeout. No journey needs it.

### Open items
- Line 3's CI half: the PR's Linux job.
- Review is outstanding.
