---
schema: atelier.review
schema_version: 1
id: astromech-tf5w
title: 'R1: transport.rs body.truncate(500) panics mid-char; the panic escapes deliver_due before the row is updated, jamming the outbox.'
target: astromech-tyjz
identity: review-agent (tyjz round 1, el1v)
commit: 2f33d41b7b529f5ca666b41e263eec59a10eb002
created_at: 2026-10-16T18:48:43.002019302Z
event:
  event: finding
  recommendation: fix_now
---

R1: transport.rs body.truncate(500) panics mid-char; the panic escapes deliver_due before the row is updated, jamming the outbox.
