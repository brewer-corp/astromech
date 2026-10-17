---
schema: atelier.review
schema_version: 1
id: astromech-3pkl
title: 'R3 [low] crates/astromech-eddn/src/edsm/mod.rs:240-258,374: Answer::Hold never gives up. Any non-200 HTTP status (for example a 413 for a 50-event batch of large Loadout/Materials events), an unreadable body, or an events list whose length is not n holds the whole batch and every due row. The queue is sent oldest first, so a request that fails the same way every time blocks all EDSM upload indefinitely (bounded only by the outbox cap). The per-event-count assumption is already queued for live checking in astromech-shi5. Suggest: after MAX_ATTEMPTS, give up on (or halve) a batch that is held for a non-5xx, non-transport reason.'
target: astromech-ah0i
identity: review-agent (opus, independent)
commit: aa58b47beb073e878c106e16878529bca17d43f1
created_at: 2026-10-17T05:04:00.386315096Z
event:
  event: finding
  recommendation: defer
---

R3 [low] crates/astromech-eddn/src/edsm/mod.rs:240-258,374: Answer::Hold never gives up. Any non-200 HTTP status (for example a 413 for a 50-event batch of large Loadout/Materials events), an unreadable body, or an events list whose length is not n holds the whole batch and every due row. The queue is sent oldest first, so a request that fails the same way every time blocks all EDSM upload indefinitely (bounded only by the outbox cap). The per-event-count assumption is already queued for live checking in astromech-shi5. Suggest: after MAX_ATTEMPTS, give up on (or halve) a batch that is held for a non-5xx, non-transport reason.
