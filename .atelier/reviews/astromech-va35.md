---
schema: atelier.review
schema_version: 1
id: astromech-va35
title: finding on astromech-dfvp @84b18f26
target: astromech-dfvp
identity: reviewer-fable-dfvp
commit: 84b18f26398d85ca5984acce5abcc1a316f7281d
created_at: 2026-11-05T17:20:01.933211071Z
event:
  event: finding
  recommendation: defer
---

low: DESIGN.md:578 vs app/src/modules/engineering/Specs.svelte:53 The new Multi-value stat pattern says the label names the values in order with '/' ('Jump min / current / max') and '|' separates the values; the Specs band's first stat is labelled 'Speed (top | boost)', using the value separator in the label while the two labels beside it use '/'. One-word fix ('Speed top / boost'); not blocking.
