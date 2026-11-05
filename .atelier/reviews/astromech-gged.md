---
schema: atelier.review
schema_version: 1
id: astromech-gged
title: finding on astromech-dzwy @379c96dc
target: astromech-dzwy
identity: reviewer-sonnet-dzwy
commit: 379c96dc4e15349b9b42017ada4a1bda6814cdbb
created_at: 2026-11-05T18:05:23.952690896Z
event:
  event: finding
  recommendation: fix_now
---

medium: DESIGN.md:470 The Tabular Rule says every number in a column, tile, cell or strip is right-aligned, but the branch (correctly, per decisions) leaves tile values (Home, Session, Exploration, Mining), label/value fact-list numbers (Powerplay, Data sharing, Station From the star, pads, distance) and date columns (Logbook First/Last, Station Seen, Diagnostics time) left-aligned. Rule and code now contradict, and the next agent will either right-align them or flag them. Reword: quantities in a table column (counts, prices, distances) right-align; a tile's value follows its tile spec; fact-list values, dates and numbers inside a sentence stay with their text; tabular figures apply to all of them.
