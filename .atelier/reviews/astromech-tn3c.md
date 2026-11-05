---
schema: atelier.review
schema_version: 1
id: astromech-tn3c
title: finding on astromech-dzwy @379c96dc
target: astromech-dzwy
identity: reviewer-sonnet-dzwy
commit: 379c96dc4e15349b9b42017ada4a1bda6814cdbb
created_at: 2026-11-05T18:05:24.202132972Z
event:
  event: finding
  recommendation: fix_now
---

low: app/src/modules/engineering/Specs.svelte:~.q The .q rule (18px, Value L) lost the mono face's tight line box and picks up Saira's natural line height, making the Ships specs block about 9 px taller, contradicting the no vertical moves claim. DESIGN.md typography value-l sets line-height 1.2; add line-height: 1.2 to .q.
