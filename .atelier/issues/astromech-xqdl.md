---
created_at: "2026-10-15T20:36:13.661401872+00:00"
id: "astromech-xqdl"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-05bi"
  - kind: "issue"
    id: "astromech-1per"
  - kind: "issue"
    id: "astromech-d1lo"
  - kind: "issue"
    id: "astromech-mcxc"
  - kind: "issue"
    id: "astromech-qw2m"
  - kind: "issue"
    id: "astromech-sd56"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:40:50.490881492+00:00"
status: "done"
title: "Where to sell or buy a commodity near me"
updated_at: "2026-10-16T20:40:50.490881492+00:00"
---

## Goal

Best sell price within N ly, with demand, pad size and data age; and the lowest buy price, with supply.

## Done when

Recorded-response tests return the best price within the distance and filters, and exclude stations whose demand or supply is below what the user asked for.

## Context

Ardent `system/{s}/commodity/name/{c}/nearby/imports` and `.../exports`.

## Notes

- 2026-10-16 [note] Built in 391bf16: astromech-market (prices/search: list of commodity symbols in, ranked Prices out; reusable for astromech-05bi), commodity catalogue data/catalog/commodities.json (157 ordinary commodities, no rares), search-kit radius/volume filters, Trade tab > Commodities page, shell command trade_search, voice trade.sell/trade.buy with a {commodity} grammar slot answered in astromech-core with the page's remembered filters. Proof: evidence kwq2 (recorded lookups), f0n2 (voice phrases), re3e (UI), cqrg (full checks). Open: shell compiled only on Windows CI; independent review outstanding.

## Close Reason

Recorded-response tests return the best price within the distance and filters and exclude markets with less demand or stock than asked (evidence kwq2), voice phrase tests (f0n2), UI check (re3e), full checks (cqrg). Independent review is the parent's gate.
