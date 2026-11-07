---
schema: atelier.review
schema_version: 2
id: astromech-96mk
title: finding on astromech-yis2 @910de95e
target: astromech-yis2
identity: claude-reviewer-8qsx
commit: 910de95ecd1c657460e6903294cae3f7e4a762db
created_at: 2026-11-07T23:25:25.225299717Z
event:
  event: finding
  recommendation: defer
---

8qsx R3 (app/eslint-rules/tests.js no-playwright-time): Done when line 4 holds, but line 2's 'runFor only' rests on review alone. A journey calling page.clock.resume(), setSystemTime, fastForward or setFixedTime lints clean, and so does page.waitForLoadState('networkidle'), which waits on a 500 ms quiet timer (checked via eslint --stdin as journeys/m.journey.ts: only waitForTimeout and the wall-clock reads were caught). no-wall-clock's message also tells journeys to use vi.useFakeTimers. Follow-up: reject those clock methods outside harness.ts and 'networkidle' in journeys, with a journey-worded message.
