---
schema: atelier.review
schema_version: 1
id: astromech-og62
title: 'low: app/src/lib/palette.ts:27 a query with no letters or digits (e.g. ''+'' or ''?'') gives qw=[], so qw.every is vacuously true and every item scores 2; the palette lists everything as a match. Return null when qw is empty.'
target: astromech-2u1x
identity: review-agent (2u1x review, sonnet)
commit: 75aae4400dc5ad652ba7f629a31f99ad41e2cfbd
created_at: 2026-10-16T22:31:24.105126588Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/palette.ts:27 a query with no letters or digits (e.g. '+' or '?') gives qw=[], so qw.every is vacuously true and every item scores 2; the palette lists everything as a match. Return null when qw is empty.
