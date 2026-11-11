---
schema: atelier.item
schema_version: 3
id: astromech-2ebj
title: A session's assistant judges market and station data age on the session's clock
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T21:35:37.796350656Z
---

## Goal

In a world, the voice answers that weigh how old market and station data is (trade, commodity and station lookups in assistant.rs, `self.clock.now_s()`) read the world's clock, as the rest of the session does.

## Done when

- [ ] `Session::start` builds its `Assistant` with the session's clock (`Assistant::with_clock`), and a world test shows a lookup's age follows the world's clock (an `advance` past the age limit changes the answer).

## Context

Found in astromech-zgoo: `Assistant::new` defaults its clock to `SystemClock` (session.rs:375 never calls `with_clock`), so in world mode those answers use real time while the session runs on a `ManualClock`. The installed app is unaffected (both are the system clock).

## Notes

- 2026-11-07 [note] From finding astromech-dj7j (zgoo review): the Assistant's confirm window now reads the policy's clock while its own clock field (SystemClock by default, never set by Session::start) still judges data age; when with_clock is wired in, Assistant::now should read that one clock.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-zgoo (Clippy rejects direct time reads and sleeps; the remaining reads move to the clock)
