---
schema: atelier.item
schema_version: 3
id: astromech-3ca6
title: 'Finder results read as results: grouped two-line rows, demand against your hold'
status: ready
mission: astromech-xajc
labels:
- size-l
- feature
priority: high
blocked_by:
- astromech-3kf5
implementers: []
ready:
  by: Sumeet
  at: 2026-11-17T05:23:28.056709303Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.425029229Z
---

## Goal

A finder result reads as a result, not a spreadsheet row with a column per field. Sumeet, 17 Nov 2026: "table carcinisation again"; price belongs with demand, and a station with its distance from the star. Demand is coloured by whether it will take the commander's cargo.

## Done when

- [ ] The shared lookup result row is two lines in three groups, as `tone5-sell-2560.png`: the ranked value over what qualifies it; the place (station, pad, arrival distance) over its system with copy; the trip (jumps over light years). Row actions stay at the row's end. Facts the commander does not choose a row by, such as Updated, move to the side panel.
- [ ] Sell and Buy show demand or supply under the price with the market's three bars and DESIGN.md's colours, measured for Sell against the tonnes of the searched commodity aboard that are the commander's to sell, by the rules `crates/astromech-market/src/cargo.rs` already applies (mission cargo and stolen goods are left out): short below that figure, tight below five times it, plenty above. For Buy, supply is measured against the free cargo space, including with an empty hold. The page states the figure it measures against ("200 t of Painite aboard", "136 t free"). Sell with none of the commodity aboard, and either finder when the cargo or capacity is not known (Elite not running), shows the bars with no warning colour; a known zero and an unknown are told apart. The figures come from `ship_status`, which gains the sellable tonnes per commodity and the free space; `trade_search` carries no inventory. A Tip on a short row says what happens. The thresholds are constants with a test at each edge, plus a mixed hold (300 t of one commodity, 200 t of the searched one: a 250 t buyer is not short) and Buy with an empty hold, on the recorded preview world.
- [ ] A pad too small for the current ship is marked and struck through; a report older than 30 days shows its age in the trip group in Orange. Neither relies on colour alone.
- [ ] Every Places finder uses the grouped row with groups that fit its job (Outfitting, Shipyard, Services, Traders, Mining hotspots, Raw sites, Faction states, Cargo); none keeps a column per field. A text cell that is the same on every row is left out and stated once in the summary line (this takes in astromech-rfn8).
- [ ] Rows visible are measured at 1100 x 700 and 2560 x 1440 and DESIGN.md's figure updated. Browser tests on recorded markets cover the three demand states, the pad mark and the stale mark. Captures of Sell, Outfitting and Mining hotspots at both sizes. The Places guide page describes the row.

## Scope

The result row and the Places finders. The Engineers search results are astromech-7did.

## Context

Spike: branch `spike/nav-rework` at `dea5f0bb` (never merges): mockups in `app/src/mockups/nav/`, captures in `docs/design/nav-spike/`. Sumeet approved Variant B and the `tone5` versions on 17 Nov 2026 (mission notes). The demand thresholds and the list of coloured things are in the spike's final report and in DESIGN.md revision 5.
