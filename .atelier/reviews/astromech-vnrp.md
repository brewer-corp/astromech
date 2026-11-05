---
schema: atelier.review
schema_version: 1
id: astromech-vnrp
title: finding on astromech-dfvp @84b18f26
target: astromech-dfvp
identity: reviewer-fable-dfvp
commit: 84b18f26398d85ca5984acce5abcc1a316f7281d
created_at: 2026-11-05T17:20:01.726217441Z
event:
  event: finding
  recommendation: defer
---

medium: app/src/modules/unlocks/View.svelte:138,142 The glyph rules '.badge, .outline, .barred, .mail { width: 12px; height: 14px }' and '.barred { border; stripe gradient }' are unscoped, and the card is class="eng {n.status}", so a barred engineer's whole <article> matches: it collapses to a 12x14 px striped box (verified on the dev build by adding 'barred' to a card: background transparent, 1 px border, 12x14). Pre-existing on main (the sizing rule) and a real path (crates/astromech-engineering/src/unlocks.rs maps journal 'barred' to Status::Barred); the branch's rewrite of .barred adds a border and drops the hull but does not cause the collapse. Fix in a follow-up (or here, two lines): scope the glyph rules to '.access > .barred' etc., or rename the card state classes.
