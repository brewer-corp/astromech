---
created_at: "2026-10-15T20:36:13.649573438+00:00"
id: "astromech-i9w7"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-05bi"
  - kind: "issue"
    id: "astromech-kaa9"
  - kind: "issue"
    id: "astromech-xf8j"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:42:46.733288565+00:00"
status: "done"
title: "Colonisation shopping list"
updated_at: "2026-10-16T19:45:21.216184210+00:00"
---

## Goal

Remaining commodities minus cargo (and carrier cargo if known), in loads at the current ship's capacity ("6 more Type-9 runs").

## Done when

For a fixture project, cargo and ship, the remaining list and load count are right.

## Notes

- 2026-10-16 [note] Review fixes: Hold now folds MarketBuy/MarketSell, CollectCargo/EjectCargo (raw JSON; mission cargo skipped) and ColonisationContribution so delivered cargo is not subtracted twice; a Cargo event with a count but no Inventory keeps the previous hold (Count 0 clears it); Cargo items with MissionID are skipped, stolen goods are counted (depot acceptance unverified); table test of commodity_id on real names.

## Close Reason

Shopping list (remaining minus ship cargo, in loads at Loadout CargoCapacity) implemented and tested on a fixture project; carrier cargo not known from the journal.
