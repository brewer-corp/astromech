---
schema: atelier.review
schema_version: 1
id: astromech-oixz
title: 'R3 [medium] edsm/mod.rs answer(): HTTP 5xx, 408/429 and request-level msgnum 5xx are still Hold, which never gives up, so a batch EDSM deterministically fails with a 500 (a poison event) still blocks the queue forever. 3pkl asked for ''any non-200 status''. Exempting outages is a reasonable trade-off against data loss in a long outage, but it should be a recorded decision (e.g. give up 5xx after a longer limit, or split to isolate the bad event).'
target: astromech-ky3d
identity: review-agent (opus, independent)
commit: 769a9dea096cd5fb96e00ce173c87c92fc0134b2
created_at: 2026-10-17T05:37:39.505647606Z
event:
  event: finding
  recommendation: defer
---

R3 [medium] edsm/mod.rs answer(): HTTP 5xx, 408/429 and request-level msgnum 5xx are still Hold, which never gives up, so a batch EDSM deterministically fails with a 500 (a poison event) still blocks the queue forever. 3pkl asked for 'any non-200 status'. Exempting outages is a reasonable trade-off against data loss in a long outage, but it should be a recorded decision (e.g. give up 5xx after a longer limit, or split to isolate the bad event).
