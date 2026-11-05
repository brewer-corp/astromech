---
schema: atelier.review
schema_version: 1
id: astromech-d3vj
title: finding on astromech-51ue @249bf97a
target: astromech-51ue
identity: reviewer-fable-51ue-final
commit: 249bf97a81ab3640206b4a73b02f7eeaf70a95de
created_at: 2026-11-05T16:09:28.409982281Z
event:
  event: finding
  recommendation: defer
---

low: docs/V1-DESIGN.md:197 the 'Exploration: System' row still says Where to sell is 'the nearest Universal Cartographics or Vista Genomics on the search kit (the Services lookup and its filters)'; since astromech-kwoi (modules/exploration/View.svelte) they are two links into the Services finder, and the embedded search kit is gone. The 2hbw pass (7r5k) rewrote the Trade and Galaxy rows but not this one. An agent reading the v1 scope is told Exploration embeds a lookup. Fix: reword to 'links into the Services finder (Universal Cartographics, Vista Genomics)'; docs only.
