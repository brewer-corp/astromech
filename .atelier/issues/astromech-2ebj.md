---
created_at: "2026-11-07T21:35:37.796350656+00:00"
id: "astromech-2ebj"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-zgoo"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "A session's assistant judges market and station data age on the session's clock"
updated_at: "2026-11-07T21:47:31.178350647+00:00"
---

## Goal

In a world, the voice answers that weigh how old market and station data is (trade, commodity and station lookups in assistant.rs, `self.clock.now_s()`) read the world's clock, as the rest of the session does.

## Done when

- [ ] `Session::start` builds its `Assistant` with the session's clock (`Assistant::with_clock`), and a world test shows a lookup's age follows the world's clock (an `advance` past the age limit changes the answer).

## Context

Found in astromech-zgoo: `Assistant::new` defaults its clock to `SystemClock` (session.rs:375 never calls `with_clock`), so in world mode those answers use real time while the session runs on a `ManualClock`. The installed app is unaffected (both are the system clock).

## Notes

- 2026-11-07 [note] From finding astromech-dj7j (zgoo review): the Assistant's confirm window now reads the policy's clock while its own clock field (SystemClock by default, never set by Session::start) still judges data age; when with_clock is wired in, Assistant::now should read that one clock.
