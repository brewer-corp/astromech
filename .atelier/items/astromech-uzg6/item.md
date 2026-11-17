---
schema: atelier.item
schema_version: 3
id: astromech-uzg6
title: Seven finders ask their source twice when opened from a link
status: ready
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T09:00:09.190860084Z
trivial: null
caused_by: null
created_at: 2026-11-17T08:59:59.893012974Z
---

## Goal

Opening a finder from a link asks its source once. astromech-3ca6's implementer found that Sell and Buy searched twice when opened from a link, fixed those two, and reported that Outfitting, Services, Shipyard, Material traders, Raw sites, Faction states and Mining hotspots probably do the same. Each extra search is a request to Spansh or Ardent that the commander did not make.

## Done when

- [ ] A failing browser test first: each of the seven finders, opened from a link that carries its query, sends one request on the fake transport, not two.
- [ ] The cause is fixed where the finders share it (the fix astromech-3ca6 made for Sell and Buy shows where), not finder by finder.
- [ ] The Report names the commit that introduced the double search, for `caused_by`.

## Scope

The finders' first search. The cause predates this item's filing and has not been traced to a commit; the third line traces it.
