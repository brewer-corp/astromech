---
schema: atelier.review
schema_version: 1
id: astromech-uuqh
title: 'Low, crates/astromech-eddn/src/outbox.rs:366 (Outbox::due ORDER BY refusals > 0, id): an event refused once now goes after every newer due event, not only while it backs off. Scenario: a good FSDJump gets one single-event 5xx, then later jumps are stored before it. main already let newer events overtake a refused event during its back-off, so this widens existing reordering; the transient fields are fixed at queue time and each event carries its own timestamp. Couldn''t confirm from EDSM''s docs (Cloudflare) whether EDSM orders the commander''s history by timestamp or by arrival. If Sumeet sees a wrong last position on EDSM after a refusal, look here first.'
target: astromech-ux0h
identity: review-agent-ux0h-1
commit: fa52c077d4e9350a5872eee56189099ec1c3ece3
created_at: 2026-10-22T21:07:45.696227815Z
event:
  event: finding
  recommendation: no_action
---

Low, crates/astromech-eddn/src/outbox.rs:366 (Outbox::due ORDER BY refusals > 0, id): an event refused once now goes after every newer due event, not only while it backs off. Scenario: a good FSDJump gets one single-event 5xx, then later jumps are stored before it. main already let newer events overtake a refused event during its back-off, so this widens existing reordering; the transient fields are fixed at queue time and each event carries its own timestamp. Couldn't confirm from EDSM's docs (Cloudflare) whether EDSM orders the commander's history by timestamp or by arrival. If Sumeet sees a wrong last position on EDSM after a refusal, look here first.
