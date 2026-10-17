---
schema: atelier.review
schema_version: 1
id: astromech-1ca1
title: 'R1 [low] crates/astromech-eddn/src/outbox.rs Outbox::init (by inspection): user_version is read outside the migration transaction and bumped after COMMIT in a separate statement. Two connections opening a v1 file at once (Feed::new and the app.rs discard/log opens) can both run v2; the second waits on the write lock, then rebuilds the table again. Rows survive, but the rebuilt table''s sqlite_sequence restarts at the max surviving id, so ids purged between the two runs could be reused once. Window is milliseconds on the first start after upgrade. Fix: BEGIN IMMEDIATE, re-read user_version inside it, and set PRAGMA user_version inside the same transaction (also closes the pre-existing v1 ''table exists'' race). Recommendation: DEFER to a small follow-up.'
target: astromech-8ozh
identity: review-agent (opus, independent)
commit: 2b8bc7d84e35fe62a6797df2e98da1e7c47aae55
created_at: 2026-10-17T04:55:06.302888751Z
event:
  event: finding
  recommendation: defer
---

R1 [low] crates/astromech-eddn/src/outbox.rs Outbox::init (by inspection): user_version is read outside the migration transaction and bumped after COMMIT in a separate statement. Two connections opening a v1 file at once (Feed::new and the app.rs discard/log opens) can both run v2; the second waits on the write lock, then rebuilds the table again. Rows survive, but the rebuilt table's sqlite_sequence restarts at the max surviving id, so ids purged between the two runs could be reused once. Window is milliseconds on the first start after upgrade. Fix: BEGIN IMMEDIATE, re-read user_version inside it, and set PRAGMA user_version inside the same transaction (also closes the pre-existing v1 'table exists' race). Recommendation: DEFER to a small follow-up.
