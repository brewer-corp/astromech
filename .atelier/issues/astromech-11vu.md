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
updated_at: "2026-11-07T23:56:02.925224932+00:00"
---

## Goal

The suite holds two journeys about what matters most while playing (strategy, Shape: copilot reliability wins over window features):
- The kill switch: the page shows it on, then off after a re-arm.
- The voice tester: running "gear down" live from the page presses the gear key, the game confirms it, and the page shows the confirmation.

The Rust world tests already cover the core's half of both (crates/astromech-core/tests/world_voice.rs:45, :93). These journeys add what the commander sees.

## Done when

- [ ] Journey: the kill switch.
  - The world is worlds/kill-switch, or a copy under worlds/journeys/. Elite is running and the page is open.
  - A kill-switch press and release (world steps) makes the page's header show "Kill switch on" (app/src/lib/core.ts:92) once the page clock runs the status poll.
  - A hold past `rearm_hold_ms` (a world `advance` of 1000 ms) re-arms the kill switch, and the header no longer shows it.
  - Mutation: a 999 ms hold leaves the kill switch on, and the journey fails.
  - The denied command and the short hold stay in world_voice.rs:93's test.
- [ ] Journey: the voice tester runs gear down live.
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
