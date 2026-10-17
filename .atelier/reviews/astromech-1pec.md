---
schema: atelier.review
schema_version: 1
id: astromech-1pec
title: 'Dead merge in SystemFold::absorb: body_count and all_found are taken from the earlier fold, but system_lines never selects FSSDiscoveryScan or FSSAllBodiesFound, so earlier.body_count is always None and all_found false. Either select those lines (the honk count on a return would be useful) or drop the two lines and the doc wording.'
target: astromech-0n4h
identity: review-agent (opus, independent)
commit: e1a1d87ea2d901d8c55dd406f77c768b1fe467a0
created_at: 2026-10-17T08:00:42.188776993Z
event:
  event: finding
  recommendation: defer
---

Dead merge in SystemFold::absorb: body_count and all_found are taken from the earlier fold, but system_lines never selects FSSDiscoveryScan or FSSAllBodiesFound, so earlier.body_count is always None and all_found false. Either select those lines (the honk count on a return would be useful) or drop the two lines and the doc wording.
