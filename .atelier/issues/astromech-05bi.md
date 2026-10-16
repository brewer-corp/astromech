---
created_at: "2026-10-15T20:36:13.649703140+00:00"
id: "astromech-05bi"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-xf8j"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T23:04:28.398826526+00:00"
status: "done"
title: "Where to buy each colonisation commodity"
updated_at: "2026-10-16T23:04:28.398826526+00:00"
---

## Goal

Near the construction site or near me, list stations selling each remaining commodity, with supply and pad size, ranked by how many of the list they stock.

## Done when

With recorded Ardent exports, a station stocking three listed commodities ranks above a closer one stocking one.

## Context

Reuses the commodity buy lookup (Ardent exports).

## Notes

- 2026-10-16 [note] Built in 2e9bc93: astromech-market::list (stations ranked by how many of the list they stock, then nearest; per-commodity stock, price, pad; buy-lookup filters incl. min stock; unavailable/truncated commodities named), astromech-colonisation::from_journal_dir (journal replay, nothing stored; per-commander keying stays with astromech-0x68), shell colonisation_projects/colonisation_buy, Trade > Colonisation page (shopping list in loads, Where to buy from the site's system or near you, lookup filters remembered under colonisation.buy). Proof: dn8s (Done-when recorded test), bh0u, ter4 (component tests), fh6f (UI check), g6zv (shell type-check), jlgd (full gates). Open: shell compiled only on Windows CI; independent review outstanding.

## Close Reason

Recorded Ardent exports rank a station stocking three listed commodities (Efremov Plant, 8.6 ly) above a nearer one stocking one (FCX-801, 6.0 ly) (evidence dn8s); component tests (ter4), UI check (fh6f), full gates (jlgd). Independent review is the parent's gate.
