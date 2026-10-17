---
schema: atelier.review
schema_version: 1
id: astromech-x5ci
title: 'crates/astromech-core/src/history.rs:154-158, 167-169, 226: no test covers the stop flag. With both stop checks removed (mutation), every test still passes, because Follower::stop drops the poke sender and the watcher, and the thread ends on Disconnected. The flag only skips one more import of the old folder when a notification is already queued. That is harmless, so this is noted only.'
target: astromech-5t34
identity: review-agent (5t34 review)
commit: cdaba6007cd054cf1d165f344050e991005f5ea5
created_at: 2026-10-17T00:09:00.712531535Z
event:
  event: finding
  recommendation: no_action
---

crates/astromech-core/src/history.rs:154-158, 167-169, 226: no test covers the stop flag. With both stop checks removed (mutation), every test still passes, because Follower::stop drops the poke sender and the watcher, and the thread ends on Disconnected. The flag only skips one more import of the old folder when a notification is already queued. That is harmless, so this is noted only.
