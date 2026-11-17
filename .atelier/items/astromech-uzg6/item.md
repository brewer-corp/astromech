---
schema: atelier.item
schema_version: 3
id: astromech-uzg6
title: Seven finders ask their source twice when opened from a link
status: in_review
mission: astromech-xajc
labels:
- bug
- size-s
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Aba535992-dad7-4912-a239-fbe8b0bde061
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

- [x] A failing browser test first: each of the seven finders, opened from a link that carries its query, sends one request on the fake transport, not two.
- [x] The cause is fixed where the finders share it (the fix astromech-3ca6 made for Sell and Buy shows where), not finder by finder.
- [x] The Report names the commit that introduced the double search, for `caused_by`.

## Scope

The finders' first search. The cause predates this item's filing and has not been traced to a commit; the third line traces it.

## Report

**The premise holds for two of the seven, not seven.** Outfitting and Mining hotspots asked
Spansh twice; Services, Shipyard, Material traders, Raw sites and Faction states already asked
once. Those five keep their query as a string, and Svelte does not re-run anything when a
`$state` is set to an equal string. Outfitting and Mining hotspots keep an object, and their
hashchange handler replaced it with an equal new one, which handed SearchKit a new `run`.

**The double search the commander actually hit was a pick, not a link.** Opening a finder from a
link inside the app asks once: Places (`View.svelte`) mounts the page from within the hashchange
dispatch, and a listener added during a dispatch does not hear it. I checked this through `View`
for Outfitting, Mining hotspots and Services (one request each). But a pick on the page (a class,
rating, mount, module, mineral, ring type or reserve) sets the query and then the hash, and the
page's own listener then replaced the query: two requests per pick (Outfitting went from 1 to 3
requests on one Rating click). The link case fails only when a page mounts before the hash change
that named it lands, which is how astromech-3ca6's Market test and this item's test open a page.

**Tests first** (`70889555`, red): `app/src/modules/places/FinderLinks.browser.test.ts` opens each
of the seven from a link that names a non-default query and counts requests on the fake
transport, and adds a pick case for Outfitting and Mining hotspots. Before the fix: four red with
two requests each (both link cases, both pick cases), five green. After: all nine green.

**The fix** (`55830e8e`): the eight finder pages each carried a copy of the hashchange handler
and the address rewrite. `app/src/lib/finderAddress.svelte.ts` `followAddress` is now the only
copy, and it replaces the query only when its route differs (Market's compare from `0815325c`,
moved and generalised). Comparing canonical routes rather than raw hashes keeps Outfitting's
older `/5A/Fixed` links and its size settling from counting as a change. Every finder page,
Market included, calls it; Market's local compare is gone. Back and forward to another query still
search once (the existing Outfitting, Hotspots, Market and Places tests pass, and the pick cases
pin the new query reaching the search).

**caused_by: `438e48d6`** ("Open a module's nearby outfitting search from the command palette",
2026-10-16). It added the hashchange handler `want = parseModuleRoute(location.hash) ?? DEFAULT`
to the then `modules/outfitting/View.svelte`, whose `pick` already set `want` and now also set
the hash. Mining hotspots copied the pattern in `0b6806b9` (astromech-f7i0) and Sell and Buy in
Market. I traced this from the diff; I did not run a test at that commit, because the fake
transport (`app/src/test/transport.ts`) did not exist yet. I could not set `caused_by` from the
item branch (`atelier set` refuses it there), so the lead needs to set it on `main`.

**Checks:** `pnpm test` gives 736 of 737 passing. The failure is `App.browser.test.ts` "docks from
the dock width, where the Sell finder keeps its panel beside its rows, at every Text size"
(`100 %: below the rows at 1430 px`). It fails the same way on this branch with my change stashed,
so it was already failing before this change. It needs an item; I could not file one from the item
branch. `pnpm run check` and `pnpm run lint` are clean.

