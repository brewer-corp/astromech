---
schema: atelier.review
schema_version: 1
id: astromech-k5sa
title: finding on astromech-nxz9 @326fd18e
target: astromech-nxz9
identity: reviewer-fable-nxz9
commit: 326fd18eebaf7994530bf4843f8fa9d107921807
created_at: 2026-11-05T08:46:20.844590101Z
event:
  event: finding
  recommendation: no_action
---

info: app/src/modules/places/View.svelte:21 #/places/<unknown finder> shows the home but keeps the bad address (#/places/nope), while an unknown input on a known finder is normalised to its default (#/places/services/nope -> interstellar_factors). No consequence: the next tile click sets a good address; noted for consistency only.
