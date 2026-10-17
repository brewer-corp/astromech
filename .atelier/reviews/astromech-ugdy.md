---
schema: atelier.review
schema_version: 1
id: astromech-ugdy
title: 'R4 [low] edsm/mod.rs Refused arm: a refused request gives up the whole batch (up to 50 events) rather than splitting it; for 413 (too large) or a wrong-length answer caused by one bad event, halving the batch would deliver the rest. The issue allowed ''give up on or split'', so not a contract miss; worth a follow-up.'
target: astromech-ky3d
identity: review-agent (opus, independent)
commit: 769a9dea096cd5fb96e00ce173c87c92fc0134b2
created_at: 2026-10-17T05:37:39.594480618Z
event:
  event: finding
  recommendation: defer
---

R4 [low] edsm/mod.rs Refused arm: a refused request gives up the whole batch (up to 50 events) rather than splitting it; for 413 (too large) or a wrong-length answer caused by one bad event, halving the batch would deliver the rest. The issue allowed 'give up on or split', so not a contract miss; worth a follow-up.
