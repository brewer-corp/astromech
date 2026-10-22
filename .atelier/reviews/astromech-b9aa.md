---
schema: atelier.review
schema_version: 1
id: astromech-b9aa
title: 'R5 [low] crates/astromech-data/src/edsm.rs:112 with crates/astromech-data/src/cache.rs:83: the cache stores each row''s expiry at write time, so edsm_bodies rows written by the previous version (7-day ttl::SYSTEM) keep their old expiry after upgrade. For up to a week after updating, a system looked up in the week before still returns its old count on the hourly refresh, so yfje''s fix lands gradually rather than at once. Self-healing, one-time, and only affects freshness of a count; no data-integrity or replay effect. No change needed (a migration to clamp old rows would cost more than it saves).'
target: astromech-g3qt
identity: review-agent-g3qt-2
commit: b72a76cb2a16a77adc1bd664ddd800829e2cb69b
created_at: 2026-10-22T21:17:06.383922907Z
event:
  event: finding
  recommendation: no_action
---

R5 [low] crates/astromech-data/src/edsm.rs:112 with crates/astromech-data/src/cache.rs:83: the cache stores each row's expiry at write time, so edsm_bodies rows written by the previous version (7-day ttl::SYSTEM) keep their old expiry after upgrade. For up to a week after updating, a system looked up in the week before still returns its old count on the hourly refresh, so yfje's fix lands gradually rather than at once. Self-healing, one-time, and only affects freshness of a count; no data-integrity or replay effect. No change needed (a migration to clamp old rows would cost more than it saves).
