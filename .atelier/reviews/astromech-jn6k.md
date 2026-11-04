---
schema: atelier.review
schema_version: 1
id: astromech-jn6k
title: finding on astromech-t34c @75b3bfe8
target: astromech-t34c
identity: review-agent-t34c-1
commit: 75b3bfe8704fc0dc1fb74864bd7c7f73a4f93ba9
created_at: 2026-11-04T23:55:08.645520548Z
event:
  event: finding
  recommendation: no_action
---

low: crates/astromech-core/src/pipeline.rs:127-137 (puak) One-sided freshness (at most 120 s old, at most 5 s ahead) assumes the game stamps lines with this PC's clock, which it does (same machine, same clock, as the header-name check relies on), so a PC's steady drift doesn't matter: now and the stamps move together. The only new loss is a backward step of this PC's clock by more than 5 s (an NTP step correction): for that many seconds the game's just-written lines look future-stamped, so a Gated batch read then is not live and being_written is false. It fails closed and the window is seconds. No action unless real clocks are seen stepping; AHEAD_SECS of 30 or so would absorb it without reopening the puak window for clocks minutes ahead.
