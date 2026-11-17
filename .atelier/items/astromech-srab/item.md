---
schema: atelier.item
schema_version: 3
id: astromech-srab
title: 'Finder rows: station names line up with or without a type icon, and folded units keep their width'
status: building
mission: astromech-xajc
labels:
- feature
- size-s
priority: medium
blocked_by:
- astromech-fom5
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A3d33278c-7ce4-4e5f-9607-0d81962341c1
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T09:00:09.190860084Z
trivial: null
caused_by: null
created_at: 2026-11-17T08:59:59.859181066Z
---

## Goal

Taken from deferred finding astromech-3ca6/1-79uh.2, in .atelier/items/astromech-3ca6/reviews/1-79uh.md:

> Finding 2: Reserve the type-icon space so station names align
> Recommendation: defer
>
> The Report's alignment caveat is reproducible. `app/src/lib/ui/SearchKit.svelte:485` conditionally renders the station kind without reserving its space, and `:591-592` removes the system line's indent when the icon is missing. On Sell anywhere at 1100 x 700 and 100 percent, FCX-800's station and system begin at x=217, while SavorgnandeBrazza Legacy's and Asgaard's begin at x=193. Both lines therefore shift by 24 px between rows in the same group, contrary to DESIGN.md's Aligned Columns Rule. This is a cosmetic inconsistency rather than a loss of facts and can follow separately. Reserve a consistent icon slot and indent within the shared place group instead of letting optional source metadata move the text.

## Done when

- [ ] The place group reserves one type-icon slot on every row, so station and system names start at the same x whether or not a row has an icon; a browser test on Sell anywhere (carriers mixed with stations) checks it.
- [ ] On a narrow row (Colonisation at 1100 x 700 and at 145 percent) long stock text is what shrinks or wraps; the trip's units and the report's age keep their width (finding astromech-3ca6/2-ppw4.2).
- [ ] Captures of Sell anywhere and Colonisation at 1100 x 700, before and after.

## Scope
