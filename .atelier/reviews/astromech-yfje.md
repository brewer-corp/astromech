---
schema: atelier.review
schema_version: 1
id: astromech-yfje
title: 'R1 [medium] crates/astromech-core/src/background.rs:30 with crates/astromech-data/src/edsm.rs:112: astromech-54q6''s Goal (an answer that goes stale over a long session is refreshed) is met for known biology (Spansh search, ttl::SEARCH 1 h) but not for bodies known to EDSM. The hour-old Done answer is dropped, but the new lookup goes through edsm_bodies, which caches for ttl::SYSTEM (7 days), so it gets back the same count. The record''s Context names exactly this case (on top of the client cache, EDSM a week). Scenario: arrive in a system, leave, others scan it, come back 2 h later in the same session: the callout still uses the old count. Fix: a shorter TTL for the known-bodies lookup (e.g. ttl::SEARCH, or a TTL parameter on edsm_bodies used only here), with a test; or have Sumeet re-scope the Goal in the record. The new background.rs doc admits the gap, but that doesn''t close it.'
target: astromech-g3qt
identity: review-agent-g3qt-1
commit: 98e6827717c46e9692d70b475f2300e4eb9f4b51
created_at: 2026-10-22T21:06:25.791847237Z
event:
  event: finding
  recommendation: fix_now
---

R1 [medium] crates/astromech-core/src/background.rs:30 with crates/astromech-data/src/edsm.rs:112: astromech-54q6's Goal (an answer that goes stale over a long session is refreshed) is met for known biology (Spansh search, ttl::SEARCH 1 h) but not for bodies known to EDSM. The hour-old Done answer is dropped, but the new lookup goes through edsm_bodies, which caches for ttl::SYSTEM (7 days), so it gets back the same count. The record's Context names exactly this case (on top of the client cache, EDSM a week). Scenario: arrive in a system, leave, others scan it, come back 2 h later in the same session: the callout still uses the old count. Fix: a shorter TTL for the known-bodies lookup (e.g. ttl::SEARCH, or a TTL parameter on edsm_bodies used only here), with a test; or have Sumeet re-scope the Goal in the record. The new background.rs doc admits the gap, but that doesn't close it.
