---
schema: atelier.review
schema_version: 1
id: astromech-7r5k
title: finding on astromech-kwoi @b59bfe06
target: astromech-kwoi
identity: reviewer-fable-kwoi
commit: b59bfe06cbc77259f52c02a6a4adae596a989658
created_at: 2026-11-05T15:10:06.639510587Z
event:
  event: finding
  recommendation: defer
---

low: docs/V1-DESIGN.md:194 the page table's 'Trade: Commodities' row still describes Sell, Buy and My cargo as modes of that page with their filters, and rows 196-198 still call Services, Outfitting and Shipyard 'Galaxy:' pages (stale since astromech-nxz9; the branch never touched V1-DESIGN.md). An agent reading the v1 scope doc is told the lookups live on Trade. One pass for the parent before astromech-kyqw validates: rewrite those rows as Places finders and add a Places row; not kwoi's Done when (docs/guide only).
